//! Element spacing
//!
//! `ElementSpacing` can be used to change the distance of objects along the layout orientation.
//! The default spacing is [`Tight`], which means objects are placed right next to each other,
//! without any space between them.
//!
//! Change the default spacing by calling [`LinearLayout::with_spacing`]
//!
//! [`LinearLayout::with_spacing`]: crate::layout::linear::LinearLayout::with_spacing

use crate::align::Alignment;
use embedded_graphics::primitives::Rectangle;

/// `ElementSpacing` base trait
pub trait ElementSpacing: Copy + Clone {
    /// Align `view` to `reference` using the element spacing rules
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

/// Lay out objects tightly, leaving no space between them
///
/// # Example:
/// ```rust
/// use embedded_layout::{
///     layout::linear::{spacing::Tight, LinearLayout},
///     prelude::*,
/// };
/// use embedded_graphics::{prelude::*, primitives::Line};
///
/// let _ = LinearLayout::horizontal(
///         Views::new(&mut [
///             Line::new(Point::zero(), Point::new(0, 5)),
///             Line::new(Point::zero(), Point::new(0, 5)),
///             Line::new(Point::zero(), Point::new(0, 5)),
///         ])
///     )
///     .with_spacing(Tight);
/// ```
#[derive(Copy, Clone)]
pub struct Tight;
impl ElementSpacing for Tight {
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
///
/// # Example:
/// ```
/// use embedded_layout::{
///     layout::linear::{spacing::FixedMargin, LinearLayout},
///     prelude::*,
/// };
/// use embedded_graphics::{prelude::*, primitives::Line};
///
/// // Apply a 3px margin between objects
/// let _ = LinearLayout::horizontal(
///         Views::new(&mut [
///             Line::new(Point::zero(), Point::new(0, 5)),
///             Line::new(Point::zero(), Point::new(0, 5)),
///             Line::new(Point::zero(), Point::new(0, 5)),
///         ])
///     )
///     .with_spacing(FixedMargin(3));
/// ```
#[derive(Copy, Clone)]
pub struct FixedMargin(pub i32);
impl ElementSpacing for FixedMargin {
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
/// Forces the layout to be as high or wide as set for this spacing
///
/// # Example:
/// ```rust
/// use embedded_layout::{
///     layout::linear::{spacing::DistributeFill, LinearLayout},
///     prelude::*,
/// };
/// use embedded_graphics::{prelude::*, primitives::Line};
///
/// // Distribute views in a 64px high space
/// let _ = LinearLayout::vertical(
///         Views::new(&mut [
///             Line::new(Point::zero(), Point::new(0, 5)),
///             Line::new(Point::zero(), Point::new(0, 5)),
///             Line::new(Point::zero(), Point::new(0, 5)),
///         ])
///     )
///     .with_spacing(DistributeFill(64));
/// ```
#[derive(Copy, Clone)]
pub struct DistributeFill(pub u32);
impl ElementSpacing for DistributeFill {
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
        // bit of a mess, but calculate using i32 in case the views don't fit the space
        let empty_space = self.0 as i32 - total_size as i32;
        let base = empty_space / (objects - 1) as i32;
        let remainder = empty_space % (objects - 1) as i32;

        let offset = if n == 0 {
            0
        } else if n as i32 <= remainder {
            base + 1
        } else {
            base
        };
        alignment.align_with_offset(view, reference, offset)
    }
}
