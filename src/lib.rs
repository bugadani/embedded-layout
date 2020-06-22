//! Enable simple layout operations in `embedded-graphics`
//!
//! This crate extends `embedded-graphics` objects that implement the `Transform` trait
//! to be aligned to other objects that have `Dimensions`.

#![cfg_attr(not(test), no_std)]

use embedded_graphics::{geometry::Point, prelude::*};

pub mod horizontal;
pub mod vertical;

/// Implement this trait for horizontal alignment algorithms
pub trait HorizontalAlignment {
    fn align(&self, what: &impl Dimensions, reference: &impl Dimensions) -> i32;
}

/// Implement this trait for vertical alignment algorithms
///
/// Vertical alignment assumes lower coordinate values are higher up
pub trait VerticalAlignment {
    fn align(&self, what: &impl Dimensions, reference: &impl Dimensions) -> i32;
}

/// This trait enables alignment operations of `embedded-graphics` primitives
pub trait Align: Transform {
    fn align_to<D, H, V>(self, reference: D, horizontal: H, vertical: V) -> Self
    where
        D: Dimensions,
        H: HorizontalAlignment,
        V: VerticalAlignment;
}

impl<T> Align for T
where
    T: Dimensions + Transform,
{
    fn align_to<D, H, V>(self, reference: D, horizontal: H, vertical: V) -> Self
    where
        D: Dimensions,
        H: HorizontalAlignment,
        V: VerticalAlignment,
    {
        let h = horizontal.align(&self, &reference);
        let v = vertical.align(&self, &reference);
        self.translate(Point::new(h, v))
    }
}
