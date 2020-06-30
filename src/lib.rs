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

mod align;
mod utils;

use utils::rect_helper::RectExt;

/// The essentials
pub mod prelude {
    pub use crate::{
        align::{horizontal, vertical, Align},
        utils::{display_area::DisplayArea, rect_helper::RectExt},
        View,
    };
}

/// A view is the base unit for most of the `embedded-layout` operations.
///
/// Views must have a size and a position, so they need to implement the `Dimensions` and
/// `Transform` traits.
pub trait View {
    /// Get the size of a View.
    fn size(&self) -> Size;
    fn translate(&mut self, by: Point) -> &mut Self;
    fn bounds(&self) -> Rectangle;
}

impl<T> View for T
where
    T: Transform + Dimensions,
{
    fn size(&self) -> Size {
        let bounds = self.bounds();
        RectExt::size(&bounds)
    }

    fn translate(&mut self, by: Point) -> &mut Self {
        self.translate_mut(by)
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(self.top_left(), self.bottom_right())
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use embedded_graphics::{
        geometry::{Point, Size},
        primitives::Rectangle,
    };

    #[test]
    fn test_size() {
        let rect = Rectangle::new(Point::zero(), Point::new(1, 2));

        assert_eq!(RectExt::size(&rect), Size::new(2, 3));
    }
}
