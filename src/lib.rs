//! Enable simple layout operations in [`embedded-graphics`]
//!
//! This crate extends [`embedded-graphics`] with tools that ease positioning of drawable objects.
//!
//! `embedded-layout` consists of three main parts:
//! - [alignments] that can be used to position two objects relative to one another
//!   * `horizontal`
//!     * `NoAlignment`, `Left`, `Right`, `Center`
//!     * `LeftToRight`, `RightToLeft`
//!   * `vertical`
//!     * `NoAlignment`, `Top`, `Bottom`, `Center`
//!     * `TopToBottom`, `BottomToTop`
//! - [layouts] that can be used to arrange multiple views
//!   * `LinearLayout`
//! - [view groups] which are collections of view objects
//!   * `Chain` to create ad-hoc collections (can hold views of different types)
//!   * `Views` to create view groups from arrays and slices (can only hold views of a single type)
//!   * `derive(ViewGroup)` to turn any plain old Rust struct into a view group
//!
//! # Views
//!
//! The term "view" refers to anything `embedded-layout` can work with. Basically, a view is an
//! object that can be displayed. [`View`] is the most basic trait in `embedded-layout`. Views
//! implement the [`View`] trait to enable translation and alignment operations on them, and also to
//! allow them to be used with layouts.
//!
//! [`View`] is implemented for [`embedded-graphics`] display objects. There's also an example about
//! how you can implement custom [`View`] objects.
//!
//! ## Examples
//!
//! The examples are based on [the `embedded-graphics` simulator]. The simulator is built on top of
//! `SDL2`. See the [simulator README] for more information.
//!
//! ### Draw some text to the center of the display
//!
//! ```
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
//! #
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     pixelcolor::BinaryColor,
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
//!     // align text to the center of the display
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
//! #
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     pixelcolor::BinaryColor,
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
//! LinearLayout::vertical(
//!     Chain::new(Text::new("Vertical", Point::zero()).into_styled(text_style))
//!         .append(Text::new("Linear", Point::zero()).into_styled(text_style))
//!         .append(Text::new("Layout", Point::zero()).into_styled(text_style))
//! )
//! .with_alignment(horizontal::Center)
//! .arrange()
//! .align_to(&display_area, horizontal::Center, vertical::Center)
//! .draw(&mut display)
//! .unwrap();
//! ```
//!
//! # A note on imports
//!
//! `embedded-layout` reexports most of `embedded-graphics`' `prelude` module. In most cases
//! this means you don't have to `use embedded_graphics::prelude::*`. In case you do, `Translate`
//! and `Dimensions` may interfere with `embedded-layout`'s [`View`], so if you are using functions
//! of those traits, you may need to use the [fully qualified syntax] (formerly UFCS):
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
//! [`embedded-graphics`]: https://crates.io/crates/embedded-graphics/0.6.2
//! [the `embedded-graphics` simulator]: https://crates.io/crates/embedded-graphics-simulator/0.2.1
//! [fully qualified syntax]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
//! [`View`]: crate::View
//! [layouts]: crate::layout
//! [`LinearLayout`]: crate::layout::linear::LinearLayout
//! [simulator README]: https://github.com/jamwaffles/embedded-graphics/tree/v0.6/simulator#usage-without-sdl2
//! [alignments]: crate::align
//! [view groups]: crate::view_group

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]
#![deny(clippy::missing_inline_in_public_items)]
#![warn(clippy::all)]

use embedded_graphics::{geometry::Point, prelude::*, primitives::Rectangle};

pub mod align;
pub mod layout;
pub mod object_chain;
pub mod utils;
pub mod view_group;

use utils::rect_helper::RectSize;

/// The essentials. Also contains most of `embedded-graphics'` prelude.
pub mod prelude {
    pub use crate::{
        align::{horizontal, vertical, Align},
        chain,
        object_chain::{Chain, Link},
        utils::{display_area::DisplayArea, rect_helper::RectExt},
        view_group::Views,
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
/// `View`s must have a size and a position.
///
/// See the `custom_view` example for how you can define more complex views.
pub trait View {
    /// Get the size of a View.
    #[inline]
    fn size(&self) -> Size {
        RectSize::size(self.bounds())
    }

    /// Object-safe version of `translate()`.
    fn translate_impl(&mut self, by: Point);

    /// Move the origin of an object by a given number of (x, y) pixels,
    /// mutating the object in place.
    /// If you a looking for a method to implement, you might want `translate_impl()` instead.
    fn translate_mut(&mut self, by: Point) -> &mut Self
    where
        Self: Sized,
    {
        self.translate_impl(by);
        self
    }

    /// Move the origin of an object by a given number of (x, y) pixels,
    /// returning a new object
    /// If you a looking for a method to implement, you might want `translate_impl()` instead.
    fn translate(mut self, by: Point) -> Self
    where
        Self: Sized,
    {
        self.translate_impl(by);
        self
    }

    /// Returns the bounding box of the `View` as a `Rectangle`
    fn bounds(&self) -> Rectangle;
}

impl<T> View for T
where
    T: Transform + Dimensions,
{
    #[inline]
    fn translate_impl(&mut self, by: Point) {
        Transform::translate_mut(self, by);
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

    #[allow(dead_code)]
    fn view_is_object_safe(_: &dyn View) {}

    #[test]
    fn test_size() {
        let rect = Rectangle::new(Point::zero(), Point::new(1, 2));

        assert_eq!(rect.size(), Size::new(2, 3));
    }
}
