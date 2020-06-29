//! Horizontal alignment options
use crate::HorizontalAlignment;
use crate::View;
use crate::rect_helper::RectExt;

/// Keep the object's horizontal coordinate unchanged
#[derive(Copy, Clone)]
pub struct NoAlignment;

impl HorizontalAlignment for NoAlignment {
    fn align(&self, _object: &impl View, _reference: &impl View) -> i32 {
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
    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        reference.bounds().center_x() - object.bounds().center_x()
    }
}

/// Align the left edge of the object to the left edge of the reference
#[derive(Copy, Clone)]
pub struct Left;

impl HorizontalAlignment for Left {
    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        reference.bounds().top_left.x - object.bounds().top_left.x
    }
}

/// Align the right edge of the object to the right edge of the reference
#[derive(Copy, Clone)]
pub struct Right;

impl HorizontalAlignment for Right {
    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        reference.bounds().bottom_right.x - object.bounds().bottom_right.x
    }
}

/// Align the left edge of the object to the right edge of the reference, non-overlapping
#[derive(Copy, Clone)]
pub struct LeftToRight;

impl HorizontalAlignment for LeftToRight {
    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        (reference.bounds().bottom_right.x + 1) - object.bounds().top_left.x
    }
}

/// Align the right edge of the object to the left edge of the reference, non-overlapping
#[derive(Copy, Clone)]
pub struct RightToLeft;

/// Align the bottom edge of the object to the top edge of the reference, non-overlapping
impl HorizontalAlignment for RightToLeft {
    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        (reference.bounds().top_left.x - 1) - object.bounds().bottom_right.x
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use embedded_graphics::{geometry::Point, primitives::Rectangle};

    #[test]
    fn test_center() {
        fn check_center_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            let center_of_reference = reference.top_left + RectExt::size(&reference) / 2;
            let center_of_result = result.top_left + RectExt::size(&result) / 2;

            // The size hasn't changed
            assert_eq!(RectExt::size(&result), RectExt::size(&source));

            // Horizontal coordinate matches reference
            assert_eq!(center_of_result.x, center_of_reference.x);

            // Vertical coordinate is unchanged
            assert_eq!(result.top_left.y, source.top_left.y);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let mut source = rect1;
        source.align_to(&rect2, horizontal::Center, vertical::NoAlignment);
        check_center_alignment(rect1, rect2, source);

        // Test the other direction
        let mut source = rect2;
        source.align_to(&rect1, horizontal::Center, vertical::NoAlignment);
        check_center_alignment(rect2, rect1, source);
    }

    #[test]
    fn test_left() {
        fn check_left_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            // The size hasn't changed
            assert_eq!(RectExt::size(&result), RectExt::size(&source));

            // Horizontal coordinate matches reference
            assert_eq!(result.top_left.x, reference.top_left.x);

            // Vertical coordinate is unchanged
            assert_eq!(result.top_left.y, source.top_left.y);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let mut source = rect1;
        source.align_to(&rect2, horizontal::Left, vertical::NoAlignment);
        check_left_alignment(rect1, rect2, source);

        // Test the other direction
        let mut source = rect2;
        source.align_to(&rect1, horizontal::Left, vertical::NoAlignment);
        check_left_alignment(rect2, rect1, source);
    }

    #[test]
    fn test_right() {
        fn check_right_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            // The size hasn't changed
            assert_eq!(RectExt::size(&result), RectExt::size(&source));

            // Horizontal coordinate matches reference
            assert_eq!(result.bottom_right.x, reference.bottom_right.x);

            // Vertical coordinate is unchanged
            assert_eq!(result.bottom_right.y, source.bottom_right.y);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let mut source = rect1;
        source.align_to(&rect2, horizontal::Right, vertical::NoAlignment);
        check_right_alignment(rect1, rect2, source);

        // Test the other direction
        let mut source = rect2;
        source.align_to(&rect1, horizontal::Right, vertical::NoAlignment);
        check_right_alignment(rect2, rect1, source);
    }

    #[test]
    fn test_left_to_right() {
        fn check_left_to_right_alignment(
            source: Rectangle,
            reference: Rectangle,
            result: Rectangle,
        ) {
            // The size hasn't changed
            assert_eq!(RectExt::size(&result), RectExt::size(&source));

            // Left is at right + 1
            assert_eq!(result.top_left.x, reference.bottom_right.x + 1);

            // Vertical coordinate is unchanged
            assert_eq!(result.bottom_right.y, source.bottom_right.y);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let mut source = rect1;
        source.align_to(&rect2, horizontal::LeftToRight, vertical::NoAlignment);
        check_left_to_right_alignment(rect1, rect2, source);

        // Test the other direction
        let mut source = rect2;
        source.align_to(&rect1, horizontal::LeftToRight, vertical::NoAlignment);
        check_left_to_right_alignment(rect2, rect1, source);
    }

    #[test]
    fn test_right_to_left() {
        fn check_right_to_left_alignment(
            source: Rectangle,
            reference: Rectangle,
            result: Rectangle,
        ) {
            // The size hasn't changed
            assert_eq!(RectExt::size(&result), RectExt::size(&source));

            // Left is at right + 1
            assert_eq!(result.bottom_right.x, reference.top_left.x - 1);

            // Vertical coordinate is unchanged
            assert_eq!(result.bottom_right.y, source.bottom_right.y);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let mut source = rect1;
        source.align_to(&rect2, horizontal::RightToLeft, vertical::NoAlignment);
        check_right_to_left_alignment(rect1, rect2, source);

        // Test the other direction
        let mut source = rect2;
        source.align_to(&rect1, horizontal::RightToLeft, vertical::NoAlignment);
        check_right_to_left_alignment(rect2, rect1, source);
    }
}
