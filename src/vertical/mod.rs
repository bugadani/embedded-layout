//! Vertical alignment options
use crate::VerticalAlignment;
use crate::View;

/// Keep the object's vertical coordinate unchanged
#[derive(Copy, Clone)]
pub struct NoAlignment;

impl VerticalAlignment for NoAlignment {
    fn align(&self, _object: &impl View, _reference: &impl View) -> i32 {
        0
    }
}

/// Center the objects vertically
///
/// *Note:* in certain cases it's not possible to center objects perfectly because of
///         the integer cordinates used.
#[derive(Copy, Clone)]
pub struct Center;

impl VerticalAlignment for Center {
    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        let center_object = (object.top_left().y + object.bottom_right().y) / 2;
        let center_ref = (reference.top_left().y + reference.bottom_right().y) / 2;

        center_ref - center_object
    }
}

/// Align the top edge of the object to the top edge of the reference
#[derive(Copy, Clone)]
pub struct Top;

impl VerticalAlignment for Top {
    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        reference.top_left().y - object.top_left().y
    }
}

/// Align the bottom edge of the object to the bottom edge of the reference
#[derive(Copy, Clone)]
pub struct Bottom;

impl VerticalAlignment for Bottom {
    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        reference.bottom_right().y - object.bottom_right().y
    }
}

/// Align the top edge of the object to the bottom edge of the reference, non-overlapping
#[derive(Copy, Clone)]
pub struct TopToBottom;

impl VerticalAlignment for TopToBottom {
    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        (reference.bottom_right().y + 1) - object.top_left().y
    }
}

/// Align the bottom edge of the object to the top edge of the reference, non-overlapping
#[derive(Copy, Clone)]
pub struct BottomToTop;

impl VerticalAlignment for BottomToTop {
    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        (reference.top_left().y - 1) - object.bottom_right().y
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use embedded_graphics::{geometry::Point, primitives::Rectangle};

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

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

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
            assert_eq!(result.size(), source.size());

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
            assert_eq!(result.size(), source.size());

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
            assert_eq!(result.size(), source.size());

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
