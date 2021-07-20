//! `Rectangle` utility methods
//!
//! This module implements a few useful extensions to `Rectangle`.
use embedded_graphics::{geometry::AnchorPoint, prelude::*, primitives::Rectangle};

/// The trait that describes the extension methods.
pub trait RectExt {
    /// Return the bounding `Rectangle` that encompasses both `Rectangles`
    fn enveloping(&self, other: &Rectangle) -> Rectangle;
}

impl RectExt for Rectangle {
    #[inline]
    fn enveloping(&self, other: &Rectangle) -> Rectangle {
        Rectangle::with_corners(
            Point::new(
                self.top_left.x.min(other.top_left.x),
                self.top_left.y.min(other.top_left.y),
            ),
            Point::new(
                self.anchor_point(AnchorPoint::BottomRight)
                    .x
                    .max(other.anchor_point(AnchorPoint::BottomRight).x),
                self.anchor_point(AnchorPoint::BottomRight)
                    .y
                    .max(other.anchor_point(AnchorPoint::BottomRight).y),
            ),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use embedded_graphics::{prelude::*, primitives::Rectangle};

    #[test]
    fn test_enveloping() {
        let rect0 = Rectangle::with_corners(Point::new(-1, -1), Point::new(0, 0));
        let rect1 = Rectangle::with_corners(Point::zero(), Point::new(1, 1));
        let rect2 = Rectangle::with_corners(Point::zero(), Point::new(2, 2));

        assert_eq!(rect2, rect2.enveloping(&rect1));
        assert_eq!(rect2, rect1.enveloping(&rect2));
        assert_eq!(
            Rectangle::with_corners(Point::new(-1, -1), Point::new(2, 2),),
            rect0.enveloping(&rect2)
        );
    }
}
