use core::ops::{Deref, DerefMut};

use embedded_graphics::{
    drawable::Drawable, pixelcolor::PixelColor, prelude::Point, primitives::Rectangle, DrawTarget,
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
    fn len(&self) -> usize {
        self.views.len()
    }

    fn at(&self, idx: usize) -> &dyn View {
        &self.views[idx]
    }

    fn at_mut(&mut self, idx: usize) -> &mut dyn View {
        &mut self.views[idx]
    }
}

impl<T> View for Views<'_, T>
where
    T: View,
{
    fn translate_impl(&mut self, by: Point) {
        ViewGroupHelper::translate(self, by)
    }

    fn bounds(&self) -> Rectangle {
        ViewGroupHelper::bounds(self)
    }
}

impl<'a, T> Deref for Views<'a, T>
where
    T: View,
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.views
    }
}

impl<'a, T> DerefMut for Views<'a, T>
where
    T: View,
{
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.views
    }
}

impl<'a, C, T> Drawable<C> for &'a Views<'_, T>
where
    C: PixelColor,
    T: View,
    &'a T: Drawable<C>,
{
    #[inline]
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
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
