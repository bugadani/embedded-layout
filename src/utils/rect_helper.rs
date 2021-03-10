//! `Rectangle` utility methods
//!
//! This module implements a few useful extensions to `Rectangle`.
use embedded_graphics::{prelude::Point, primitives::Rectangle};

/// The trait that describes the extension methods.
pub trait RectExt: Sized {
    /// Return the horizontal center coordinate
    ///
    /// *Note:* when an object's width is an even number, the returned center point will not
    ///         be perfectly in the middle.
    fn center_x(self) -> i32;

    /// Return the vertical center coordinate
    ///
    /// *Note:* when an object's height is an even number, the returned center point will not
    ///         be perfectly in the middle.
    fn center_y(self) -> i32;

    /// Return the center point
    /// *Note:* when an object's width or height is an even number, the returned center point will
    ///         not be perfectly in the middle.
    fn center(self) -> Point;

    /// Return the bounding `Rectangle` that encompasses both `Rectangles`
    fn enveloping(self, other: &Rectangle) -> Rectangle;
}

impl RectExt for Rectangle {
    #[inline]
    fn center_x(self) -> i32 {
        (self.top_left.x + self.bottom_right().map(|p| p.x).unwrap_or(self.top_left.x)) / 2
    }

    #[inline]
    fn center_y(self) -> i32 {
        (self.top_left.y + self.bottom_right().map(|p| p.y).unwrap_or(self.top_left.y)) / 2
    }

    #[inline]
    fn center(self) -> Point {
        Point::new(self.center_x(), self.center_y())
    }

    #[inline]
    fn enveloping(self, other: &Rectangle) -> Rectangle {
        let self_bottom_right = if self.is_zero_sized() {
            if other.is_zero_sized() {
                return Rectangle::zero();
            } else {
                other.bottom_right().unwrap()
            }
        } else {
            self.bottom_right().unwrap()
        };
        let other_bottom_right = if other.is_zero_sized() {
            if self.is_zero_sized() {
                return Rectangle::zero();
            } else {
                self.bottom_right().unwrap()
            }
        } else {
            other.bottom_right().unwrap()
        };

        Self::with_corners(
            Point::new(
                self.top_left.x.min(other.top_left.x),
                self.top_left.y.min(other.top_left.y),
            ),
            Point::new(
                self_bottom_right.x.max(other_bottom_right.x),
                self_bottom_right.y.max(other_bottom_right.y),
            ),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use embedded_graphics::{
        prelude::{Point, Size},
        primitives::Rectangle,
    };

    #[test]
    fn test_enveloping() {
        let rect0 = Rectangle::new(Point::new(-1, -1), Size::new(0, 0));
        let rect1 = Rectangle::new(Point::zero(), Size::new(1, 1));
        let rect2 = Rectangle::new(Point::zero(), Size::new(2, 2));

        assert_eq!(rect2, rect2.enveloping(&rect1));
        assert_eq!(rect2, rect1.enveloping(&rect2));
        assert_eq!(
            Rectangle::with_corners(Point::new(-1, -1), Point::new(2, 2),),
            rect0.enveloping(&rect2)
        );
    }
}
