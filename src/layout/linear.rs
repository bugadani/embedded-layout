//! Linear layout

use super::*;

/// Helper trait that describes a layout direction.
pub trait LayoutDirection: Copy + Clone {}
pub trait LayoutOperation<LD: LayoutDirection> {
    fn measure(&self) -> Size;
    fn arrange(&self);
}

/// Horizontal layout direction
#[derive(Copy, Clone)]
pub struct Horizontal;
impl LayoutDirection for Horizontal {}
impl<V, VCE> LayoutOperation<Horizontal> for ViewLink<V, VCE>
where
    V: View,
    VCE: ViewChainElement + LayoutOperation<Horizontal>,
{
    fn measure(&self) -> Size {
        // Counting this way assumes that views are aligned and not cascading in the other direction
        let current_el_size = RectExt::size(&self.view.bounds());
        if VCE::IS_TERMINATOR {
            current_el_size
        } else {
            let prev_size = self.next.measure();
            Size::new(
                prev_size.width + current_el_size.width,
                prev_size.height.max(current_el_size.height),
            )
        }
    }

    fn arrange(&self) {
        if VCE::IS_TERMINATOR {
        } else {
        }
        todo!();
    }
}

impl LayoutOperation<Horizontal> for ChainTerminator {
    fn measure(&self) -> Size {
        Size::new(0, 0)
    }

    fn arrange(&self) {
        // Nothing to do
    }
}

/// Vertical layout direction
#[derive(Copy, Clone)]
pub struct Vertical;
impl LayoutDirection for Vertical {}
impl<V, VCE> LayoutOperation<Vertical> for ViewLink<V, VCE>
where
    V: View,
    VCE: ViewChainElement + LayoutOperation<Vertical>,
{
    fn measure(&self) -> Size {
        // Counting this way assumes that views are aligned and not cascading in the other direction
        let current_el_size = RectExt::size(&self.view.bounds());
        if VCE::IS_TERMINATOR {
            current_el_size
        } else {
            let prev_size = self.next.measure();
            Size::new(
                prev_size.width.max(current_el_size.width),
                prev_size.height + current_el_size.height,
            )
        }
    }

    fn arrange(&self) {
        if VCE::IS_TERMINATOR {
        } else {
        }
        todo!();
    }
}

impl LayoutOperation<Vertical> for ChainTerminator {
    fn measure(&self) -> Size {
        Size::new(0, 0)
    }

    fn arrange(&self) {
        // Nothing to do
    }
}

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

impl LinearLayout<Horizontal, ChainTerminator> {
    /// Create a new, empty `LinearLayout` that places views horizontally next to each other
    pub fn horizontal() -> Self {
        Self {
            direction: Horizontal,
            views: ViewGroup::new(),
        }
    }
}

impl LinearLayout<Vertical, ChainTerminator> {
    /// Create a new, empty `LinearLayout` that places views vertically next to each other
    pub fn vertical() -> Self {
        Self {
            direction: Vertical,
            views: ViewGroup::new(),
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
    pub fn arrange(self) -> ViewGroup<VCE> {
        todo!("actually arrange views");
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
