//! Linear layout
//!
//! A linear layout is a list of `View`s that are placed one after the other along
//! the horizontal or vertical axis.
//!
//! The main flow when working with a `LinearLayout` is the following:
//!  - Create the layout: you need to choose which orientation you want your views arranged in
//!  - Optionally, set secondary alignment
//!  - Add views you want to arrange
//!  - Call `arrange` to finalize view placement
//!  - Align the returned `ViewGroup` to where you want it to be displayed
//!  - Call `draw` to display the views
//!
//! # Orientation
//!
//! When constructing a `LinearLayout` object, you need to choose an orientation along which
//! the views will be arranged. This can either be horizontal or vertical.
//!
//! ## Examples:
//!
//! Create a `LinearLayout` with two pieces of text, where one is below the other:
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
//!  - Horizontal orientation: `vertical::Bottom`
//!  - Vertical orientation: `horizontal::Left`
//!

use crate::{
    align::{HorizontalAlignment, VerticalAlignment},
    layout::{Guard, Link, ViewChainElement, ViewGroup},
    prelude::*,
};
use embedded_graphics::primitives::Rectangle;

mod layout_element;
mod orientation;
mod secondary_alignment;
mod spacing;

pub use orientation::{Horizontal, Orientation, Vertical};
pub use secondary_alignment::SecondaryAlignment;

pub use spacing::{ElementSpacing, FixedMargin};

use layout_element::LayoutElement;
use spacing::Tight;

/// LinearLayout
///
/// `LinearLayout` is used to arrange views along the horizontal or vertical axis.
/// A `LinearLayout` object is not a `View`, it does not have a location, instead it is used to
/// arrange a group of views into a `ViewGroup` object using the `arrange` method. It does have a
/// `size` however.
///
/// For more information and examples see the module level documentation.
pub struct LinearLayout<LD: Orientation, ELS: ElementSpacing, VC: ViewChainElement> {
    direction: LD,
    spacing: ELS,
    views: ViewGroup<VC>,
}

impl LinearLayout<Horizontal<vertical::Bottom>, Tight, Guard> {
    /// Create a new, empty `LinearLayout` that places views horizontally next to each other
    #[inline]
    pub fn horizontal() -> Self {
        Self {
            direction: Horizontal::default(),
            spacing: Tight,
            views: ViewGroup::new(),
        }
    }
}

impl LinearLayout<Vertical<horizontal::Left>, Tight, Guard> {
    /// Create a new, empty `LinearLayout` that places views vertically next to each other
    #[inline]
    pub fn vertical() -> Self {
        Self {
            direction: Vertical::default(),
            spacing: Tight,
            views: ViewGroup::new(),
        }
    }
}

impl<S, ELS, VCE> LinearLayout<Horizontal<S>, ELS, VCE>
where
    S: SecondaryAlignment + VerticalAlignment,
    ELS: ElementSpacing,
    VCE: ViewChainElement,
{
    /// Create a new, empty `LinearLayout` that places views horizontally next to each other
    #[inline]
    pub fn with_alignment<Sec>(self, alignment: Sec) -> LinearLayout<Horizontal<Sec>, ELS, VCE>
    where
        Sec: SecondaryAlignment + VerticalAlignment,
    {
        LinearLayout {
            direction: Horizontal {
                secondary: alignment,
            },
            spacing: self.spacing,
            views: self.views,
        }
    }
}

impl<S, ELS, VCE> LinearLayout<Vertical<S>, ELS, VCE>
where
    S: SecondaryAlignment + HorizontalAlignment,
    ELS: ElementSpacing,
    VCE: ViewChainElement,
{
    /// Create a new, empty `LinearLayout` that places views horizontally next to each other
    #[inline]
    pub fn with_alignment<Sec>(self, alignment: Sec) -> LinearLayout<Vertical<Sec>, ELS, VCE>
    where
        Sec: SecondaryAlignment + HorizontalAlignment,
    {
        LinearLayout {
            direction: Vertical {
                secondary: alignment,
            },
            spacing: self.spacing,
            views: self.views,
        }
    }
}

impl<LD, ELS, LE> LinearLayout<LD, ELS, LE>
where
    LD: Orientation,
    ELS: ElementSpacing,
    LE: LayoutElement<LD>,
{
    /// Add a `View` to the layout
    ///
    /// Views will be laid out sequentially, keeping the order in which they were added to the
    /// layout.
    #[inline]
    pub fn add_view<V: View>(self, view: V) -> LinearLayout<LD, ELS, Link<V, LE>> {
        LinearLayout {
            direction: self.direction,
            spacing: self.spacing,
            views: self.views.add_view(view),
        }
    }

    /// Change the element spacing
    #[inline]
    pub fn with_spacing<ES: ElementSpacing>(self, spacing: ES) -> LinearLayout<LD, ES, LE> {
        LinearLayout {
            direction: self.direction,
            spacing: spacing,
            views: self.views,
        }
    }

    /// Arrange the views according to the layout properties and return the views as a `ViewGroup`.
    /// Notes:
    ///  - the top right point is always (0, 0)
    ///  - for horizontal layouts, the elements will be vertically bottom aligned
    ///  - for vertical layouts, the elements will be horizontally left aligned
    #[inline]
    pub fn arrange(mut self) -> ViewGroup<LE> {
        let bounds = Rectangle::with_size(Point::zero(), self.size());
        self.views.views.arrange(bounds);
        self.views
    }

    /// Returns the current size the layout will take up after `arrange`.
    #[inline]
    pub fn size(&self) -> Size {
        self.views.views.measure()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        layout::linear::{spacing::FixedMargin, LinearLayout},
        prelude::*,
    };
    use embedded_graphics::{
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{Circle, Rectangle},
        style::PrimitiveStyle,
    };

    #[test]
    fn compile_check() {
        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        let rect = Rectangle::with_size(Point::zero(), Size::new(10, 20)).into_styled(style);
        let circ = Circle::new(Point::zero(), 10).into_styled(style);
        let _ = LinearLayout::horizontal().add_view(rect).add_view(circ);
    }

    #[test]
    fn layout_size() {
        let rect = Rectangle::with_size(Point::zero(), Size::new(10, 20));
        let rect2 = Rectangle::with_size(Point::zero(), Size::new(10, 20));
        let size = LinearLayout::horizontal()
            .add_view(rect)
            .add_view(rect2)
            .size();

        assert_eq!(Size::new(20, 20), size);

        let rect = Rectangle::with_size(Point::zero(), Size::new(10, 20));
        let rect2 = Rectangle::with_size(Point::zero(), Size::new(10, 20));
        let size = LinearLayout::vertical()
            .add_view(rect)
            .add_view(rect2)
            .size();

        assert_eq!(Size::new(10, 40), size);
    }

    #[test]
    fn layout_arrange_vertical() {
        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let rect = Rectangle::with_size(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::with_size(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);
        let mut view_group = LinearLayout::vertical()
            .add_view(rect)
            .add_view(rect2)
            .arrange();

        view_group.translate(Point::new(1, 2));

        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        view_group.draw(&mut disp).unwrap();
        assert_eq!(
            disp,
            MockDisplay::from_pattern(&[
                "           ",
                "           ",
                " ##########",
                " #        #",
                " #        #",
                " #        #",
                " ##########",
                " #####     ",
                " #   #     ",
                " #   #     ",
                " #   #     ",
                " #   #     ",
                " #   #     ",
                " #   #     ",
                " #   #     ",
                " #   #     ",
                " #####     ",
            ])
        );
    }

    #[test]
    fn layout_arrange_vertical_secondary() {
        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let rect = Rectangle::with_size(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::with_size(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);
        let mut view_group = LinearLayout::vertical()
            .with_alignment(horizontal::Right)
            .add_view(rect)
            .add_view(rect2)
            .arrange();

        view_group.translate(Point::new(1, 2));

        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        view_group.draw(&mut disp).unwrap();
        assert_eq!(
            disp,
            MockDisplay::from_pattern(&[
                "           ",
                "           ",
                " ##########",
                " #        #",
                " #        #",
                " #        #",
                " ##########",
                "      #####",
                "      #   #",
                "      #   #",
                "      #   #",
                "      #   #",
                "      #   #",
                "      #   #",
                "      #   #",
                "      #   #",
                "      #####",
            ])
        );
    }

    #[test]
    fn layout_arrange_horizontal() {
        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let rect = Rectangle::with_size(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::with_size(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);
        let mut view_group = LinearLayout::horizontal()
            .add_view(rect)
            .add_view(rect2)
            .arrange();

        view_group.translate(Point::new(1, 2));

        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        view_group.draw(&mut disp).unwrap();
        assert_eq!(
            disp,
            MockDisplay::from_pattern(&[
                "                ",
                "                ",
                "           #####",
                "           #   #",
                "           #   #",
                "           #   #",
                "           #   #",
                " ###########   #",
                " #        ##   #",
                " #        ##   #",
                " #        ##   #",
                " ###############",
            ])
        );
    }

    #[test]
    fn layout_arrange_horizontal_secondary() {
        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let rect = Rectangle::with_size(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::with_size(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);
        let mut view_group = LinearLayout::horizontal()
            .with_alignment(vertical::Top)
            .add_view(rect)
            .add_view(rect2)
            .arrange();

        view_group.translate(Point::new(1, 2));

        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        view_group.draw(&mut disp).unwrap();
        assert_eq!(
            disp,
            MockDisplay::from_pattern(&[
                "                ",
                "                ",
                " ###############",
                " #        ##   #",
                " #        ##   #",
                " #        ##   #",
                " ###########   #",
                "           #   #",
                "           #   #",
                "           #   #",
                "           #   #",
                "           #####",
            ])
        );
    }

    #[test]
    fn layout_spacing() {
        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let rect = Rectangle::with_size(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::with_size(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);
        let mut view_group = LinearLayout::horizontal()
            .with_spacing(FixedMargin(2))
            .with_alignment(vertical::Top)
            .add_view(rect)
            .add_view(rect2)
            .arrange();

        view_group.translate(Point::new(1, 2));

        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        view_group.draw(&mut disp).unwrap();
        assert_eq!(
            disp,
            MockDisplay::from_pattern(&[
                "                  ",
                "                  ",
                " ##########  #####",
                " #        #  #   #",
                " #        #  #   #",
                " #        #  #   #",
                " ##########  #   #",
                "             #   #",
                "             #   #",
                "             #   #",
                "             #   #",
                "             #####",
            ])
        );
    }

    #[test]
    fn layout_size_independent_of_view_location() {
        let rect = Rectangle::with_size(Point::zero(), Size::new(10, 20));
        let rect2 = Rectangle::with_size(Point::zero(), Size::new(10, 20));
        let size1 = LinearLayout::horizontal()
            .add_view(rect)
            .add_view(rect2)
            .size();
        let size2 = LinearLayout::horizontal()
            .add_view(rect.translate(Point::new(30, 50)))
            .add_view(rect2)
            .size();

        assert_eq!(size1, size2);
    }
}
