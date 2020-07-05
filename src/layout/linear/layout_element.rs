use crate::{
    layout::{
        linear::{
            orientation::Orientation, secondary_alignment::SecondaryAlignment,
            spacing::ElementSpacing,
        },
        Guard, Link, ViewChainElement,
    },
    prelude::*,
};
use embedded_graphics::primitives::Rectangle;

pub trait LayoutElement<LD: Orientation>: ViewChainElement {
    fn measure(&self) -> Size;
    fn arrange(
        &mut self,
        bounds: Rectangle,
        spacing: &impl ElementSpacing,
        count: usize,
    ) -> Rectangle;
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

    fn arrange(
        &mut self,
        bounds: Rectangle,
        spacing: &impl ElementSpacing,
        count: usize,
    ) -> Rectangle {
        if VCE::IS_TERMINATOR {
            self.object.align_to_mut(
                &bounds,
                LD::FirstHorizontalAlignment::default(),
                LD::FirstVerticalAlignment::default(),
            );
        } else {
            let previous = self.next.arrange(bounds, spacing, count);

            self.object.align_to_mut(
                &previous,
                LD::HorizontalAlignment::default(),
                LD::VerticalAlignment::default(),
            );
        }
        LD::adjust_placement(
            &mut self.object,
            spacing,
            VCE::count(),
            RectExt::size(&bounds),
            count,
        );
        self.object.bounds()
    }
}

impl<LD: Orientation> LayoutElement<LD> for Guard {
    fn measure(&self) -> Size {
        Size::new(0, 0)
    }

    fn arrange(
        &mut self,
        _bounds: Rectangle,
        _spacing: &impl ElementSpacing,
        _count: usize,
    ) -> Rectangle {
        // Nothing to do
        Rectangle::new(Point::zero(), Point::zero())
    }
}
