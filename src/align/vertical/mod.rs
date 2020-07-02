//! Vertical alignment options
use crate::{
    align::{Alignment, VerticalAlignment},
    prelude::*,
};

/// Keep the object's vertical coordinate unchanged
#[derive(Copy, Clone)]
pub struct NoAlignment;
impl VerticalAlignment for NoAlignment {}

impl Alignment for NoAlignment {
    fn new() -> Self {
        Self
    }

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
impl VerticalAlignment for Center {}

impl Alignment for Center {
    fn new() -> Self {
        Self
    }

    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        reference.bounds().center_y() - object.bounds().center_y()
    }
}

/// Align the top edge of the object to the top edge of the reference
#[derive(Copy, Clone)]
pub struct Top;
impl VerticalAlignment for Top {}

impl Alignment for Top {
    fn new() -> Self {
        Self
    }

    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        reference.bounds().top_left.y - object.bounds().top_left.y
    }
}

/// Align the bottom edge of the object to the bottom edge of the reference
#[derive(Copy, Clone)]
pub struct Bottom;
impl VerticalAlignment for Bottom {}

impl Alignment for Bottom {
    fn new() -> Self {
        Self
    }

    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        reference.bounds().bottom_right.y - object.bounds().bottom_right.y
    }
}

/// Align the top edge of the object to the bottom edge of the reference, non-overlapping
#[derive(Copy, Clone)]
pub struct TopToBottom;
impl VerticalAlignment for TopToBottom {}

impl Alignment for TopToBottom {
    fn new() -> Self {
        Self
    }
    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        (reference.bounds().bottom_right.y + 1) - object.bounds().top_left.y
    }
}

/// Align the bottom edge of the object to the top edge of the reference, non-overlapping
#[derive(Copy, Clone)]
pub struct BottomToTop;
impl VerticalAlignment for BottomToTop {}

impl Alignment for BottomToTop {
    fn new() -> Self {
        Self
    }

    fn align(&self, object: &impl View, reference: &impl View) -> i32 {
        (reference.bounds().top_left.y - 1) - object.bounds().bottom_right.y
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
            assert_eq!(RectExt::size(&result), RectExt::size(&source));

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
            assert_eq!(RectExt::size(&result), RectExt::size(&source));

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
            assert_eq!(RectExt::size(&result), RectExt::size(&source));

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
            assert_eq!(RectExt::size(&result), RectExt::size(&source));

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
