//! Enable simple layout operations in [`embedded-graphics`]
//!
//! This crate extends [`embedded-graphics`]' objects that implement the `Transform` trait
//! to be aligned to other objects that implement `Dimensions`.
//!
//! # A note on imports
//!
//! `embedded-layout` reexports most of `embedded-graphics`' `prelude` module. In most cases
//! this means you don't have to `use embedded_graphics::prelude::*`. In case you need to,
//! `Translate` and `Dimensions` may interfere with `embedded-layout`'s [`View`], so if you are using
//! functions of those traits, you may need to use the [fully qualified syntax] (formerly UFCS):
//!
//! ```compile_fail
//! use embedded_layout::prelude::*;
//! use embedded_graphics::prelude::*; //< this imports `Dimensions` which has a `size` function
//! use embedded_graphics::primitives::Rectangle;
//!
//! let rect = Rectangle::with_size(Point::zero(), Size::new(10, 10));
//! let size = rect.size(); //< this fails to compile
//! ```
//!
//! The above example fails to compile with this message:
//!
//! ```text
//! ---- src\lib.rs - (line 13) stdout ----
//! error[E0034]: multiple applicable items in scope
//! --> src\lib.rs:19:17
//!     |
//! 9   | let size = rect.size(); //< this fails to compile
//!     |                 ^^^^ multiple `size` found
//!     | [some other lines about where the candidates are]
//! ```
//!
//! Here's the above example using [fully qualified syntax]:
//!
//! ```
//! use embedded_graphics::{prelude::*, primitives::Rectangle};
//! use embedded_layout::prelude::*;
//!
//! let rect = Rectangle::with_size(Point::zero(), Size::new(10, 10));
//! let size = View::size(&rect); //< Note that we are explicitly picking which `size` to call
//! let size = Dimensions::size(&rect);
//! ```
//!
//! ## Examples
//!
//! The examples are based on [the `embedded-graphics` simulator]. The simulator is built on top of
//! `SDL2`. If you don't have that installed, set the `EG_SIMULATOR_DUMP="screenshot.png"`
//! environment variable so that running the examples produce a screenshot image instead of a window.
//!
//! ### Draw some text to the center of the display
//!
//! ```
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
//!
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     style::TextStyleBuilder,
//! };
//! use embedded_layout::prelude::*;
//!
//! // Create a Rectangle from the display's dimensions
//! let display_area = display.display_area();
//!
//! let text_style = TextStyleBuilder::new(Font6x8)
//!     .text_color(BinaryColor::On)
//!     .build();
//!
//! Text::new("Hello, World!", Point::zero())
//!     .into_styled(text_style)
//!     // align text to the display
//!     .align_to(&display_area, horizontal::Center, vertical::Center)
//!     .draw(&mut display)
//!     .unwrap();
//! ```
//!
//! ### Use [`LinearLayout`] to arrange multiple objects
//!
//! ```
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
//!
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     style::TextStyleBuilder,
//! };
//! use embedded_layout::{layout::linear::LinearLayout, prelude::*};
//!
//! let display_area = display.display_area();
//!
//! let text_style = TextStyleBuilder::new(Font6x8)
//!     .text_color(BinaryColor::On)
//!     .build();
//!
//! LinearLayout::vertical()
//!     .with_alignment(horizontal::Center)
//!     .add_view(Text::new("Vertical", Point::zero()).into_styled(text_style))
//!     .add_view(Text::new("Linear", Point::zero()).into_styled(text_style))
//!     .add_view(Text::new("Layout", Point::zero()).into_styled(text_style))
//!     .arrange()
//!     .align_to(&display_area, horizontal::Center, vertical::Center)
//!     .draw(&mut display)
//!     .unwrap();
//! ```
//!
//! [`embedded-graphics`]: https://github.com/jamwaffles/embedded-graphics/
//! [the `embedded-graphics` simulator]: https://github.com/jamwaffles/embedded-graphics/tree/master/simulator
//! [fully qualified syntax]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
//! [`View`]: crate::View
//! [`LinearLayout`]: crate::layout::linear::LinearLayout

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]
#![deny(clippy::missing_inline_in_public_items)]

use embedded_graphics::{geometry::Point, prelude::*, primitives::Rectangle};

pub mod align;
pub mod layout;
mod utils;

use utils::rect_helper::RectSize;

/// The essentials
pub mod prelude {
    pub use crate::{
        align::{horizontal, vertical, Align},
        utils::{display_area::DisplayArea, rect_helper::RectExt},
        View,
    };

    pub use embedded_graphics::{
        drawable::{Drawable, Pixel},
        fonts::Font,
        geometry::{Point, Size},
        image::{ImageDimensions, IntoPixelIter},
        pixelcolor::{raw::RawData, GrayColor, IntoStorage, PixelColor, RgbColor},
        primitives::Primitive,
    };
}

/// A `View` is the base unit for most of the `embedded-layout` operations.
///
/// `View`s must have a size and a position, so they need to implement the `Dimensions` and
/// `Transform` traits.
///
/// See the `custom_view` example for how you can define more complex views.
pub trait View {
    /// Get the size of a View.
    #[inline]
    fn size(&self) -> Size {
        RectSize::size(&self.bounds())
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
    use embedded_graphics::primitives::Rectangle;

    #[test]
    fn test_size() {
        let rect = Rectangle::new(Point::zero(), Point::new(1, 2));

        assert_eq!(rect.size(), Size::new(2, 3));
    }
}
