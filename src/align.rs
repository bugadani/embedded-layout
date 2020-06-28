
use embedded_graphics::geometry::Point;
use crate::{HorizontalAlignment, VerticalAlignment, prelude::*};

/// This trait enables alignment operations of `embedded-graphics` primitives
pub trait Align {
    fn align_to<H, V>(self, reference: &impl View, horizontal: H, vertical: V) -> Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment;
}

pub trait AlignMut {
    fn align_to_mut<H, V>(
        &mut self,
        reference: &impl View,
        horizontal: H,
        vertical: V,
    ) -> &mut Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment;
}

impl<T> Align for T
where
    T: View,
{
    fn align_to<H, V>(self, reference: &impl View, horizontal: H, vertical: V) -> Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment,
    {
        let h = horizontal.align(&self, reference);
        let v = vertical.align(&self, reference);
        self.translate(Point::new(h, v))
    }
}

impl<T> AlignMut for T
where
    T: View,
{
    fn align_to_mut<H, V>(&mut self, reference: &impl View, horizontal: H, vertical: V) -> &mut Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment,
    {
        let h = horizontal.align(self, reference);
        let v = vertical.align(self, reference);
        self.translate_mut(Point::new(h, v))
    }
}
