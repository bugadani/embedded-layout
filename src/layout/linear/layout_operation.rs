use crate::{
    align::{HorizontalAlignment, VerticalAlignment},
    layout::{ChainTerminator, ViewChainElement, ViewLink},
    prelude::*,
};
use embedded_graphics::primitives::Rectangle;

use super::{
    layout_direction::{Horizontal, LayoutDirection, Vertical},
    secondary_alignment::SecondaryAlignment,
};

pub trait LayoutOperation<LD: LayoutDirection> {
    fn measure(&self) -> Size;
    fn arrange(&mut self, bounds: Rectangle) -> Rectangle;
}

impl<V, VCE, Secondary> LayoutOperation<Horizontal<Secondary>> for ViewLink<V, VCE>
where
    V: View + Align,
    VCE: ViewChainElement + LayoutOperation<Horizontal<Secondary>>,
    Secondary: SecondaryAlignment + VerticalAlignment,
{
    fn measure(&self) -> Size {
        let current_el_size = self.view.size();
        if VCE::IS_TERMINATOR {
            current_el_size
        } else {
            let prev_size = self.next.measure();
            Secondary::measure(prev_size, current_el_size)
        }
    }

    fn arrange(&mut self, bounds: Rectangle) -> Rectangle {
        if VCE::IS_TERMINATOR {
            self.view
                .align_to_mut(&bounds, horizontal::Left, Secondary::new());
        } else {
            let previous = self.next.arrange(bounds);

            self.view
                .align_to_mut(&previous, horizontal::LeftToRight, Secondary::new());
        }
        self.view.bounds()
    }
}

impl<Secondary> LayoutOperation<Horizontal<Secondary>> for ChainTerminator
where
    Secondary: SecondaryAlignment + VerticalAlignment,
{
    fn measure(&self) -> Size {
        Size::new(0, 0)
    }

    fn arrange(&mut self, _bounds: Rectangle) -> Rectangle {
        // Nothing to do
        Rectangle::new(Point::zero(), Point::zero())
    }
}

impl<V, VCE, Secondary> LayoutOperation<Vertical<Secondary>> for ViewLink<V, VCE>
where
    V: View + Align,
    VCE: ViewChainElement + LayoutOperation<Vertical<Secondary>>,
    Secondary: SecondaryAlignment + HorizontalAlignment,
{
    fn measure(&self) -> Size {
        let current_el_size = self.view.size();
        if VCE::IS_TERMINATOR {
            current_el_size
        } else {
            let prev_size = self.next.measure();
            Secondary::measure(prev_size, current_el_size)
        }
    }

    fn arrange(&mut self, bounds: Rectangle) -> Rectangle {
        if VCE::IS_TERMINATOR {
            self.view
                .align_to_mut(&bounds, Secondary::new(), vertical::Top);
        } else {
            let previous = self.next.arrange(bounds);

            self.view
                .align_to_mut(&previous, Secondary::new(), vertical::TopToBottom);
        }
        self.view.bounds()
    }
}

impl<Secondary> LayoutOperation<Vertical<Secondary>> for ChainTerminator
where
    Secondary: SecondaryAlignment + HorizontalAlignment,
{
    fn measure(&self) -> Size {
        Size::new(0, 0)
    }

    fn arrange(&mut self, _bounds: Rectangle) -> Rectangle {
        // Nothing to do
        Rectangle::new(Point::zero(), Point::zero())
    }
}
