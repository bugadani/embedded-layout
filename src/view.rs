//! `View` trait
//!
//! `View` is a basic trait that represents something that can be displayed.
use embedded_graphics::prelude::*;

/// A view is the base unit for most of the `embedded-layout` operations.
///
/// Views must have a size and a position, so they need to implement the `Dimensions` and
/// `Transform` traits.
pub trait View: Transform + Dimensions {
    /// Get the size of a View.
    fn size(&self) -> Size {
        // TODO: remove if fixed in embedded-graphics
        let top_left = self.top_left();
        let bottom_right = self.bottom_right();

        let width = (top_left.x - bottom_right.x).abs() as u32 + 1;
        let height = (top_left.y - bottom_right.y).abs() as u32 + 1;

        Size::new(width, height)
    }
}

impl<T> View for T where T: Transform + Dimensions {}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use embedded_graphics::{
        geometry::{Point, Size},
        primitives::Rectangle,
    };

    #[test]
    fn test_size() {
        let rect = Rectangle::new(Point::zero(), Point::new(1, 2));

        assert_eq!(rect.size(), Size::new(2, 3));
    }
}
