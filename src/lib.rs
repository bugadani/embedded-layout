//! Enable simple layout operations in [`embedded-graphics`]
//!
//! This crate extends [`embedded-graphics`] with tools that ease positioning of drawable objects.
//!
//! `embedded-layout` consists of two main parts:
//! - [alignments] that can be used to position two objects relative to one another
//!   * `horizontal`
//!     * `NoAlignment`, `Left`, `Right`, `Center`
//!     * `LeftToRight`, `RightToLeft`
//!   * `vertical`
//!     * `NoAlignment`, `Top`, `Bottom`, `Center`
//!     * `TopToBottom`, `BottomToTop`
//! - [layouts] that can be used to arrange multiple views
//!   * `ViewGroup`
//!   * `LinearLayout`
//!
//! # Views
//!
//! The term "view" refers to anything `embedded-layout` can work with. Basically, a view is an
//! object that can be displayed. [`View`] is the most basic trait in `embedded-layout`. Views
//! implement [`View`] to enable translation and alignment operations on them, and also to allow
//! them to be used with layouts.
//!
//! [`View`] is implemented for [`embedded-graphics`] display objects. There's also an example about
//! how to implement custom [`View`] objects.
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
//! [`embedded-graphics`]: https://github.com/jamwaffles/embedded-graphics/
//! [the `embedded-graphics` simulator]: https://github.com/jamwaffles/embedded-graphics/tree/master/simulator
//! [fully qualified syntax]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
//! [`View`]: crate::View
//! [layouts]: crate::layout
//! [`LinearLayout`]: crate::layout::linear::LinearLayout
//! [simulator README]: https://github.com/jamwaffles/embedded-graphics/tree/master/simulator#usage-without-sdl2
//! [alignments]: crate::align

#![cfg_attr(not(test), no_std)]
//#![deny(missing_docs)]
#![deny(clippy::missing_inline_in_public_items)]
#![warn(clippy::all)]

use core::ops::{Index, IndexMut};

use embedded_graphics::{geometry::Point, prelude::*, primitives::Rectangle};

pub mod align;
pub mod layout;
pub mod utils;

/// The essentials. Also contains most of `embedded-graphics'` prelude.
pub mod prelude {
    pub use crate::{
        align::{horizontal, vertical, Align},
        chain,
        utils::object_chain::{ChainElement, Link, Tail},
        View,
    };
}

/// A `View` is the base unit for most of the `embedded-layout` operations.
///
/// `View`s must have a size and a position, so they need to implement the `Dimensions` and
/// `Transform` traits.
///
/// See the `custom_view` example for how you can define more complex views.
pub trait View {
    /// Move the origin of an object by a given number of (x, y) pixels,
    /// mutating the object in place
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
        Transform::translate_mut(self, by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        Dimensions::bounding_box(self)
    }
}

pub trait ViewGroup: Index<usize, Output = dyn View> + IndexMut<usize> {
    fn len() -> usize;
}

#[cfg(test)]
mod test {
    use embedded_graphics::{
        pixelcolor::BinaryColor,
        primitives::{Line, PrimitiveStyle},
        Styled,
    };

    use super::*;

    struct S {
        a: Styled<Line, PrimitiveStyle<BinaryColor>>,
        b: Styled<Rectangle, PrimitiveStyle<BinaryColor>>,
    }

    impl Index<usize> for S {
        type Output = dyn View;

        fn index(&self, index: usize) -> &Self::Output {
            match index {
                0 => &self.a,
                1 => &self.b,
                _ => panic!(),
            }
        }
    }

    impl IndexMut<usize> for S {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            match index {
                0 => &mut self.a,
                1 => &mut self.b,
                _ => panic!(),
            }
        }
    }

    impl ViewGroup for S {
        fn len() -> usize {
            2
        }
    }

    impl Drawable for S {
        type Color = BinaryColor;

        fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
        where
            D: DrawTarget<Color = Self::Color>,
        {
            self.a.draw(display)?;
            self.b.draw(display)?;

            Ok(())
        }
    }
}
