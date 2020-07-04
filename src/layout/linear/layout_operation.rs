use crate::{
    align::Alignment,
    layout::{ChainTerminator, ViewChainElement, ViewLink},
    prelude::*,
};
use embedded_graphics::primitives::Rectangle;

use super::{layout_direction::LayoutDirection, secondary_alignment::SecondaryAlignment};

pub trait LayoutOperation<LD: LayoutDirection> {
    fn measure(&self) -> Size;
    fn arrange(&mut self, bounds: Rectangle) -> Rectangle;
}

impl<V, VCE, LD> LayoutOperation<LD> for ViewLink<V, VCE>
where
    V: View + Align,
    VCE: ViewChainElement + LayoutOperation<LD>,
    LD: LayoutDirection,
{
    fn measure(&self) -> Size {
        let current_el_size = self.view.size();
        if VCE::IS_TERMINATOR {
            current_el_size
        } else {
            let prev_size = self.next.measure();
            LD::Secondary::measure(prev_size, current_el_size)
        }
    }

    fn arrange(&mut self, bounds: Rectangle) -> Rectangle {
        if VCE::IS_TERMINATOR {
            self.view.align_to_mut(
                &bounds,
                LD::FirstHorizontalAlignment::new(),
                LD::FirstVerticalAlignment::new(),
            );
        } else {
            let previous = self.next.arrange(bounds);

            self.view.align_to_mut(
                &previous,
                LD::HorizontalAlignment::new(),
                LD::VerticalAlignment::new(),
            );
        }
        self.view.bounds()
    }
}

impl<LD: LayoutDirection> LayoutOperation<LD> for ChainTerminator {
    fn measure(&self) -> Size {
        Size::new(0, 0)
    }

    fn arrange(&mut self, _bounds: Rectangle) -> Rectangle {
        // Nothing to do
        Rectangle::new(Point::zero(), Point::zero())
    }
}
