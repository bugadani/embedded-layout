use crate::VerticalAlignment;
use embedded_graphics::geometry::Dimensions;

pub struct NoAlignment;
pub struct Center;
pub struct Top;
pub struct Bottom;

impl VerticalAlignment for NoAlignment {
    fn align(&self, _what: &impl Dimensions, _reference: &impl Dimensions) -> i32 {
        0
    }
}

/// Calculate the difference of the center points, used to align the objects vertically
impl VerticalAlignment for Center {
    fn align(&self, what: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        let center_what = (what.top_left().y + what.bottom_right().y) / 2;
        let center_ref = (reference.top_left().y + reference.bottom_right().y) / 2;

        center_ref - center_what
    }
}

impl VerticalAlignment for Top {
    fn align(&self, what: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        reference.top_left().y - what.top_left().y
    }
}

impl VerticalAlignment for Bottom {
    fn align(&self, what: &impl Dimensions, reference: &impl Dimensions) -> i32 {
        reference.bottom_right().y - what.bottom_right().y
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::horizontal::NoAlignment;
    use crate::Align;
    use embedded_graphics::geometry::Point;
    use embedded_graphics::primitives::Rectangle;

    #[test]
    fn test_center() {
        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(rect2, NoAlignment, Center);
        assert_eq!(
            result,
            Rectangle::new(Point::new(0, 30), Point::new(10, 40))
        );

        let result = rect2.align_to(rect1, NoAlignment, Center);
        assert_eq!(
            result,
            Rectangle::new(Point::new(30, -10), Point::new(40, 20))
        );
    }

    #[test]
    fn test_top() {
        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(rect2, NoAlignment, Top);
        assert_eq!(
            result,
            Rectangle::new(Point::new(0, 20), Point::new(10, 30))
        );

        let result = rect2.align_to(rect1, NoAlignment, Top);
        assert_eq!(
            result,
            Rectangle::new(Point::new(30, 0), Point::new(40, 30))
        );
    }

    #[test]
    fn test_bottom() {
        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(rect2, NoAlignment, Bottom);
        assert_eq!(
            result,
            Rectangle::new(Point::new(0, 40), Point::new(10, 50))
        );

        let result = rect2.align_to(rect1, NoAlignment, Bottom);
        assert_eq!(
            result,
            Rectangle::new(Point::new(30, -20), Point::new(40, 10))
        );
    }
}
