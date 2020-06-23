use crate::VerticalAlignment;
use embedded_graphics::geometry::Dimensions;

pub struct NoAlignment;
pub struct Center;
pub struct Top;
pub struct Bottom;

impl VerticalAlignment for NoAlignment {
    fn align(&self, _object: &impl Dimensions, _reference: &impl Dimensions) -> i32 {
        0
    }
}

/// Calculate the difference of the center points, used to align the objects vertically
impl VerticalAlignment for Center {
    fn align(&self, object: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        let center_object = (object.top_left().y + object.bottom_right().y) / 2;
        let center_ref = (reference.top_left().y + reference.bottom_right().y) / 2;

        center_ref - center_object
    }
}

impl VerticalAlignment for Top {
    fn align(&self, object: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        reference.top_left().y - object.top_left().y
    }
}

impl VerticalAlignment for Bottom {
    fn align(&self, object: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        reference.bottom_right().y - object.bottom_right().y
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

            // Vertical coordinate matches reference
            assert_eq!(center_of_result.y, center_of_reference.y);

            // Horizontal coordinate is unchanged
            assert_eq!(result.top_left.x, source.top_left.x);
        }

        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(rect2, horizontal::NoAlignment, vertical::Center);
        check_center_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(rect1, horizontal::NoAlignment, vertical::Center);
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

        let result = rect1.align_to(rect2, horizontal::NoAlignment, vertical::Top);
        check_top_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(rect1, horizontal::NoAlignment, vertical::Top);
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

        let result = rect1.align_to(rect2, horizontal::NoAlignment, vertical::Bottom);
        check_bottom_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(rect1, horizontal::NoAlignment, vertical::Bottom);
        check_bottom_alignment(rect2, rect1, result);
    }
}
