//! Horizontal alignment options
use crate::HorizontalAlignment;
use embedded_graphics::geometry::Dimensions;

/// Keep the object's horizontal coordinate unchanged
#[derive(Copy, Clone)]
pub struct NoAlignment;

impl HorizontalAlignment for NoAlignment {
    fn align(&self, _object: &impl Dimensions, _reference: &impl Dimensions) -> i32 {
        0
    }
}

/// Center the objects horizontally
///
/// *Note:* in certain cases it's not possible to center objects perfectly because of
///         the integer cordinates used.
#[derive(Copy, Clone)]
pub struct Center;

impl HorizontalAlignment for Center {
    fn align(&self, object: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        let center_object = (object.top_left().x + object.bottom_right().x) / 2;
        let center_ref = (reference.top_left().x + reference.bottom_right().x) / 2;

        center_ref - center_object
    }
}

/// Align the left edge of the object to the left edge of the reference
#[derive(Copy, Clone)]
pub struct Left;

impl HorizontalAlignment for Left {
    fn align(&self, object: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        reference.top_left().x - object.top_left().x
    }
}

/// Align the right edge of the object to the right edge of the reference
#[derive(Copy, Clone)]
pub struct Right;

impl HorizontalAlignment for Right {
    fn align(&self, object: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        reference.bottom_right().x - object.bottom_right().x
    }
}

/// Align the left edge of the object to the right edge of the reference, non-overlapping
#[derive(Copy, Clone)]
pub struct LeftToRight;

impl HorizontalAlignment for LeftToRight {
    fn align(&self, object: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        (reference.bottom_right().x + 1) - object.top_left().x
    }
}

/// Align the right edge of the object to the left edge of the reference, non-overlapping
#[derive(Copy, Clone)]
pub struct RightToLeft;

/// Align the bottom edge of the object to the top edge of the reference, non-overlapping
impl HorizontalAlignment for RightToLeft {
    fn align(&self, object: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        (reference.top_left().x - 1) - object.bottom_right().x
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use embedded_graphics::geometry::{Dimensions, Point};
    use embedded_graphics::primitives::Rectangle;

    #[test]
    fn test_center() {
        fn check_center_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            let center_of_reference = reference.top_left() + reference.size() / 2;
            let center_of_result = result.top_left() + result.size() / 2;

            // The size hasn't changed
            assert_eq!(result.size(), source.size());

            // Horizontal coordinate matches reference
            assert_eq!(center_of_result.x, center_of_reference.x);

            // Vertical coordinate is unchanged
            assert_eq!(result.top_left.y, source.top_left.y);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::Center, vertical::NoAlignment);
        check_center_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::Center, vertical::NoAlignment);
        check_center_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_left() {
        fn check_left_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            // The size hasn't changed
            assert_eq!(result.size(), source.size());

            // Horizontal coordinate matches reference
            assert_eq!(result.top_left.x, reference.top_left.x);

            // Vertical coordinate is unchanged
            assert_eq!(result.top_left.y, source.top_left.y);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::Left, vertical::NoAlignment);
        check_left_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::Left, vertical::NoAlignment);
        check_left_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_right() {
        fn check_right_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            // The size hasn't changed
            assert_eq!(result.size(), source.size());

            // Horizontal coordinate matches reference
            assert_eq!(result.bottom_right.x, reference.bottom_right.x);

            // Vertical coordinate is unchanged
            assert_eq!(result.bottom_right.y, source.bottom_right.y);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::Right, vertical::NoAlignment);
        check_right_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::Right, vertical::NoAlignment);
        check_right_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_left_to_right() {
        fn check_left_to_right_alignment(
            source: Rectangle,
            reference: Rectangle,
            result: Rectangle,
        ) {
            // The size hasn't changed
            assert_eq!(result.size(), source.size());

            // Left is at right + 1
            assert_eq!(result.top_left.x, reference.bottom_right.x + 1);

            // Vertical coordinate is unchanged
            assert_eq!(result.bottom_right.y, source.bottom_right.y);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::LeftToRight, vertical::NoAlignment);
        check_left_to_right_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::LeftToRight, vertical::NoAlignment);
        check_left_to_right_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_right_to_left() {
        fn check_right_to_left_alignment(
            source: Rectangle,
            reference: Rectangle,
            result: Rectangle,
        ) {
            // The size hasn't changed
            assert_eq!(result.size(), source.size());

            // Left is at right + 1
            assert_eq!(result.bottom_right.x, reference.top_left.x - 1);

            // Vertical coordinate is unchanged
            assert_eq!(result.bottom_right.y, source.bottom_right.y);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, horizontal::RightToLeft, vertical::NoAlignment);
        check_right_to_left_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, horizontal::RightToLeft, vertical::NoAlignment);
        check_right_to_left_alignment(rect2, rect1, result);
    }
}
