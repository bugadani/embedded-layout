//! Linear layout
//!
//! A linear layout is a list of [`View`]s that are placed one after the other along
//! the horizontal or vertical axis.
//!
//! The main flow when working with a [`LinearLayout`] is the following:
//!  - Create the layout
//!    * you need to choose which orientation you want your views arranged in
//!    * pass in your views wrapped in a [`ViewGroup`].
//!  - Optionally, set [secondary alignment]
//!  - Optionally, set [element spacing]
//!  - Call [`LinearLayout::arrange`] to finalize view placement
//!  - Align the layout object to where you want it to be displayed
//!  - Call `draw` to display the views
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
//! #     mono_font::{ascii::FONT_6X9, MonoTextStyle},
//! #     pixelcolor::BinaryColor,
//! #     text::Text,
//! #     prelude::*,
//! # };
//! let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
//! let _ = LinearLayout::vertical(
//!     Chain::new(Text::new("Hello,", Point::zero(), text_style))
//!         .append(Text::new("World!", Point::zero(), text_style)),
//! )
//! .arrange();
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
//! [`ViewGroup`]: crate::view_group::ViewGroup
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

use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::{PixelColor, Point},
    primitives::Rectangle,
    Drawable,
};
pub use orientation::{Horizontal, Orientation, Vertical};
pub use secondary_alignment::SecondaryAlignment;
pub use spacing::{ElementSpacing, FixedMargin};

use spacing::Tight;

/// `LinearLayout`
///
/// [`LinearLayout`] is used to arrange views along the horizontal or vertical axis.
///
/// For more information and examples see the [module level documentation](crate::layout::linear).
pub struct LinearLayout<LD, VG> {
    position: Point,
    direction: LD,
    views: VG,
}

impl<VG> LinearLayout<Horizontal<vertical::Bottom, Tight>, VG>
where
    VG: ViewGroup,
{
    /// Create a new [`LinearLayout`] that places views left to right
    #[inline]
    #[must_use]
    pub fn horizontal(views: VG) -> Self {
        Self {
            position: Point::new(0, 0),
            direction: Horizontal::default(),
            views,
        }
    }
}

impl<VG> LinearLayout<Vertical<horizontal::Left, Tight>, VG>
where
    VG: ViewGroup,
{
    /// Create a new [`LinearLayout`] that places views top to bottom
    #[inline]
    #[must_use]
    pub fn vertical(views: VG) -> Self {
        Self {
            position: Point::new(0, 0),
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
            position: self.position,
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
            position: self.position,
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
            position: self.position,
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
            position: self.position,
            direction: self.direction.with_spacing(spacing),
            views: self.views,
        }
    }
}

impl<LD, VG> Clone for LinearLayout<LD, VG>
where
    LD: Orientation,
    VG: ViewGroup + Clone,
{
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            direction: self.direction,
            views: self.views.clone(),
        }
    }
}

impl<LD, VG> LinearLayout<LD, VG>
where
    LD: Orientation,
    VG: ViewGroup,
{
    /// Consume the layout object and return the wrapped [`ViewGroup`].
    ///
    /// After calling `arrange()` it is no longer necessary to hold the views in a `LinearLayout`.
    /// Use this method to extract the original view group object if you need to work with the
    /// arranged views.
    ///
    /// # Example
    ///
    /// Arrange an array of `StyledText` objects, then check the second object's position.
    ///
    /// ```rust
    /// # use embedded_layout::prelude::*;
    /// # use embedded_layout::layout::linear::LinearLayout;
    /// # use embedded_graphics::{
    /// #     mono_font::{ascii::FONT_6X9, MonoTextStyle},
    /// #     pixelcolor::BinaryColor,
    /// #     prelude::*,
    /// #     mock_display::MockDisplay,
    /// #     text::Text,
    /// # };
    /// # let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
    /// #
    /// let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
    ///
    /// // First, wrap out views in a `ViewGroup`.
    /// let mut texts = [
    ///     Text::new("Hello,", Point::zero(), text_style),
    ///     Text::new("World!", Point::zero(), text_style)
    /// ];
    /// let mut views = Views::new(&mut texts);
    ///
    /// // Arrange our views and extract our original view group.
    /// let views = LinearLayout::vertical(views).arrange().into_inner();
    ///
    /// // We can access our `StyledText` objects now. Note that `Views` works like a slice!
    /// assert_eq!(Point::new(0, 9), views[1].bounds().top_left);
    ///
    /// // `Views` is also a drawable `ViewGroup`, so let's display our arranged text!
    /// views.draw(&mut display).unwrap();
    /// ```
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> VG {
        self.views
    }

    /// Arrange the views according to the layout properties and return the views as a [`ViewGroup`].
    /// Note: The top left point is always `Point::zero()`.
    ///
    /// [`View::translate`]: crate::View::translate
    /// [`Align`]: crate::align::Align
    #[inline]
    pub fn arrange(mut self) -> Self {
        let view_count = self.views.len();

        // measure
        let mut size = self.views.at(0).size();
        for i in 1..view_count {
            let current_el_size = self.views.at(i).size();
            size = LD::Secondary::measure(size, current_el_size);
        }

        // arrange
        let mut bounds = Rectangle::new(self.position, size);
        for i in 0..view_count {
            self.direction
                .place(self.views.at_mut(i), size, bounds, i, view_count);
            bounds = self.views.at(i).bounds();
        }

        self
    }
}

impl<LD, VG> View for LinearLayout<LD, VG>
where
    LD: Orientation,
    VG: ViewGroup,
{
    #[inline]
    fn translate_impl(&mut self, by: Point) {
        self.position += by;
        View::translate_impl(&mut self.views, by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        let bounds = View::bounds(&self.views);
        let top_left = bounds.top_left;
        let correction = self.position - top_left;

        bounds.translate(correction)
    }
}

impl<C, LD, VG> Drawable for LinearLayout<LD, VG>
where
    C: PixelColor,
    LD: Orientation,
    VG: ViewGroup + Drawable<Color = C>,
{
    type Color = C;
    type Output = ();

    #[inline]
    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        self.views.draw(display)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        layout::linear::{
            spacing::{DistributeFill, FixedMargin},
            LinearLayout,
        },
        object_chain::Chain,
        prelude::*,
    };
    use embedded_graphics::{
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        prelude::{Point, Primitive, Size},
        primitives::{Circle, PrimitiveStyle, Rectangle},
        Drawable,
    };

    #[allow(dead_code)]
    fn compile_check() {
        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        let rect = Rectangle::new(Point::zero(), Size::new(10, 20)).into_styled(style);
        let circ = Circle::new(Point::zero(), 10).into_styled(style);
        let _ = LinearLayout::horizontal(Chain::new(rect).append(circ));
    }

    #[test]
    fn layout_size() {
        let rect = Rectangle::new(Point::zero(), Size::new(10, 20));
        let rect2 = Rectangle::new(Point::zero(), Size::new(10, 20));
        let size = LinearLayout::horizontal(Chain::new(rect).append(rect2))
            .arrange()
            .size();

        assert_eq!(Size::new(20, 20), size);

        let rect = Rectangle::new(Point::zero(), Size::new(10, 20));
        let rect2 = Rectangle::new(Point::zero(), Size::new(10, 20));
        let size = LinearLayout::vertical(Chain::new(rect).append(rect2))
            .arrange()
            .size();

        assert_eq!(Size::new(10, 40), size);
    }

    #[test]
    fn layout_arrange_vertical() {
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let rect = Rectangle::new(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::new(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);

        LinearLayout::vertical(Chain::new(rect).append(rect2))
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
        let rect = Rectangle::new(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::new(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);

        LinearLayout::vertical(Chain::new(rect).append(rect2))
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
        let rect = Rectangle::new(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::new(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);

        LinearLayout::horizontal(Chain::new(rect).append(rect2))
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
        let rect = Rectangle::new(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::new(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);

        LinearLayout::horizontal(Chain::new(rect).append(rect2))
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

        let rect = Rectangle::new(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::new(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);

        let size = LinearLayout::horizontal(Chain::new(rect).append(rect2))
            .with_spacing(FixedMargin(2))
            .with_alignment(vertical::Top)
            .arrange()
            .size();

        assert_eq!(Size::new(17, 10), size);

        let size = LinearLayout::vertical(Chain::new(rect).append(rect2))
            .with_spacing(FixedMargin(2))
            .arrange()
            .size();

        assert_eq!(Size::new(10, 17), size);
    }

    #[test]
    fn layout_spacing() {
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

        let rect = Rectangle::new(Point::new(10, 30), Size::new(10, 5)).into_styled(style);
        let rect2 = Rectangle::new(Point::new(-50, 10), Size::new(5, 10)).into_styled(style);

        LinearLayout::horizontal(Chain::new(rect).append(rect2))
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

        let rect = Rectangle::new(Point::zero(), Size::new(5, 5)).into_styled(style);

        let layout = LinearLayout::horizontal(Chain::new(rect).append(rect).append(rect))
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

        let rect = Rectangle::new(Point::zero(), Size::new(2, 2)).into_styled(style);

        let view_group =
            LinearLayout::vertical(Chain::new(rect).append(rect).append(rect).append(rect))
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
        let rect = Rectangle::new(Point::zero(), Size::new(10, 20));
        let rect2 = Rectangle::new(Point::zero(), Size::new(10, 20));

        let size1 = LinearLayout::horizontal(Chain::new(rect).append(rect2))
            .arrange()
            .bounds()
            .size();

        let rect = rect.translate(Point::new(30, 50));

        let size2 = LinearLayout::horizontal(Chain::new(rect).append(rect2))
            .arrange()
            .bounds()
            .size();

        assert_eq!(size1, size2);
    }
}
