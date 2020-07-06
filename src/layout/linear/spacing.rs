//! Element spacing
//!
//! `ElementSpacing` can be used to change the distance of objects along the layout orientation.
//! The default spacing is `Tight`, which means objects are placed right next to each other,
//! without any space between them.

use crate::align::Alignment;
use embedded_graphics::primitives::Rectangle;

/// `ElementSpacing` base trait
pub trait ElementSpacing: Copy + Clone {
    /// Calculate how much the total size of a layout changes by applying the current spacing
    fn modify_measurement(&self, measured_size: u32, objects: usize) -> u32;

    /// Calculate the alignment for the nth object
    fn align(
        &self,
        alignment: impl Alignment,
        view: Rectangle,
        reference: Rectangle,
        n: usize,
        objects: usize,
        total_size: u32,
    ) -> i32;
}

/// Lay out objects tightly
#[derive(Copy, Clone)]
pub struct Tight;
impl ElementSpacing for Tight {
    #[inline]
    fn modify_measurement(&self, measured_size: u32, _objects: usize) -> u32 {
        measured_size
    }

    #[inline]
    fn align(
        &self,
        alignment: impl Alignment,
        view: Rectangle,
        reference: Rectangle,
        _n: usize,
        _objects: usize,
        _total_size: u32,
    ) -> i32 {
        alignment.align_with_offset(view, reference, 0)
    }
}

/// Lay out objects with fixed margin between them
///
/// The margin can be negative, in which case the elements will be placed over one another.
#[derive(Copy, Clone)]
pub struct FixedMargin(pub i32);
impl ElementSpacing for FixedMargin {
    #[inline]
    fn modify_measurement(&self, measured_size: u32, objects: usize) -> u32 {
        if objects == 0 {
            measured_size
        } else {
            (measured_size as i32 + self.0 * (objects - 1) as i32) as u32
        }
    }

    #[inline]
    fn align(
        &self,
        alignment: impl Alignment,
        view: Rectangle,
        reference: Rectangle,
        n: usize,
        _objects: usize,
        _total_size: u32,
    ) -> i32 {
        let offset = if n == 0 { 0 } else { self.0 };
        alignment.align_with_offset(view, reference, offset)
    }
}

/// Distribute views to fill a given space
///
/// Forces layout to be as high or wide as set for this spacing
#[derive(Copy, Clone)]
pub struct DistributeFill(pub u32);
impl ElementSpacing for DistributeFill {
    #[inline]
    fn modify_measurement(&self, _measured_size: u32, _objects: usize) -> u32 {
        self.0
    }

    #[inline]
    fn align(
        &self,
        alignment: impl Alignment,
        view: Rectangle,
        reference: Rectangle,
        n: usize,
        objects: usize,
        total_size: u32,
    ) -> i32 {
        let empty_space = self.0 - total_size;
        let base = empty_space as i32 / (objects as i32 - 1);
        let remainder = empty_space as usize % (objects - 1);

        let offset = if n == 0 {
            0
        } else if n <= remainder {
            base + 1
        } else {
            base
        };
        alignment.align_with_offset(view, reference, offset)
    }
}
