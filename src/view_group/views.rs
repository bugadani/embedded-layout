use core::ops::{Deref, DerefMut};

use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::PixelColor, prelude::Point, primitives::Rectangle,
    Drawable,
};

use crate::{
    view_group::{ViewGroup, ViewGroupHelper},
    View,
};

/// Wrapper that implements ViewGroup for a slice of views.
pub struct Views<'a, T>
where
    T: View,
{
    views: &'a mut [T],
}

impl<'a, T> Views<'a, T>
where
    T: View,
{
    /// Wraps the given slice.
    #[inline]
    pub fn new(views: &'a mut [T]) -> Self {
        Self { views }
    }
}

impl<T> ViewGroup for Views<'_, T>
where
    T: View,
{
    #[inline]
    fn len(&self) -> usize {
        self.views.len()
    }

    #[inline]
    fn at(&self, idx: usize) -> &dyn View {
        &self.views[idx]
    }

    #[inline]
    fn at_mut(&mut self, idx: usize) -> &mut dyn View {
        &mut self.views[idx]
    }
}

impl<T> View for Views<'_, T>
where
    T: View,
{
    #[inline]
    fn translate_impl(&mut self, by: Point) {
        ViewGroupHelper::translate(self, by)
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        ViewGroupHelper::bounds(self)
    }
}

impl<'a, T> Deref for Views<'a, T>
where
    T: View,
{
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.views
    }
}

impl<'a, T> DerefMut for Views<'a, T>
where
    T: View,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.views
    }
}

impl<C, T> Drawable for Views<'_, T>
where
    C: PixelColor,
    T: View,
    T: Drawable<Color = C>,
{
    type Color = C;
    type Output = ();

    #[inline]
    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        for view in self.views.iter() {
            view.draw(display)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use embedded_graphics::primitives::Line;

    #[test]
    fn len_is_slice_length() {
        let mut views = [
            Line::new(Point::zero(), Point::new(1, 2)),
            Line::new(Point::new(1, 2), Point::new(3, 1)),
            Line::new(Point::new(3, 1), Point::zero()),
        ];

        let vg = Views::new(&mut views);

        assert_eq!(3, vg.len());
    }

    #[test]
    fn views_behaves_as_slice() {
        let mut views = [
            Line::new(Point::zero(), Point::new(1, 2)),
            Line::new(Point::new(1, 2), Point::new(3, 1)),
            Line::new(Point::new(3, 1), Point::zero()),
        ];

        let vg = Views::new(&mut views);

        // deliberate count() because Views only exposes `iter()` through `Deref`.
        assert_eq!(1, vg[1..2].iter().count());
    }
}
