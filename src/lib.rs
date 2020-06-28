//! Enable simple layout operations in [`embedded-graphics`]
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
//!      .align_to(&display_area, horizontal::Center, vertical::Center)
//!      .draw(&mut disp)
//!      .unwrap();
//! ```
//!
//! [`embedded-graphics`]: https://github.com/jamwaffles/embedded-graphics/

#![cfg_attr(not(test), no_std)]

use embedded_graphics::{geometry::Point, prelude::*, primitives::Rectangle};

pub mod horizontal;
pub mod vertical;

mod view;
mod align;

pub use view::View;
pub use align::{Align, AlignMut};

/// The essentials
pub mod prelude {
    pub use crate::{horizontal, vertical, Align, AlignMut, DisplayArea, View};
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
pub trait HorizontalAlignment: Copy + Clone {
    fn align(&self, what: &impl View, reference: &impl View) -> i32;
}

/// Implement this trait for vertical alignment algorithms
///
/// Vertical alignment assumes lower coordinate values are higher up
pub trait VerticalAlignment: Copy + Clone {
    fn align(&self, what: &impl View, reference: &impl View) -> i32;
}
