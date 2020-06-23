use crate::HorizontalAlignment;
use embedded_graphics::geometry::Dimensions;

pub struct NoAlignment;
pub struct Center;
pub struct Left;
pub struct Right;

impl HorizontalAlignment for NoAlignment {
    fn align(&self, _what: &impl Dimensions, _reference: &impl Dimensions) -> i32 {
        0
    }
}

/// Calculate the difference of the center points, used to align the objects horizontally
impl HorizontalAlignment for Center {
    fn align(&self, what: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        let center_what = (what.top_left().x + what.bottom_right().x) / 2;
        let center_ref = (reference.top_left().x + reference.bottom_right().x) / 2;

        center_ref - center_what
    }
}

impl HorizontalAlignment for Left {
    fn align(&self, what: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        reference.top_left().x - what.top_left().x
    }
}

impl HorizontalAlignment for Right {
    fn align(&self, what: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        reference.bottom_right().x - what.bottom_right().x
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vertical::NoAlignment;
    use crate::Align;
    use embedded_graphics::geometry::Point;
    use embedded_graphics::primitives::Rectangle;

    #[test]
    fn test_center() {
        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(5, 20), Point::new(25, 50));

        let result = rect1.align_to(rect2, Center, NoAlignment);
        assert_eq!(
            result,
            Rectangle::new(Point::new(10, 0), Point::new(20, 10))
        );

        let result = rect2.align_to(rect1, Center, NoAlignment);
        assert_eq!(
            result,
            Rectangle::new(Point::new(-5, 20), Point::new(15, 50))
        );
    }

    #[test]
    fn test_left() {
        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(5, 20), Point::new(25, 50));

        let result = rect1.align_to(rect2, Left, NoAlignment);
        assert_eq!(result, Rectangle::new(Point::new(5, 0), Point::new(15, 10)));

        let result = rect2.align_to(rect1, Left, NoAlignment);
        assert_eq!(
            result,
            Rectangle::new(Point::new(0, 20), Point::new(20, 50))
        );
    }

    #[test]
    fn test_right() {
        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(5, 20), Point::new(25, 50));

        let result = rect1.align_to(rect2, Right, NoAlignment);
        assert_eq!(
            result,
            Rectangle::new(Point::new(15, 0), Point::new(25, 10))
        );

        let result = rect2.align_to(rect1, Right, NoAlignment);
        assert_eq!(
            result,
            Rectangle::new(Point::new(-10, 20), Point::new(10, 50))
        );
    }
}
