use crate::HorizontalAlignment;
use embedded_graphics::geometry::Dimensions;

pub struct NoAlignment;
pub struct Center;
pub struct Left;
pub struct Right;

impl HorizontalAlignment for NoAlignment {
    fn align(&self, _object: &impl Dimensions, _reference: &impl Dimensions) -> i32 {
        0
    }
}

/// Calculate the difference of the center points, used to align the objects horizontally
impl HorizontalAlignment for Center {
    fn align(&self, object: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        let center_object = (object.top_left().x + object.bottom_right().x) / 2;
        let center_ref = (reference.top_left().x + reference.bottom_right().x) / 2;

        center_ref - center_object
    }
}

impl HorizontalAlignment for Left {
    fn align(&self, object: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        reference.top_left().x - object.top_left().x
    }
}

impl HorizontalAlignment for Right {
    fn align(&self, object: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        reference.bottom_right().x - object.bottom_right().x
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

        let result = rect1.align_to(rect2, horizontal::Center, vertical::NoAlignment);
        check_center_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(rect1, horizontal::Center, vertical::NoAlignment);
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

        let result = rect1.align_to(rect2, horizontal::Left, vertical::NoAlignment);
        check_left_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(rect1, horizontal::Left, vertical::NoAlignment);
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

        let result = rect1.align_to(rect2, horizontal::Right, vertical::NoAlignment);
        check_right_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(rect1, horizontal::Right, vertical::NoAlignment);
        check_right_alignment(rect2, rect1, result);
    }
}
