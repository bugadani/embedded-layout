use crate::prelude::*;

pub struct ViewLink<V: View, C: ViewChainElement> {
    pub view: V,
    pub next: C,
}

pub struct ChainTerminator;
pub trait ViewChainElement {}

impl<V: View, C: ViewChainElement> ViewChainElement for ViewLink<V, C> {}
impl ViewChainElement for ChainTerminator {}

pub struct ViewGroup<C: ViewChainElement> {
    pub views: C,
}

impl ViewGroup<ChainTerminator> {
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
        let _ = ViewGroup::new()
            .add_view(Rectangle::with_size(Point::zero(), Size::new(5, 10)))
            .add_view(Circle::new(Point::zero(), 5));
    }
}
