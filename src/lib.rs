//! Enable simple layout operations in [`embedded-graphics`]
//!
//! This crate extends `embedded-graphics` objects that implement the `Transform` trait
//! to be aligned to other objects that have `Dimensions`.
//!
//! ## Examples
//!
//! The examples are based on [the `embedded-graphics` simulator]. The simulator is built on top of
//! `SDL2`. If you don't have that installed, set the `EG_SIMULATOR_DUMP="screenshot.png"`
//! environment variable so that running the examples produce a screenshot image instead of a window.
//!
//! ### Draw some text to the center of the display
//!
//! ```no_run
//! use embedded_graphics_simulator::{
//!     BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
//! };
//!
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     style::TextStyleBuilder,
//! };
//! use embedded_layout::prelude::*;
//!
//! fn main() -> Result<(), core::convert::Infallible> {
//!     let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(129, 129));
//!
//!     // Create a Rectangle from the display's dimensions
//!     let display_area = display.display_area();
//!
//!     let text_style = TextStyleBuilder::new(Font6x8)
//!         .text_color(BinaryColor::On)
//!         .build();
//!
//!     Text::new("Hello, World!", Point::zero())
//!         .into_styled(text_style)
//!         // align text to the display
//!         .align_to(&display_area, horizontal::Center, vertical::Center)
//!         .draw(&mut display)
//!         .unwrap();
//!
//!     let output_settings = OutputSettingsBuilder::new()
//!         .theme(BinaryColorTheme::OledBlue)
//!         .build();
//!     Window::new("Hello World", &output_settings).show_static(&display);
//!     Ok(())
//! }
//! ```
//!
//! ### Use `LinearLayout` to arrange multiple objects
//!
//! ```no_run
//! use embedded_graphics_simulator::{
//!     BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
//! };
//!
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     geometry::Point,
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     style::TextStyleBuilder,
//! };
//! use embedded_layout::layout::linear::LinearLayout;
//! use embedded_layout::prelude::*;
//!
//! fn main() -> Result<(), core::convert::Infallible> {
//!     let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(64, 48));
//!     let output_settings = OutputSettingsBuilder::new()
//!         .theme(BinaryColorTheme::OledBlue)
//!         .build();
//!
//!     let display_area = display.display_area();
//!
//!     let text_style = TextStyleBuilder::new(Font6x8)
//!         .text_color(BinaryColor::On)
//!         .build();
//!
//!     LinearLayout::vertical()
//!         .with_alignment(horizontal::Center)
//!         .add_view(Text::new("Vertical", Point::zero()).into_styled(text_style))
//!         .add_view(Text::new("Linear", Point::zero()).into_styled(text_style))
//!         .add_view(Text::new("Layout", Point::zero()).into_styled(text_style))
//!         .arrange()
//!         .align_to(&display_area, horizontal::Center, vertical::Center)
//!         .draw(&mut display)
//!         .unwrap();
//!
//!     Window::new("LinearLayout exmaple", &output_settings).show_static(&display);
//!     Ok(())
//! }
//! ```
//!
//! [`embedded-graphics`]: https://github.com/jamwaffles/embedded-graphics/
//! [the `embedded-graphics` simulator]: https://github.com/jamwaffles/embedded-graphics/tree/master/simulator

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]
#![deny(clippy::missing_inline_in_public_items)]

use embedded_graphics::{geometry::Point, prelude::*, primitives::Rectangle};

mod align;
pub mod layout;
mod utils;

use utils::rect_helper::RectExt;

/// The essentials
pub mod prelude {
    pub use crate::{
        align::{horizontal, vertical, Align},
        utils::{display_area::DisplayArea, rect_helper::RectExt},
        View,
    };

    pub use embedded_graphics::prelude::*;
}

/// A view is the base unit for most of the `embedded-layout` operations.
///
/// Views must have a size and a position, so they need to implement the `Dimensions` and
/// `Transform` traits.
pub trait View {
    /// Get the size of a View.
    #[inline]
    fn size(&self) -> Size {
        RectExt::size(&self.bounds())
    }

    /// Move the origin of an object by a given number of (x, y) pixels
    fn translate(&mut self, by: Point);

    /// Returns the bounding box of the `View` as a `Rectangle`
    fn bounds(&self) -> Rectangle;
}

impl<T> View for T
where
    T: Transform + Dimensions,
{
    #[inline]
    fn translate(&mut self, by: Point) {
        self.translate_mut(by);
    }

    #[inline]
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
