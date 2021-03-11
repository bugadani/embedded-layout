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
    prelude::*,
    view_group::ViewGroup,
};

mod orientation;
mod secondary_alignment;
pub mod spacing;

use embedded_graphics::{primitives::Rectangle, DrawTarget};
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
pub struct LinearLayout<LD, VG> {
    direction: LD,
    views: VG,
}

impl<VG> LinearLayout<Horizontal<vertical::Bottom, Tight>, VG>
where
    VG: ViewGroup,
{
    /// Create a new, empty [`LinearLayout`] that places views left to right
    #[inline]
    #[must_use]
    pub fn horizontal(views: VG) -> Self {
        Self {
            direction: Horizontal::default(),
            views,
        }
    }
}

impl<VG: ViewGroup> LinearLayout<Vertical<horizontal::Left, Tight>, VG> {
    /// Create a new, empty [`LinearLayout`] that places views top to bottom
    #[inline]
    #[must_use]
    pub fn vertical(views: VG) -> Self {
        Self {
            direction: Vertical::default(),
            views,
        }
    }
}

impl<S, ELS, VG> LinearLayout<Horizontal<S, ELS>, VG>
where
    S: SecondaryAlignment + VerticalAlignment,
    ELS: ElementSpacing,
    VG: ViewGroup,
{
    /// Change the secondary alignment for this [`LinearLayout`] object.
    ///
    /// For layouts created using [`LinearLayout::horizontal`] the secondary alignment is [`vertical`].
    ///
    /// [`LinearLayout::horizontal`]: crate::layout::linear::LinearLayout::horizontal
    /// [`vertical`]: crate::align::vertical
    #[inline]
    pub fn with_alignment<Sec>(self, alignment: Sec) -> LinearLayout<Horizontal<Sec, ELS>, VG>
    where
        Sec: SecondaryAlignment + VerticalAlignment,
    {
        LinearLayout {
            direction: self.direction.with_secondary_alignment(alignment),
            views: self.views,
        }
    }

    /// Change the element spacing
    ///
    /// For available values and their properties, see [spacing]
    ///
    /// [spacing]: crate::layout::linear::spacing
    #[inline]
    pub fn with_spacing<ES>(self, spacing: ES) -> LinearLayout<Horizontal<S, ES>, VG>
    where
        ES: ElementSpacing,
    {
        LinearLayout {
            direction: self.direction.with_spacing(spacing),
            views: self.views,
        }
    }
}

impl<S, ELS, VG> LinearLayout<Vertical<S, ELS>, VG>
where
    S: SecondaryAlignment + HorizontalAlignment,
    ELS: ElementSpacing,
    VG: ViewGroup,
{
    /// Change the secondary alignment for this [`LinearLayout`] object.
    ///
    /// For layouts created using [`LinearLayout::vertical`] the secondary alignment is [`horizontal`].
    ///
    /// [`LinearLayout::vertical`]: crate::layout::linear::LinearLayout::vertical
    /// [`horizontal`]: crate::align::horizontal
    #[inline]
    pub fn with_alignment<Sec>(self, alignment: Sec) -> LinearLayout<Vertical<Sec, ELS>, VG>
    where
        Sec: SecondaryAlignment + HorizontalAlignment,
    {
        LinearLayout {
            direction: self.direction.with_secondary_alignment(alignment),
            views: self.views,
        }
    }

    /// Change the element spacing
    ///
    /// For available values and their properties, see [spacing]
    ///
    /// [spacing]: crate::layout::linear::spacing
    #[inline]
    pub fn with_spacing<ES>(self, spacing: ES) -> LinearLayout<Vertical<S, ES>, VG>
    where
        ES: ElementSpacing,
    {
        LinearLayout {
            direction: self.direction.with_spacing(spacing),
            views: self.views,
        }
    }
}

impl<LD, VG> LinearLayout<LD, VG>
where
    LD: Orientation,
    VG: ViewGroup,
{
    /// Arrange the views according to the layout properties and return the views as a [`ViewGroup`].
    /// Note: The top right point is always `Point::zero()`. Change this by calling [`View::translate`] or
    /// [`Align`] methods.
    ///
    /// [`View::translate`]: crate::View::translate
    /// [`Align`]: crate::align::Align
    #[inline]
    pub fn arrange(mut self) -> Self {
        let view_count = self.views.len();

        // measure
        let mut size = self.views[0].size();
        for i in 1..view_count {
            let current_el_size = self.views[i].size();
            size = LD::Secondary::measure(size, current_el_size);
        }

        // arrange
        let mut bounds = Rectangle::with_size(Point::zero(), size);
        for i in 0..view_count {
            self.direction
                .place(&mut self.views[i], size, bounds, i, view_count);
            bounds = self.views[i].bounds();
        }

        self
    }
}

impl<LD, VG> View for LinearLayout<LD, VG>
where
    LD: Orientation,
    VG: ViewGroup,
{
    fn translate_impl(&mut self, by: Point) {
        View::translate_impl(&mut self.views, by);
    }

    fn bounds(&self) -> Rectangle {
        View::bounds(&self.views)
    }
}

impl<'a, C, LD, VG> Drawable<C> for &'a LinearLayout<LD, VG>
where
    C: PixelColor,
    LD: Orientation,
    VG: ViewGroup,
    &'a VG: Drawable<C>,
{
    fn draw<D>(self, display: &mut D) -> Result<(), <D as DrawTarget<C>>::Error>
    where
        D: DrawTarget<C>,
    {
        self.views.draw(display)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        layout::linear::{
            spacing::{DistributeFill, FixedMargin},
            LinearLayout,
        },
        prelude::*,
    };
    use embedded_graphics::{
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{Circle, Rectangle},
        style::PrimitiveStyle,
    };

    #[allow(dead_code)]
    fn compile_check() {
        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        let rect = Rectangle::with_size(Point::zero(), Size::new(10, 20)).into_styled(style);
        let circ = Circle::new(Point::zero(), 10).into_styled(style);
        let _ = LinearLayout::horizontal(Tail::new(rect).append(circ));
    }

    #[test]
    fn layout_size() {
        let rect = Rectangle::with_size(Point::zero(), Size::new(10, 20));
        let rect2 = Rectangle::with_size(Point::zero(), Size::new(10, 20));
        let size = LinearLayout::horizontal(Tail::new(rect).append(rect2))
            .arrange()
            .size();

        assert_eq!(Size::new(20, 20), size);

        let rect = Rectangle::with_size(Point::zero(), Size::new(10, 20));
        let rect2 = Rectangle::with_size(Point::zero(), Size::new(10, 20));
        let size = LinearLayout::vertical(Tail::new(rect).append(rect2))
            .arrange()
            .size();

        assert_eq!(Size::new(10, 40), size);
    }

    #[test]
    fn layout_arrange_vertical() {
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let rect = Rectangle::with_size(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::with_size(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);

        LinearLayout::vertical(Tail::new(rect).append(rect2))
            .arrange()
            .translate(Point::new(1, 2))
            .draw(&mut disp)
            .unwrap();

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
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let rect = Rectangle::with_size(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::with_size(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);

        LinearLayout::vertical(Tail::new(rect).append(rect2))
            .with_alignment(horizontal::Right)
            .arrange()
            .translate(Point::new(1, 2))
            .draw(&mut disp)
            .unwrap();

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
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let rect = Rectangle::with_size(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::with_size(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);

        LinearLayout::horizontal(Tail::new(rect).append(rect2))
            .arrange()
            .translate(Point::new(1, 2))
            .draw(&mut disp)
            .unwrap();

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
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let rect = Rectangle::with_size(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::with_size(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);

        LinearLayout::horizontal(Tail::new(rect).append(rect2))
            .with_alignment(vertical::Top)
            .arrange()
            .translate(Point::new(1, 2))
            .draw(&mut disp)
            .unwrap();

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
    fn layout_spacing_size() {
        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

        let rect = Rectangle::with_size(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::with_size(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);

        let size = LinearLayout::horizontal(Tail::new(rect).append(rect2))
            .with_spacing(FixedMargin(2))
            .with_alignment(vertical::Top)
            .arrange()
            .size();

        assert_eq!(Size::new(17, 10), size);

        let size = LinearLayout::vertical(Tail::new(rect).append(rect2))
            .with_spacing(FixedMargin(2))
            .arrange()
            .size();

        assert_eq!(Size::new(10, 17), size);
    }

    #[test]
    fn layout_spacing() {
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

        let rect = Rectangle::with_size(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::with_size(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);

        LinearLayout::horizontal(Tail::new(rect).append(rect2))
            .with_spacing(FixedMargin(2))
            .with_alignment(vertical::Top)
            .arrange()
            .translate(Point::new(1, 2))
            .draw(&mut disp)
            .unwrap();

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
    fn layout_spacing_distribute_overflow() {
        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

        let rect = Rectangle::with_size(Point::zero(), Size::new(5, 5)).into_styled(style);

        let layout = LinearLayout::horizontal(Tail::new(rect).append(rect).append(rect))
            .with_spacing(DistributeFill(11))
            .with_alignment(vertical::TopToBottom)
            .arrange();

        assert_eq!(Size::new(11, 15), layout.size());

        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        layout.arrange().draw(&mut disp).unwrap();
        assert_eq!(
            disp,
            MockDisplay::from_pattern(&[
                "#####      ",
                "#   #      ",
                "#   #      ",
                "#   #      ",
                "#####      ",
                "   #####   ",
                "   #   #   ",
                "   #   #   ",
                "   #   #   ",
                "   #####   ",
                "      #####",
                "      #   #",
                "      #   #",
                "      #   #",
                "      #####",
            ])
        );
    }

    #[test]
    fn layout_spacing_distribute_fill() {
        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

        let rect = Rectangle::with_size(Point::zero(), Size::new(2, 2)).into_styled(style);

        let view_group =
            LinearLayout::vertical(Tail::new(rect).append(rect).append(rect).append(rect))
                .with_spacing(DistributeFill(18))
                .arrange();

        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        view_group.draw(&mut disp).unwrap();
        assert_eq!(
            disp,
            MockDisplay::from_pattern(&[
                "##            ",
                "##            ",
                "              ",
                "              ",
                "              ",
                "              ",
                "##            ",
                "##            ",
                "              ",
                "              ",
                "              ",
                "##            ",
                "##            ",
                "              ",
                "              ",
                "              ",
                "##            ",
                "##            "
            ])
        );
    }

    #[test]
    fn layout_size_independent_of_view_location() {
        let rect = Rectangle::with_size(Point::zero(), Size::new(10, 20));
        let rect2 = Rectangle::with_size(Point::zero(), Size::new(10, 20));

        let size1 = LinearLayout::horizontal(Tail::new(rect).append(rect2))
            .arrange()
            .bounds()
            .size();

        let rect = rect.translate(Point::new(30, 50));

        let size2 = LinearLayout::horizontal(Tail::new(rect).append(rect2))
            .arrange()
            .bounds()
            .size();

        assert_eq!(size1, size2);
    }
}
