//! `Rectangle` utility methods
//!
//! This module implements a few usfeul extensions to `Rectangle`.
use embedded_graphics::{prelude::*, primitives::Rectangle};

/// The trait that describes the extension methods.
pub trait RectExt {

    /// Create a new `Rectangle` from a top left point and a `Size`
    fn with_size(top_left: Point, size: Size) -> Rectangle;

    /// Return the `Size` of the `Rectangle`
    ///
    /// The `size` method provided by `embedded-graphics 0.6.2` returns an incorrect value.
    fn size(&self) -> Size;

    /// Return the horizontal center coordinate
    ///
    /// *Note:* when an object's width is an even number, the returned center point will not
    ///         be perfectly in the middle.
    fn center_x(&self) -> i32;

    /// Return the vertical center coordinate
    ///
    /// *Note:* when an object's height is an even number, the returned center point will not
    ///         be perfectly in the middle.
    fn center_y(&self) -> i32;

    /// Return the center point
    /// *Note:* when an object's width or height is an even number, the returned center point will
    ///         not be perfectly in the middle.
    fn center(&self) -> Point;
}

impl RectExt for Rectangle {
    fn with_size(top_left: Point, size: Size) -> Rectangle {
        Rectangle::new(
            top_left,
            top_left + Point::new(
                (size.width - 1) as i32,
                (size.height - 1) as i32,
            ),
        )
    }

    fn size(&self) -> Size {
        // TODO: remove if fixed in embedded-graphics
        let top_left = self.top_left;
        let bottom_right = self.bottom_right;

        let width = (top_left.x - bottom_right.x).abs() as u32 + 1;
        let height = (top_left.y - bottom_right.y).abs() as u32 + 1;

        Size::new(width, height)
    }

    fn center_x(&self) -> i32 {
        (self.top_left.x + self.bottom_right.x) / 2
    }

    fn center_y(&self) -> i32 {
        (self.top_left.y + self.bottom_right.y) / 2
    }

    fn center(&self) -> Point {
        Point::new(self.center_x(), self.center_y())
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use embedded_graphics::{prelude::*, primitives::Rectangle};

    #[test]
    fn test_sized() {
        let rect0 = Rectangle::with_size(Point::new(-1, -1), Size::new(3, 3));

        assert_eq!(
            Point::new(1, 1),
            rect0.bottom_right
        );
    }
}
