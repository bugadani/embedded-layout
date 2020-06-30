use crate::prelude::*;
use embedded_graphics::geometry::Point;

pub mod horizontal;
pub mod vertical;

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
        self.translate(Point::new(h, v));
        self
    }
}

/// Implement this trait for horizontal alignment algorithms
pub trait HorizontalAlignment: Copy + Clone {
    fn align(&self, what: &impl View, reference: &impl View) -> i32;
}

/// Implement this trait for vertical alignment algorithms
///
/// Vertical alignment assumes lower coordinate values are higher up
pub trait VerticalAlignment: Copy + Clone {
    fn align(&self, what: &impl View, reference: &impl View) -> i32;
}
