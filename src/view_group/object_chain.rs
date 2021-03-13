//! ViewGroup implementation for object chains.

use embedded_graphics::{
    drawable::Drawable, pixelcolor::PixelColor, prelude::Point, primitives::Rectangle, DrawTarget,
};

use crate::{
    object_chain::{Chain, ChainElement, Link},
    prelude::RectExt,
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

impl<'a, C, V> Drawable<C> for &'a Chain<V>
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

        return self.parent.at(index);
    }

    fn at_mut(&mut self, index: usize) -> &mut dyn View {
        if index == self.len() - 1 {
            return &mut self.object;
        }

        return self.parent.at_mut(index);
    }
}

impl<V> ViewGroup for Chain<V>
where
    V: 'static + View,
{
    fn len(&self) -> usize {
        Chain::count(self) as usize
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

#[cfg(test)]
mod test {
    use crate::{prelude::*, view_group::ViewGroup};
    use embedded_graphics::{
        pixelcolor::BinaryColor,
        primitives::{Circle, Rectangle},
        style::PrimitiveStyleBuilder,
    };

    #[allow(dead_code)]
    fn compile_check() {
        fn is_viewgroup(_v: &impl ViewGroup) {}
        fn is_drawable(_v: impl Drawable<BinaryColor>) {}

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .build();

        let rect = Rectangle::with_size(Point::zero(), Size::new(5, 10));
        let circle = Circle::new(Point::zero(), 12);

        let styled_rect = rect.into_styled(style);
        let styled_circle = circle.into_styled(style);

        let chain = Chain::new(styled_rect).append(styled_circle);

        is_viewgroup(&chain);
        is_drawable(&chain);
    }
}
