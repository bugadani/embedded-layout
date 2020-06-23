//! Enable simple layout operations in `embedded-graphics`
//!
//! This crate extends `embedded-graphics` objects that implement the `Transform` trait
//! to be aligned to other objects that have `Dimensions`.
//!
//! # Example
//!
//! Draw some text to the center of the display:
//!
//! ```rust
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();
//! #
//! use embedded_layout::prelude::*;
//! use embedded_graphics::{
//!     prelude::*,
//!     fonts::{Font6x8, Text},
//!     geometry::Point,
//!     primitives::Rectangle,
//!     pixelcolor::BinaryColor,
//!     style::TextStyleBuilder,
//! };
//!
//! let display_area = disp.display_area();
//!
//! let text_style = TextStyleBuilder::new(Font6x8)
//!                         .text_color(BinaryColor::On)
//!                         .build();
//!
//! Text::new("Hello, world!", Point::zero())
//!      .into_styled(text_style)
//!      .align_to(display_area, horizontal::Center, vertical::Center)
//!      .draw(&mut disp)
//!      .unwrap();
//! ```

#![cfg_attr(not(test), no_std)]

use embedded_graphics::{geometry::Point, prelude::*, primitives::Rectangle, DrawTarget};

pub mod horizontal;
pub mod vertical;

/// The essentials
pub mod prelude {
    pub use crate::{horizontal, vertical, Align, DisplayArea};
}

/// Helper trait to retrieve display area as a `Rectangle`.
pub trait DisplayArea<C>: DrawTarget<C>
where
    C: PixelColor,
{
    fn display_area(&self) -> Rectangle;
}

impl<C, T> DisplayArea<C> for T
where
    C: PixelColor,
    T: DrawTarget<C>,
{
    fn display_area(&self) -> Rectangle {
        Rectangle::new(
            Point::new(0, 0),
            Point::new(
                (self.size().width - 1) as i32,
                (self.size().height - 1) as i32,
            ),
        )
    }
}

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
