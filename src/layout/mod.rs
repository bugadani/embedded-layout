//! Layout module
//!
//! This module implements layouts that can be used to work with multiple `View`s easily.
//! Layouts are either `View` objects, or can be used to return `View` objects.
//!
//! The base of all layouts is the `ViewGroup` which binds multiple `View`s together.

use crate::prelude::*;
use embedded_graphics::primitives::Rectangle;

pub mod linear;

pub trait ViewChainElement: View {
    const IS_TERMINATOR: bool;

    fn view_count() -> usize;
    fn for_each(&mut self, op: &mut impl FnMut(&mut dyn View));
}

/// Chain element that can store a `View` in a `ViewGroup`
///
/// You probably shouldn't ever use this struct
pub struct ViewLink<V: View, C: ViewChainElement> {
    pub(crate) view: V,
    pub(crate) next: C,
}

impl<C, V, VC> Drawable<C> for ViewLink<V, VC>
where
    C: PixelColor,
    V: View,
    for<'a> &'a V: Drawable<C>,
    VC: ViewChainElement + Drawable<C>,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        self.view.draw(display)?;
        self.next.draw(display)?;

        Ok(())
    }
}

impl<V: View, VC: ViewChainElement> ViewChainElement for ViewLink<V, VC> {
    const IS_TERMINATOR: bool = false;

    fn view_count() -> usize {
        1 + VC::view_count()
    }

    fn for_each(&mut self, op: &mut impl FnMut(&mut dyn View)) {
        // Keep order of elements
        self.next.for_each(op);
        op(&mut self.view);
    }
}

impl<V: View, C: ViewChainElement> View for ViewLink<V, C> {
    fn bounds(&self) -> Rectangle {
        let bounds = self.view.bounds();

        if !C::IS_TERMINATOR {
            bounds.enveloping(&self.next.bounds())
        } else {
            bounds
        }
    }

    fn translate(&mut self, by: Point) {
        self.view.translate(by);
        self.next.translate(by);
    }
}

/// The last chain element that marks the end of a `ViewGroup`
///
/// You probably shouldn't ever use this struct
pub struct ChainTerminator;

impl ViewChainElement for ChainTerminator {
    const IS_TERMINATOR: bool = true;

    fn view_count() -> usize {
        0
    }

    fn for_each(&mut self, _op: &mut impl FnMut(&mut dyn View)) {
        // nothing to do
    }
}

impl<C: PixelColor> Drawable<C> for ChainTerminator {
    fn draw<D: DrawTarget<C>>(self, _display: &mut D) -> Result<(), D::Error> {
        Ok(())
    }
}

impl View for ChainTerminator {
    fn bounds(&self) -> Rectangle {
        Rectangle::new(Point::zero(), Point::zero())
    }

    fn translate(&mut self, _by: Point) {
        // nothing to do
    }
}

/// Group multiple `View`s together
///
/// `ViewGroup` takes ownership over the views, so make sure you set them up before creating
/// the group.
/// The bounds and size of a `ViewGroup` envelops all the contained `View`s.
///
/// Note: translating an empty `ViewGroup` has no effect
pub struct ViewGroup<C: ViewChainElement> {
    pub(crate) views: C,
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
    /// Bind a `View` to this `ViewGroup`
    ///
    /// The `View` remains at it's current location, until the `ViewGroup` is translated.
    fn add_view<V: View>(self, view: V) -> ViewGroup<ViewLink<V, C>> {
        ViewGroup {
            views: ViewLink {
                view,
                next: self.views,
            },
        }
    }

    /// Run the callback on each included `View` objects
    fn for_each(&mut self, op: &mut impl FnMut(&mut dyn View)) {
        self.views.for_each(op);
    }

    /// Returns the number of views in this `ViewGroup`
    fn view_count(&self) -> usize {
        C::view_count()
    }
}

impl<C: ViewChainElement> View for ViewGroup<C> {
    fn translate(&mut self, by: Point) {
        self.views.translate(by);
    }

    fn bounds(&self) -> Rectangle {
        self.views.bounds()
    }
}

impl<C: PixelColor, VC: ViewChainElement + Drawable<C>> Drawable<C> for ViewGroup<VC> {
    /// Draw the graphics object using the supplied DrawTarget.
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        self.views.draw(display)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        layout::{ViewChainElement, ViewGroup},
        prelude::*,
    };
    use embedded_graphics::mock_display::MockDisplay;
    use embedded_graphics::{
        pixelcolor::BinaryColor,
        primitives::{Circle, Rectangle},
        style::PrimitiveStyle,
    };

    #[test]
    fn sanity_check() {
        // Check if multiple different views can be included in the view group
        let vg = ViewGroup::new()
            .add_view(Rectangle::with_size(Point::zero(), Size::new(5, 10)))
            .add_view(Circle::new(Point::zero(), 5));

        fn check_vg<C: ViewChainElement>(vg: &ViewGroup<C>) {
            assert_eq!(2, vg.view_count());
        }

        check_vg(&vg);
    }

    #[test]
    fn test() {
        // Check if multiple different views can be included in the view group
        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        let mut vg = ViewGroup::new()
            .add_view(Rectangle::with_size(Point::zero(), Size::new(5, 10)).into_styled(style))
            .add_view(Rectangle::with_size(Point::new(3, 5), Size::new(5, 10)).into_styled(style))
            .add_view(
                Rectangle::with_size(Point::new(-2, -5), Size::new(5, 10)).into_styled(style),
            );

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

        // This tests that the view group implements Drawable as expected
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        vg.draw(&mut disp).unwrap();
    }

    #[test]
    fn test_align() {
        // This tests that the view group implements Drawable as expected
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        // Check if multiple different views can be included in the view group
        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        let rect3 = Rectangle::with_size(Point::new(-2, -5), Size::new(5, 10)).into_styled(style);
        ViewGroup::new()
            .add_view(Rectangle::with_size(Point::zero(), Size::new(5, 10)).into_styled(style))
            .add_view(Rectangle::with_size(Point::new(3, 5), Size::new(5, 10)).into_styled(style))
            .align_to(&rect3, horizontal::LeftToRight, vertical::TopToBottom)
            .draw(&mut disp)
            .unwrap();
    }
}
