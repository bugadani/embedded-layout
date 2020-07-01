//! Linear layout
//!
//! Linear layout is used to arrange views along the horizontal or vertical axis.

use super::*;

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

/// Special case implementation for empty `LinearLayout`
impl<LD: LayoutDirection> LinearLayout<LD, ChainTerminator> {
    pub fn add_view<V: View>(self, view: V) -> LinearLayout<LD, ViewLink<V, ChainTerminator>> {
        // TODO place first view
        LinearLayout {
            direction: self.direction,
            views: self.views.add_view(view),
        }
    }
}

impl<VV: View, LD: LayoutDirection, VCE: ViewChainElement> LinearLayout<LD, ViewLink<VV, VCE>> {
    pub fn add_view<V: View>(self, view: V) -> LinearLayout<LD, ViewLink<V, ViewLink<VV, VCE>>> {
        // TODO place view
        LinearLayout {
            direction: self.direction,
            views: self.views.add_view(view),
        }
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