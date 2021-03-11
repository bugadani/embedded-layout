//! ViewGroup implementation for object chains.

use embedded_graphics::{
    drawable::Drawable, pixelcolor::PixelColor, prelude::Point, primitives::Rectangle, DrawTarget,
};

use crate::{
    layout::{Link, Tail},
    prelude::{ChainElement, RectExt},
    view_group::ViewGroup,
    View,
};

impl<'a, C, V, VC> Drawable<C> for &'a Link<V, VC>
where
    C: PixelColor,
    V: View,
    &'a V: Drawable<C>,
    VC: View + ChainElement,
    &'a VC: Drawable<C>,
{
    #[inline]
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        self.object.draw(display)?;
        self.next.draw(display)?;

        Ok(())
    }
}

impl<V, VC> View for Link<V, VC>
where
    V: View,
    VC: View + ChainElement,
{
    #[inline]
    fn bounds(&self) -> Rectangle {
        let bounds = self.object.bounds();

        bounds.enveloping(&self.next.bounds())
    }

    #[inline]
    fn translate_impl(&mut self, by: Point) {
        self.object.translate_mut(by);
        self.next.translate_mut(by);
    }
}

impl<'a, C, V> Drawable<C> for &'a Tail<V>
where
    C: PixelColor,
    V: View,
    &'a V: Drawable<C>,
{
    #[inline]
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        self.object.draw(display)
    }
}

impl<V> View for Tail<V>
where
    V: View,
{
    #[inline]
    fn bounds(&self) -> Rectangle {
        self.object.bounds()
    }

    #[inline]
    fn translate_impl(&mut self, by: Point) {
        self.object.translate_mut(by);
    }
}

impl<V, VC> ViewGroup for Link<V, VC>
where
    V: 'static + View,
    VC: 'static + ViewGroup + View + ChainElement,
{
    fn len(&self) -> usize {
        Link::count(self) as usize
    }

    fn at(&self, index: usize) -> &dyn View {
        if index == self.len() - 1 {
            return &self.object;
        }

        return self.next.at(index);
    }

    fn at_mut(&mut self, index: usize) -> &mut dyn View {
        if index == self.len() - 1 {
            return &mut self.object;
        }

        return self.next.at_mut(index);
    }
}

impl<V> ViewGroup for Tail<V>
where
    V: 'static + View,
{
    fn len(&self) -> usize {
        Tail::count(self) as usize
    }

    fn at(&self, index: usize) -> &dyn View {
        assert!(index == 0);

        return &self.object;
    }

    fn at_mut(&mut self, index: usize) -> &mut dyn View {
        assert!(index == 0);

        return &mut self.object;
    }
}
