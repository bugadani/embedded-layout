//! Bounding box cache wrapper
use crate::prelude::*;
use embedded_graphics::{primitives::Rectangle, DrawTarget};

/// Cache the bounding box of the wrapped [`View`]
pub struct Cached<V: View> {
    bounds: Rectangle,
    view: V,
}

impl<V: View> Cached<V> {
    /// Calculate a View's bounding box and store it for reuse
    #[inline]
    pub fn new(v: V) -> Self {
        let bounds = v.bounds();

        Self { view: v, bounds }
    }

    /// Unwrap the inner View
    #[inline]
    pub fn into_inner(self) -> V {
        self.view
    }
}

impl<V: View> View for Cached<V> {
    #[inline]
    fn translate(&mut self, by: Point) {
        self.bounds.translate(by);
        self.view.translate(by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}

impl<'a, V, C> Drawable<C> for &'a Cached<V>
where
    C: PixelColor,
    V: View,
    &'a V: Drawable<C>,
{
    #[inline]
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        self.view.draw(display)
    }
}
