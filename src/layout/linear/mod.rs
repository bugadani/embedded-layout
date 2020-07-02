//! Linear layout

use crate::{
    align::{HorizontalAlignment, VerticalAlignment},
    layout::{ChainTerminator, ViewChainElement, ViewGroup, ViewLink},
    prelude::*,
};
use embedded_graphics::primitives::Rectangle;

mod layout_direction;
mod layout_operation;
mod secondary_alignment;

pub use layout_direction::{Horizontal, LayoutDirection, Vertical};
pub use secondary_alignment::SecondaryAlignment;

use layout_operation::LayoutOperation;

/// LinearLayout
///
/// `LinearLayout` is used to arrange views along the horizontal or vertical axis.
/// A `LinearLayout` object is not a `View`, it does not have a location, instead it is used to
/// arrange a group of views into a `ViewGroup` object using the `arrange` method. It does have a
/// `size` however.
pub struct LinearLayout<LD: LayoutDirection, VC: ViewChainElement> {
    direction: LD,
    views: ViewGroup<VC>,
}

impl LinearLayout<Horizontal<vertical::Bottom>, ChainTerminator> {
    /// Create a new, empty `LinearLayout` that places views horizontally next to each other
    pub fn horizontal() -> Self {
        Self {
            direction: Horizontal::default(),
            views: ViewGroup::new(),
        }
    }
}

impl LinearLayout<Vertical<horizontal::Left>, ChainTerminator> {
    /// Create a new, empty `LinearLayout` that places views vertically next to each other
    pub fn vertical() -> Self {
        Self {
            direction: Vertical::default(),
            views: ViewGroup::new(),
        }
    }
}

impl<S, VCE> LinearLayout<Horizontal<S>, VCE>
where
    S: SecondaryAlignment + VerticalAlignment,
    VCE: ViewChainElement,
{
    /// Create a new, empty `LinearLayout` that places views horizontally next to each other
    pub fn with_alignment<Sec>(self, alignment: Sec) -> LinearLayout<Horizontal<Sec>, VCE>
    where
        Sec: SecondaryAlignment + VerticalAlignment,
    {
        LinearLayout {
            direction: Horizontal {
                secondary: alignment,
            },
            views: self.views,
        }
    }
}

impl<S, VCE> LinearLayout<Vertical<S>, VCE>
where
    S: SecondaryAlignment + HorizontalAlignment,
    VCE: ViewChainElement,
{
    /// Create a new, empty `LinearLayout` that places views horizontally next to each other
    pub fn with_alignment<Sec>(self, alignment: Sec) -> LinearLayout<Vertical<Sec>, VCE>
    where
        Sec: SecondaryAlignment + HorizontalAlignment,
    {
        LinearLayout {
            direction: Vertical {
                secondary: alignment,
            },
            views: self.views,
        }
    }
}

impl<LD: LayoutDirection, VCE: ViewChainElement> LinearLayout<LD, VCE> {
    /// Add a `View` to the layout
    ///
    /// Views will be laid out sequentially, keeping the order in which they were added to the
    /// layout.
    pub fn add_view<V: View>(self, view: V) -> LinearLayout<LD, ViewLink<V, VCE>> {
        LinearLayout {
            direction: self.direction,
            views: self.views.add_view(view),
        }
    }
}

impl<LD, VCE> LinearLayout<LD, VCE>
where
    LD: LayoutDirection,
    VCE: ViewChainElement + LayoutOperation<LD>,
{
    /// Arrange the views according to the layout properties and return the views as a `ViewGroup`.
    /// Notes:
    ///  - the top right point is always (0, 0)
    ///  - for horizontal layouts, the elements will be vertically bottom aligned
    ///  - for vertical layouts, the elements will be horizontally left aligned
    pub fn arrange(mut self) -> ViewGroup<VCE> {
        let bounds = Rectangle::with_size(Point::zero(), self.size());
        self.views.views.arrange(bounds);
        self.views
    }

    /// Returns the current size the layout will take up after `arrange`.
    pub fn size(&self) -> Size {
        self.views.views.measure()
    }
}

#[cfg(test)]
mod test {
    use crate::{layout::linear::LinearLayout, prelude::*};
    use embedded_graphics::mock_display::MockDisplay;
    use embedded_graphics::{
        pixelcolor::BinaryColor,
        primitives::{Circle, Rectangle},
        style::PrimitiveStyle,
    };

    #[test]
    fn sanity_check() {
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
