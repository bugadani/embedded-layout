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
    fn enveloping(&self, other: &Rectangle) -> Rectangle;
}

impl RectExt for Rectangle {
    fn with_size(top_left: Point, size: Size) -> Rectangle {
        Rectangle::new(
            top_left,
            top_left + Point::new((size.width - 1) as i32, (size.height - 1) as i32),
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

    fn enveloping(&self, other: &Rectangle) -> Rectangle {
        Rectangle::new(
            Point::new(
                self.top_left.x.min(other.top_left.x),
                self.top_left.y.min(other.top_left.y),
            ),
            Point::new(
                self.bottom_right.x.max(other.bottom_right.x),
                self.bottom_right.y.max(other.bottom_right.y),
            ),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use embedded_graphics::{prelude::*, primitives::Rectangle};

    #[test]
    fn test_sized() {
        let rect0 = Rectangle::with_size(Point::new(-1, -1), Size::new(3, 3));

        assert_eq!(Point::new(1, 1), rect0.bottom_right);
    }

    #[test]
    fn test_enveloping() {
        let rect0 = Rectangle::new(Point::new(-1, -1), Point::new(0, 0));
        let rect1 = Rectangle::new(Point::zero(), Point::new(1, 1));
        let rect2 = Rectangle::new(Point::zero(), Point::new(2, 2));

        assert_eq!(rect2, rect2.enveloping(&rect1));
        assert_eq!(rect2, rect1.enveloping(&rect2));
        assert_eq!(
            Rectangle::new(Point::new(-1, -1), Point::new(2, 2),),
            rect0.enveloping(&rect2)
        );
    }
}
