use crate::{
    layout::{
        linear::{orientation::Orientation, secondary_alignment::SecondaryAlignment},
        Guard, Link, ViewChainElement,
    },
    prelude::*,
};
use embedded_graphics::primitives::Rectangle;

pub trait LayoutElement<LD: Orientation>: ViewChainElement {
    fn measure(&self) -> Size;
    fn arrange(&mut self, bounds: Rectangle, orientation: &LD, count: usize) -> Rectangle;
}

impl<V, VCE, LD> LayoutElement<LD> for Link<V, VCE>
where
    V: View + Align,
    VCE: LayoutElement<LD>,
    LD: Orientation,
{
    fn measure(&self) -> Size {
        let current_el_size = self.object.size();
        if VCE::IS_TERMINATOR {
            current_el_size
        } else {
            let prev_size = self.next.measure();
            LD::Secondary::measure(prev_size, current_el_size)
        }
    }

    fn arrange(&mut self, bounds: Rectangle, orientation: &LD, count: usize) -> Rectangle {
        if VCE::IS_TERMINATOR {
            orientation.place_first(&mut self.object, bounds, count);
        } else {
            let previous = self.next.arrange(bounds, orientation, count);
            orientation.place_nth(
                &mut self.object,
                RectExt::size(&bounds),
                previous,
                VCE::count(),
                count,
            );
        }
        self.object.bounds()
    }
}

impl<LD: Orientation> LayoutElement<LD> for Guard {
    fn measure(&self) -> Size {
        Size::new(0, 0)
    }

    fn arrange(&mut self, _bounds: Rectangle, _orientation: &LD, _count: usize) -> Rectangle {
        // Nothing to do
        Rectangle::new(Point::zero(), Point::zero())
    }
}
