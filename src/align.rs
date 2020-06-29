use crate::{prelude::*, HorizontalAlignment, VerticalAlignment};
use embedded_graphics::geometry::Point;

/// This trait enables alignment operations of `embedded-graphics` primitives
pub trait Align {
    fn align_to<H, V>(&mut self, reference: &impl View, horizontal: H, vertical: V) -> &mut Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment;
}

impl<T> Align for T
where
    T: View,
{
    fn align_to<H, V>(&mut self, reference: &impl View, horizontal: H, vertical: V) -> &mut Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment,
    {
        let h = horizontal.align(self, reference);
        let v = vertical.align(self, reference);
        self.translate(Point::new(h, v))
    }
}
