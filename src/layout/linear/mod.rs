//! Linear layout
//!
//! A linear layout is a list of [`View`]s that are placed one after the other along
//! the horizontal or vertical axis.
//!
//! The main flow when working with a [`LinearLayout`] is the following:
//!  - Create the layout: you need to choose which orientation you want your views arranged in
//!  - Optionally, set [secondary alignment]
//!  - Optionally, set [element spacing]
//!  - Add views you want to arrange
//!  - Call [`LinearLayout::arrange`] to finalize view placement
//!  - Align the returned [`ViewGroup`] to where you want it to be displayed
//!  - Call `draw` to display the views
//!
//! *Note:* [`LinearLayout`] is implemented using object chaining so it's exact type depends on it's contents.
//!
//! # Orientation
//!
//! When constructing a [`LinearLayout`] object, you need to choose an orientation along which
//! the views will be arranged. This can either be horizontal or vertical.
//!
//! ## Examples:
//!
//! Create a [`LinearLayout`] with two pieces of text, where one is below the other:
//!
//! ```rust
//! # use embedded_layout::prelude::*;
//! # use embedded_layout::layout::linear::LinearLayout;
//! # use embedded_graphics::{
//! #    fonts::{Font6x8, Text},
//! #     pixelcolor::BinaryColor,
//! #     style::TextStyleBuilder,
//! # };
//! let text_style = TextStyleBuilder::new(Font6x8)
//!     .text_color(BinaryColor::On)
//!     .build();
//!
//! let _ = LinearLayout::vertical()
//!     .add_view(Text::new("Hello,", Point::zero()).into_styled(text_style))
//!     .add_view(Text::new("World!", Point::zero()).into_styled(text_style))
//!     .arrange();
//! ```
//!
//! # Secondary alignment
//!
//! Secondary alignment means the alignment on the "other" axis:
//!  - horizontal alignment in vertical linear layouts
//!  - vertical alignment in horizontal linear layouts
//!
//! By default, the secondary alignments are the following:
//!  - Horizontal orientation: [`vertical::Bottom`]
//!  - Vertical orientation: [`horizontal::Left`]
//!
//! Except for using the cascading (`XtoY`) secondary alignments, the [`LinearLayout`] will take up
//! as much space along the secondary alignment as the biggest element, i.e. vertical layouts
//! will be as wide as the widest view inside them.
//!
//! # Element spacing
//!
//! It's possible to modify how views are placed relative to one another.
//!  * The default is [`Tight`] which is equivalent to [`FixedMargin(0)`]
//!  * [`FixedMargin(margin)`]: `margin` px distance between views, where `margin` can be negative to overlap views
//!  * [`DistributeFill(size)`]: force the primary layout size to `size`, distribute views evenly
//!
//! [`View`]: crate::View
//! [`ViewGroup`]: crate::layout::ViewGroup
//! [`LinearLayout`]: crate::layout::linear::LinearLayout
//! [`LinearLayout::arrange`]: crate::layout::linear::LinearLayout::arrange
//! [secondary alignment]: crate::layout::linear::LinearLayout::with_alignment
//! [element spacing]: crate::layout::linear::LinearLayout::with_spacing
//! [`Tight`]: crate::layout::linear::spacing::Tight
//! [`FixedMargin(0)`]: crate::layout::linear::spacing::FixedMargin
//! [`FixedMargin(margin)`]: crate::layout::linear::spacing::FixedMargin
//! [`DistributeFill(size)`]: crate::layout::linear::spacing::DistributeFill
//! [`vertical::Bottom`]: crate::align::vertical::Bottom
//! [`horizontal::Left`]: crate::align::horizontal::Left

use crate::{
    align::{HorizontalAlignment, VerticalAlignment},
    layout::ViewGroup,
    prelude::*,
};

mod layout_element;
mod orientation;
mod secondary_alignment;
pub mod spacing;

pub use layout_element::LayoutElement;
pub use orientation::{Horizontal, Orientation, Vertical};
pub use secondary_alignment::SecondaryAlignment;
pub use spacing::{ElementSpacing, FixedMargin};

use spacing::Tight;

/// `LinearLayout`
///
/// [`LinearLayout`] is used to arrange views along the horizontal or vertical axis.
/// A [`LinearLayout`] object is not a `View`, it does not have a location, instead it is used to
/// arrange a group of views into a `ViewGroup` object using the `arrange` method. It does have a
/// `size` however.
///
/// For more information and examples see the [module level documentation](crate::layout::linear).
pub struct LinearLayout<LD: Orientation> {
    direction: LD,
}

impl LinearLayout<Horizontal<vertical::Bottom>> {
    /// Create a new, empty [`LinearLayout`] that places views left to right
    #[inline]
    #[must_use]
    pub fn horizontal() -> Self {
        Self {
            direction: Horizontal::default(),
        }
    }
}

impl LinearLayout<Vertical<horizontal::Left, Tight>> {
    /// Create a new, empty [`LinearLayout`] that places views top to bottom
    #[inline]
    #[must_use]
    pub fn vertical() -> Self {
        Self {
            direction: Vertical::default(),
        }
    }
}

impl<S, ELS> LinearLayout<Horizontal<S, ELS>>
where
    S: SecondaryAlignment + VerticalAlignment,
    ELS: ElementSpacing,
{
    /// Change the secondary alignment for this [`LinearLayout`] object.
    ///
    /// For layouts created using [`LinearLayout::horizontal`] the secondary alignment is [`vertical`].
    ///
    /// [`LinearLayout::horizontal`]: crate::layout::linear::LinearLayout::horizontal
    /// [`vertical`]: crate::align::vertical
    #[inline]
    pub fn with_alignment<Sec>(self, alignment: Sec) -> LinearLayout<Horizontal<Sec, ELS>>
    where
        Sec: SecondaryAlignment + VerticalAlignment,
    {
        LinearLayout {
            direction: self.direction.with_secondary_alignment(alignment),
        }
    }

    /// Change the element spacing
    ///
    /// For available values and their properties, see [spacing]
    ///
    /// [spacing]: crate::layout::linear::spacing
    #[inline]
    pub fn with_spacing<ES>(self, spacing: ES) -> LinearLayout<Horizontal<S, ES>>
    where
        ES: ElementSpacing,
    {
        LinearLayout {
            direction: self.direction.with_spacing(spacing),
        }
    }
}

impl<S, ELS> LinearLayout<Vertical<S, ELS>>
where
    S: SecondaryAlignment + HorizontalAlignment,
    ELS: ElementSpacing,
{
    /// Change the secondary alignment for this [`LinearLayout`] object.
    ///
    /// For layouts created using [`LinearLayout::vertical`] the secondary alignment is [`horizontal`].
    ///
    /// [`LinearLayout::vertical`]: crate::layout::linear::LinearLayout::vertical
    /// [`horizontal`]: crate::align::horizontal
    #[inline]
    pub fn with_alignment<Sec>(self, alignment: Sec) -> LinearLayout<Vertical<Sec, ELS>>
    where
        Sec: SecondaryAlignment + HorizontalAlignment,
    {
        LinearLayout {
            direction: self.direction.with_secondary_alignment(alignment),
        }
    }

    /// Change the element spacing
    ///
    /// For available values and their properties, see [spacing]
    ///
    /// [spacing]: crate::layout::linear::spacing
    #[inline]
    pub fn with_spacing<ES>(self, spacing: ES) -> LinearLayout<Vertical<S, ES>>
    where
        ES: ElementSpacing,
    {
        LinearLayout {
            direction: self.direction.with_spacing(spacing),
        }
    }
}

impl<LD> LinearLayout<LD>
where
    LD: Orientation,
{
    /// Arrange the views according to the layout properties and return the views as a [`ViewGroup`].
    /// Note: The top right point is always `Point::zero()`. Change this by calling [`View::translate`] or
    /// [`Align`] methods.
    ///
    /// [`View::translate`]: crate::View::translate
    /// [`Align`]: crate::align::Align
    #[inline]
    pub fn arrange(mut self, view_group: &mut impl ViewGroup) {
        todo!()
    }
}
