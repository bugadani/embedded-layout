//! Vertical alignment options
//!
//! Vertical alignment types must implement [`VerticalAlignment`].
use crate::align::{Alignment, VerticalAlignment};
use embedded_graphics::{geometry::AnchorPoint, primitives::Rectangle};

/// Keep the objects' vertical alignment unchanged
#[derive(Copy, Clone, Default)]
pub struct NoAlignment;
impl VerticalAlignment for NoAlignment {}

impl Alignment for NoAlignment {
    #[inline]
    fn align_with_offset(&self, _object: Rectangle, _reference: Rectangle, _offset: i32) -> i32 {
        0
    }
}

/// Center the objects vertically
///
/// *Note:* in certain cases it's not possible to center objects perfectly because of
///         the integer cordinates used.
#[derive(Copy, Clone, Default)]
pub struct Center;
impl VerticalAlignment for Center {}

impl Alignment for Center {
    #[inline]
    fn align_with_offset(&self, object: Rectangle, reference: Rectangle, offset: i32) -> i32 {
        reference.anchor_point(AnchorPoint::Center).y - object.anchor_point(AnchorPoint::Center).y
            + offset
    }
}

/// Align the top edge of the object to the top edge of the reference
#[derive(Copy, Clone, Default)]
pub struct Top;
impl VerticalAlignment for Top {}

impl Alignment for Top {
    #[inline]
    fn align_with_offset(&self, object: Rectangle, reference: Rectangle, offset: i32) -> i32 {
        reference.top_left.y - object.top_left.y + offset
    }
}

/// Align the bottom edge of the object to the bottom edge of the reference
#[derive(Copy, Clone, Default)]
pub struct Bottom;
impl VerticalAlignment for Bottom {}

impl Alignment for Bottom {
    #[inline]
    fn align_with_offset(&self, object: Rectangle, reference: Rectangle, offset: i32) -> i32 {
        reference.anchor_point(AnchorPoint::BottomRight).y
            - object.anchor_point(AnchorPoint::BottomRight).y
            + offset
    }
}

/// Align the top edge of the object to the bottom edge of the reference, non-overlapping
#[derive(Copy, Clone, Default)]
pub struct TopToBottom;
impl VerticalAlignment for TopToBottom {}

impl Alignment for TopToBottom {
    #[inline]
    fn align_with_offset(&self, object: Rectangle, reference: Rectangle, offset: i32) -> i32 {
        let offset = if object.size.height == 0 {
            offset
        } else {
            offset + 1
        };
        reference.anchor_point(AnchorPoint::BottomRight).y - object.top_left.y + offset
    }
}

/// Align the bottom edge of the object to the top edge of the reference, non-overlapping
#[derive(Copy, Clone, Default)]
pub struct BottomToTop;
impl VerticalAlignment for BottomToTop {}

impl Alignment for BottomToTop {
    #[inline]
    fn align_with_offset(&self, object: Rectangle, reference: Rectangle, offset: i32) -> i32 {
        let offset = if object.size.height == 0 {
            offset
        } else {
            offset - 1
        };
        reference.top_left.y - object.anchor_point(AnchorPoint::BottomRight).y + offset
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use embedded_graphics::{
        geometry::{AnchorPoint, Point},
        prelude::Size,
        primitives::Rectangle,
    };

    #[test]
    fn test_center() {
        fn check_center_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            let center_of_reference = reference.top_left + reference.size() / 2;
            let center_of_result = result.top_left + result.size() / 2;

            // The size hasn't changed
            assert_eq!(result.size(), source.size());

            // Vertical coordinate matches reference
            assert_eq!(center_of_result.y, center_of_reference.y);

            // Horizontal coordinate is unchanged
            assert_eq!(result.top_left.x, source.top_left.x);
        }

        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::NoAlignment, vertical::Center);
        check_center_alignment(rect1, rect2, result);

        // Test the other direction

        let result = rect2.align_to(&rect1, horizontal::NoAlignment, vertical::Center);
        check_center_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_top() {
        fn check_top_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            // The size hasn't changed
            assert_eq!(result.size(), source.size());

            // Vertical coordinate matches reference
            assert_eq!(result.top_left.y, reference.top_left.y);

            // Horizontal coordinate is unchanged
            assert_eq!(result.top_left.x, source.top_left.x);
        }

        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::NoAlignment, vertical::Top);
        check_top_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::NoAlignment, vertical::Top);
        check_top_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_bottom() {
        fn check_bottom_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            // The size hasn't changed
            assert_eq!(result.size(), source.size());

            // Vertical coordinate matches reference
            assert_eq!(
                result.anchor_point(AnchorPoint::BottomRight).y,
                reference.anchor_point(AnchorPoint::BottomRight).y
            );

            // Horizontal coordinate is unchanged
            assert_eq!(
                result.anchor_point(AnchorPoint::BottomRight).x,
                source.anchor_point(AnchorPoint::BottomRight).x
            );
        }

        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::NoAlignment, vertical::Bottom);
        check_bottom_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::NoAlignment, vertical::Bottom);
        check_bottom_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_top_to_bottom() {
        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::NoAlignment, vertical::TopToBottom);
        // The size hasn't changed
        assert_eq!(result.size(), rect1.size());

        // Top is at bottom + 1
        assert_eq!(
            result.top_left.y,
            rect2.anchor_point(AnchorPoint::BottomRight).y + 1
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect1.anchor_point(AnchorPoint::BottomRight).x
        );

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::NoAlignment, vertical::TopToBottom);
        // The size hasn't changed
        assert_eq!(result.size(), rect2.size());

        // Top is at bottom + 1
        assert_eq!(
            result.top_left.y,
            rect1.anchor_point(AnchorPoint::BottomRight).y + 1
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect2.anchor_point(AnchorPoint::BottomRight).x
        );
    }

    #[test]
    fn test_top_to_bottom_empty() {
        let rect1 = Rectangle::new(Point::new(0, 0), Size::zero());
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::NoAlignment, vertical::TopToBottom);
        // The size hasn't changed
        assert_eq!(result.size(), rect1.size());

        // Top is at bottom
        assert_eq!(
            result.top_left.y,
            rect2.anchor_point(AnchorPoint::BottomRight).y
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect1.anchor_point(AnchorPoint::BottomRight).x
        );

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::NoAlignment, vertical::TopToBottom);
        // The size hasn't changed
        assert_eq!(result.size(), rect2.size());

        // Top is at bottom + 1
        assert_eq!(
            result.top_left.y,
            rect1.anchor_point(AnchorPoint::BottomRight).y + 1
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect2.anchor_point(AnchorPoint::BottomRight).x
        );
    }

    #[test]
    fn test_bottom_to_top() {
        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::NoAlignment, vertical::BottomToTop);
        // The size hasn't changed
        assert_eq!(result.size(), rect1.size());

        // Bottom is at top - 1
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect2.top_left.y - 1
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect1.anchor_point(AnchorPoint::BottomRight).x
        );

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::NoAlignment, vertical::BottomToTop);
        // The size hasn't changed
        assert_eq!(result.size(), rect2.size());

        // Bottom is at top - 1
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect1.top_left.y - 1
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect2.anchor_point(AnchorPoint::BottomRight).x
        );
    }

    #[test]
    fn test_bottom_to_top_empty() {
        let rect1 = Rectangle::new(Point::new(0, 0), Size::zero());
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::NoAlignment, vertical::BottomToTop);
        // The size hasn't changed
        assert_eq!(result.size(), rect1.size());

        // Bottom is at top
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect2.top_left.y
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect1.anchor_point(AnchorPoint::BottomRight).x
        );

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::NoAlignment, vertical::BottomToTop);
        // The size hasn't changed
        assert_eq!(result.size(), rect2.size());

        // Bottom is at top - 1
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect1.top_left.y - 1
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect2.anchor_point(AnchorPoint::BottomRight).x
        );
    }
}
