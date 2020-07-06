use crate::prelude::*;
use embedded_graphics::{primitives::Rectangle, DrawTarget};

/// Helper trait to retrieve display area as a `Rectangle`.
pub trait DisplayArea<C>
where
    C: PixelColor,
{
    /// Return the display area as a `Rectangle`
    ///
    /// This method is provided mainly to make it simpler to align to edges of the display.
    fn display_area(&self) -> Rectangle;
}

impl<C, T> DisplayArea<C> for T
where
    C: PixelColor,
    T: DrawTarget<C>,
{
    #[inline]
    fn display_area(&self) -> Rectangle {
        Rectangle::with_size(Point::zero(), self.size())
    }
}
