//! ViewGroup implementation for object chains.

use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::PixelColor, prelude::Point, primitives::Rectangle,
    Drawable,
};

use crate::{
    object_chain::{Chain, ChainElement, Link},
    prelude::RectExt,
    view_group::ViewGroup,
    View,
};

impl<'a, C, V, VC> Drawable for Link<V, VC>
where
    C: PixelColor,
    V: View + Drawable<Color = C>,
    VC: View + ChainElement + Drawable<Color = C>,
{
    type Color = C;
    type Output = ();

    #[inline]
    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.object.draw(display)?;
        self.parent.draw(display)?;

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

        bounds.enveloping(&self.parent.bounds())
    }

    #[inline]
    fn translate_impl(&mut self, by: Point) {
        self.object.translate_mut(by);
        self.parent.translate_mut(by);
    }
}

impl<'a, C, V> Drawable for Chain<V>
where
    C: PixelColor,
    V: View + Drawable<Color = C>,
{
    type Color = C;
    type Output = ();

    #[inline]
    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.object.draw(display)?;
        Ok(())
    }
}

impl<V> View for Chain<V>
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
    V: View,
    VC: ViewGroup + View + ChainElement,
{
    fn len(&self) -> usize {
        ChainElement::len(self) as usize
    }

    fn at(&self, index: usize) -> &dyn View {
        if index == ViewGroup::len(self) - 1 {
            return &self.object;
        }

        return self.parent.at(index);
    }

    fn at_mut(&mut self, index: usize) -> &mut dyn View {
        if index == ViewGroup::len(self) - 1 {
            return &mut self.object;
        }

        return self.parent.at_mut(index);
    }
}

impl<V> ViewGroup for Chain<V>
where
    V: View,
{
    fn len(&self) -> usize {
        ChainElement::len(self) as usize
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
