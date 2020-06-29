use embedded_graphics::{
    prelude::*,
    primitives::Rectangle,
};

pub trait RectExt {
    fn with_size(top_left: Point, size: Size) -> Rectangle;
    fn size(&self) -> Size;

    fn center_x(&self) -> i32;
    fn center_y(&self) -> i32;
    fn center(&self) -> Point;
}

impl RectExt for Rectangle {
    fn with_size(top_left: Point, size: Size) -> Rectangle {
        Rectangle::new(
            top_left,
            Point::new(
                (size.width - 1) as i32,
                (size.height - 1) as i32,
            ),
        )
    }

    fn size(&self) -> Size {
        // TODO: remove if fixed in embedded-graphics
        let top_left = self.top_left;
        let bottom_right = self.bottom_right;

        let width = (top_left.x - bottom_right.x).abs() as u32 + 1;
        let height = (top_left.y - bottom_right.y).abs() as u32 + 1;

        Size::new(width, height)
    }

    fn center_x(&self) -> i32 {
        (self.top_left.x + self.bottom_right.x) / 2
    }

    fn center_y(&self) -> i32 {
        (self.top_left.y + self.bottom_right.y) / 2
    }

    fn center(&self) -> Point {
        Point::new(
            self.center_x(),
            self.center_y(),
        )
    }
}
