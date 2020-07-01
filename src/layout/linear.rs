//! Linear layout

use super::*;

/// Helper trait that describes a layout direction.
pub trait LayoutDirection: Copy + Clone {}

/// Horizontal layout direction
#[derive(Copy, Clone)]
pub struct Horizontal;
impl LayoutDirection for Horizontal {}

/// Vertical layout direction
#[derive(Copy, Clone)]
pub struct Vertical;
impl LayoutDirection for Vertical {}

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

impl<LD: LayoutDirection, VCE: ViewChainElement> LinearLayout<LD, VCE> {
    /// Arrange the views according to the layout properties and return the views as a `ViewGroup`.
    pub fn arrange(self) -> ViewGroup<VCE> {
        todo!("actually arrange views");
        self.views
    }

    /// Returns the current size the layout will take up after `arrange`.
    pub fn size(&self) -> Size {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, layout::linear::LinearLayout};
    use embedded_graphics::{
        style::PrimitiveStyle,
        pixelcolor::BinaryColor,
        primitives::{Circle, Rectangle},
    };

    #[test]
    fn sanity_check() {
        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        let rect = Rectangle::with_size(Point::zero(), Size::new(10, 20)).into_styled(style);
        let circ = Circle::new(Point::zero(), 10).into_styled(style);
        let _ = LinearLayout::horizontal().add_view(rect).add_view(circ);
    }
}