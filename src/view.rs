//! `View` trait
//!
//! `View` is a basic trait that represents something that can be displayed.
use embedded_graphics::{
    prelude::*,
    primitives::Rectangle,
};
use crate::rect_helper::RectExt;

/// A view is the base unit for most of the `embedded-layout` operations.
///
/// Views must have a size and a position, so they need to implement the `Dimensions` and
/// `Transform` traits.
pub trait View {
    /// Get the size of a View.
    fn size(&self) -> Size;
    fn translate(&mut self, by: Point) -> &mut Self;
    fn bounds(&self) -> Rectangle;
}

impl<T> View for T where T: Transform + Dimensions {
    fn size(&self) -> Size {
        let bounds = self.bounds();
        RectExt::size(&bounds)
    }

    fn translate(&mut self, by: Point) -> &mut Self {
        self.translate_mut(by)
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(self.top_left(), self.bottom_right())
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use embedded_graphics::{
        geometry::{Point, Size},
        primitives::Rectangle,
    };

    #[test]
    fn test_size() {
        let rect = Rectangle::new(Point::zero(), Point::new(1, 2));

        assert_eq!(RectExt::size(&rect), Size::new(2, 3));
    }
}
