use crate::prelude::*;
use embedded_graphics::primitives::Rectangle;

pub mod horizontal;
pub mod vertical;

/// This trait enables alignment operations of `embedded-graphics` primitives
pub trait Align {
    /// Align a copy of the object to an other one using the alignment parameters as rules
    fn align_to<H, V>(self, reference: &impl View, horizontal: H, vertical: V) -> Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment;

    /// Align the object to an other one using the alignment parameters as rules
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
    #[inline]
    fn align_to<H, V>(mut self, reference: &impl View, horizontal: H, vertical: V) -> Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment,
    {
        let self_bounds = self.bounds();
        let reference_bounds = reference.bounds();

        let h = horizontal.align(self_bounds, reference_bounds);
        let v = vertical.align(self_bounds, reference_bounds);
        self.translate(Point::new(h, v));
        self
    }

    #[inline]
    fn align_to_mut<H, V>(&mut self, reference: &impl View, horizontal: H, vertical: V) -> &mut Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment,
    {
        let self_bounds = self.bounds();
        let reference_bounds = reference.bounds();

        let h = horizontal.align(self_bounds, reference_bounds);
        let v = vertical.align(self_bounds, reference_bounds);
        self.translate(Point::new(h, v));
        self
    }
}

/// Common trait for alignment operations
pub trait Alignment: Copy + Clone + Default {
    /// Align one coordinate of `View` to the given reference
    fn align(&self, what: Rectangle, reference: Rectangle) -> i32;
}

/// Implement this trait for horizontal alignment algorithms
pub trait HorizontalAlignment: Alignment {}

/// Implement this trait for vertical alignment algorithms
///
/// Vertical alignment assumes lower coordinate values are higher up
pub trait VerticalAlignment: Alignment {}
