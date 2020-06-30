use crate::prelude::*;
use embedded_graphics::{geometry::Point, primitives::Rectangle};

pub trait ViewChainElement: View {
    const HAS_BOUNDS: bool;
}

/// Chain element that can store a `View` in a `ViewGroup`
///
/// You probably shouldn't ever use this struct
pub struct ViewLink<V: View, C: ViewChainElement> {
    pub view: V,
    pub next: C,
}

impl<V: View, C: ViewChainElement> ViewChainElement for ViewLink<V, C> {
    const HAS_BOUNDS: bool = true;
}

impl<V: View, C: ViewChainElement> View for ViewLink<V, C> {
    fn bounds(&self) -> Rectangle {
        let bounds = self.view.bounds();

        if C::HAS_BOUNDS {
            bounds.enveloping(&self.next.bounds())
        } else {
            bounds
        }
    }

    fn translate(&mut self, by: Point) -> &mut Self {
        self.view.translate(by);
        self.next.translate(by);
        self
    }
}

/// The last chain element that marks the end of a `ViewGroup`
///
/// You probably shouldn't ever use this struct
pub struct ChainTerminator;

impl ViewChainElement for ChainTerminator {
    const HAS_BOUNDS: bool = false;
}

impl View for ChainTerminator {
    fn bounds(&self) -> Rectangle {
        Rectangle::new(Point::zero(), Point::zero())
    }

    fn translate(&mut self, _by: Point) -> &mut Self {
        // nothing to do
        self
    }
}

/// Group multiple `View`s together
///
/// Note: translating an empty `ViewGroup` has no effect
pub struct ViewGroup<C: ViewChainElement> {
    views: C,
}

impl ViewGroup<ChainTerminator> {
    /// Create a new, empty `ViewGroup` object
    pub fn new() -> Self {
        Self {
            views: ChainTerminator,
        }
    }
}

impl<C: ViewChainElement> ViewGroup<C> {
    fn add_view<V: View>(self, view: V) -> ViewGroup<ViewLink<V, C>> {
        ViewGroup {
            views: ViewLink {
                view,
                next: self.views,
            },
        }
    }
}

impl<C: ViewChainElement> View for ViewGroup<C> {
    fn translate(&mut self, by: Point) -> &mut Self {
        self.views.translate(by);
        self
    }

    fn bounds(&self) -> Rectangle {
        self.views.bounds()
    }
}

#[cfg(test)]
mod test {
    use crate::layout::*;
    use embedded_graphics::{
        geometry::{Point, Size},
        primitives::{Circle, Rectangle},
    };

    #[test]
    fn sanity_check() {
        // Check if multiple different views can be included in the view group
        let vg = ViewGroup::new()
            .add_view(Rectangle::with_size(Point::zero(), Size::new(5, 10)))
            .add_view(Circle::new(Point::zero(), 5));

        fn check_vg<C: ViewChainElement>(_vg: &ViewGroup<C>) {}

        check_vg(&vg);
    }

    #[test]
    fn test() {
        // Check if multiple different views can be included in the view group
        let mut vg = ViewGroup::new()
            .add_view(Rectangle::with_size(Point::zero(), Size::new(5, 10)))
            .add_view(Rectangle::with_size(Point::new(3, 5), Size::new(5, 10)))
            .add_view(Rectangle::with_size(Point::new(-2, -5), Size::new(5, 10)));

        assert_eq!(Size::new(10, 20), vg.size());
        assert_eq!(
            Rectangle::new(Point::new(-2, -5), Point::new(7, 14)),
            vg.bounds()
        );

        vg.translate(Point::new(2, 3));

        assert_eq!(Size::new(10, 20), vg.size());
        assert_eq!(
            Rectangle::new(Point::new(0, -2), Point::new(9, 17)),
            vg.bounds()
        );
    }
}
