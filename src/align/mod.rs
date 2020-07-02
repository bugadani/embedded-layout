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

/// Common trait for alignment operations
pub trait Alignment: Copy + Clone {
    /// Create a new alignment object
    fn new() -> Self;
    /// Align one coordinate of `View` to the given reference
    fn align(&self, what: &impl View, reference: &impl View) -> i32;
}

/// Implement this trait for horizontal alignment algorithms
pub trait HorizontalAlignment: Alignment {}

/// Implement this trait for vertical alignment algorithms
///
/// Vertical alignment assumes lower coordinate values are higher up
pub trait VerticalAlignment: Alignment {}
