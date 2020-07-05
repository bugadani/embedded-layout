use crate::{
    align::Alignment,
    layout::{Guard, Link, ViewChainElement},
    prelude::*,
};
use embedded_graphics::primitives::Rectangle;

use super::{orientation::Orientation, secondary_alignment::SecondaryAlignment};

pub trait LayoutOperation<LD: Orientation> {
    fn measure(&self) -> Size;
    fn arrange(&mut self, bounds: Rectangle) -> Rectangle;
}

impl<V, VCE, LD> LayoutOperation<LD> for Link<V, VCE>
where
    V: View + Align,
    VCE: ViewChainElement + LayoutOperation<LD>,
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

    fn arrange(&mut self, bounds: Rectangle) -> Rectangle {
        if VCE::IS_TERMINATOR {
            self.object.align_to_mut(
                &bounds,
                LD::FirstHorizontalAlignment::new(),
                LD::FirstVerticalAlignment::new(),
            );
        } else {
            let previous = self.next.arrange(bounds);

            self.object.align_to_mut(
                &previous,
                LD::HorizontalAlignment::new(),
                LD::VerticalAlignment::new(),
            );
        }
        self.object.bounds()
    }
}

impl<LD: Orientation> LayoutOperation<LD> for Guard {
    fn measure(&self) -> Size {
        Size::new(0, 0)
    }

    fn arrange(&mut self, _bounds: Rectangle) -> Rectangle {
        // Nothing to do
        Rectangle::new(Point::zero(), Point::zero())
    }
}
