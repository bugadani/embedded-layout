//! Vertical alignment options
//!
//! Vertical alignment types must implement [`VerticalAlignment`].
use crate::{
    align::{Alignment, VerticalAlignment},
    utils::rect_helper::RectExt,
};
use embedded_graphics::primitives::Rectangle;

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
///         the integer coordinates used.
#[derive(Copy, Clone, Default)]
pub struct Center;
impl VerticalAlignment for Center {}

impl Alignment for Center {
    #[inline]
    fn align_with_offset(&self, object: Rectangle, reference: Rectangle, offset: i32) -> i32 {
        reference.center_y() - object.center_y() + offset
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
        reference.bottom_right.y - object.bottom_right.y + offset
    }
}

/// Align the top edge of the object to the bottom edge of the reference, non-overlapping
#[derive(Copy, Clone, Default)]
pub struct TopToBottom;
impl VerticalAlignment for TopToBottom {}

impl Alignment for TopToBottom {
    #[inline]
    fn align_with_offset(&self, object: Rectangle, reference: Rectangle, offset: i32) -> i32 {
        (reference.bottom_right.y + 1) - object.top_left.y + offset
    }
}

/// Align the bottom edge of the object to the top edge of the reference, non-overlapping
#[derive(Copy, Clone, Default)]
pub struct BottomToTop;
impl VerticalAlignment for BottomToTop {}

impl Alignment for BottomToTop {
    #[inline]
    fn align_with_offset(&self, object: Rectangle, reference: Rectangle, offset: i32) -> i32 {
        (reference.top_left.y - 1) - object.bottom_right.y + offset
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use embedded_graphics::{geometry::Point, prelude::Size, primitives::Rectangle};

    #[test]
    fn test_center() {
        fn check_center_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            let center_of_reference = reference.top_left + reference.bounds().size / 2;
            let center_of_result = result.top_left + result.bounds().size / 2;

            // The size hasn't changed
            assert_eq!(result.bounds().size, source.bounds().size);

            // Vertical coordinate matches reference
            assert_eq!(center_of_result.y, center_of_reference.y);

            // Horizontal coordinate is unchanged
            assert_eq!(result.top_left.x, source.top_left.x);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Size::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Size::new(40, 50));

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
            assert_eq!(result.bounds().size, source.bounds().size);

            // Vertical coordinate matches reference
            assert_eq!(result.top_left.y, reference.top_left.y);

            // Horizontal coordinate is unchanged
            assert_eq!(result.top_left.x, source.top_left.x);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

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
            assert_eq!(result.bounds().size, source.bounds().size);

            // Vertical coordinate matches reference
            assert_eq!(result.bottom_right.y, reference.bottom_right.y);

            // Horizontal coordinate is unchanged
            assert_eq!(result.bottom_right.x, source.bottom_right.x);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::NoAlignment, vertical::Bottom);
        check_bottom_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::NoAlignment, vertical::Bottom);
        check_bottom_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_top_to_bottom() {
        fn check_to_to_bottom_alignment(
            source: Rectangle,
            reference: Rectangle,
            result: Rectangle,
        ) {
            // The size hasn't changed
            assert_eq!(result.bounds().size, source.bounds().size);

            // Top is at bottom + 1
            assert_eq!(result.top_left.y, reference.bottom_right.y + 1);

            // Horizontal coordinate is unchanged
            assert_eq!(result.bottom_right.x, source.bottom_right.x);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::NoAlignment, vertical::TopToBottom);
        check_to_to_bottom_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::NoAlignment, vertical::TopToBottom);
        check_to_to_bottom_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_bottom_to_top() {
        fn check_to_to_bottom_alignment(
            source: Rectangle,
            reference: Rectangle,
            result: Rectangle,
        ) {
            // The size hasn't changed
            assert_eq!(result.bounds().size, source.bounds().size);

            // Bottom is at top - 1
            assert_eq!(result.bottom_right.y, reference.top_left.y - 1);

            // Horizontal coordinate is unchanged
            assert_eq!(result.bottom_right.x, source.bottom_right.x);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::NoAlignment, vertical::BottomToTop);
        check_to_to_bottom_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::NoAlignment, vertical::BottomToTop);
        check_to_to_bottom_alignment(rect2, rect1, result);
    }
}
