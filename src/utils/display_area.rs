use embedded_graphics::{prelude::*, primitives::Rectangle};

use crate::utils::rect_helper::RectExt;

/// Helper trait to retrieve display area as a `Rectangle`.
pub trait DisplayArea<C>
where
    C: PixelColor,
{
    fn display_area(&self) -> Rectangle;
}

impl<C, T> DisplayArea<C> for T
where
    C: PixelColor,
    T: DrawTarget<C>,
{
    fn display_area(&self) -> Rectangle {
        Rectangle::with_size(Point::new(0, 0), self.size())
    }
}
