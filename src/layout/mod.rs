//! Layout module
//!
//! This module implements layouts that can be used to work with multiple [`View`]s easily.
//! Layouts are either [`View`] objects, or can be used to return [`View`] objects.
//!
//! The base of all layouts is the [`ViewGroup`] which binds multiple [`View`]s together.
//!
//! *Note:* [`ViewGroup`] is implemented using object chaining so it's exact type depends on it's contents.
//! This means that currently it's only possible to create **static** layouts, where the views must be
//! known at compile time.
//!
//! [`View`]: crate::View
//! [`ViewGroup`]: crate::layout::ViewGroup

use core::ops::{Index, IndexMut};

pub use crate::utils::object_chain::{Link, Tail};
use crate::{prelude::*, utils::object_chain::ChainElement};
use embedded_graphics::{primitives::Rectangle, DrawTarget};

pub mod linear;

/// Implementation detail necessary to store multiple different types of [`View`]s
/// in a [`ViewGroup`]
pub trait ViewChainElement: ChainElement + View {}

impl<'a, C, V, VC> Drawable<C> for &'a Link<V, VC>
where
    C: PixelColor,
    V: View,
    &'a V: Drawable<C>,
    VC: ViewChainElement,
    &'a VC: Drawable<C>,
{
    #[inline]
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        self.object.draw(display)?;
        self.next.draw(display)?;

        Ok(())
    }
}

impl<V: View, VC: ViewChainElement> ViewChainElement for Link<V, VC> {}

impl<V: View, VC: ViewChainElement> View for Link<V, VC> {
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

impl<V: View> ViewChainElement for Tail<V> {}

impl<V: View> View for Tail<V> {
    #[inline]
    fn bounds(&self) -> Rectangle {
        self.object.bounds()
    }

    #[inline]
    fn translate_impl(&mut self, by: Point) {
        self.object.translate_mut(by);
    }
}

pub trait ViewGroup: View + Index<usize, Output = dyn View> + IndexMut<usize> {
    fn len(&self) -> usize;
}

impl<V, VC> ViewGroup for Link<V, VC>
where
    V: 'static + View,
    VC: 'static + ViewGroup + ViewChainElement,
{
    fn len(&self) -> usize {
        Link::count(self) as usize
    }
}

impl<V, VC> core::ops::Index<usize> for Link<V, VC>
where
    V: 'static + View,
    VC: 'static + ViewGroup + ViewChainElement,
{
    type Output = dyn View;

    fn index(&self, index: usize) -> &Self::Output {
        if index == self.len() - 1 {
            return &self.object;
        }

        return self.next.index(index);
    }
}

impl<V, VC> core::ops::IndexMut<usize> for Link<V, VC>
where
    V: 'static + View,
    VC: 'static + ViewGroup + ViewChainElement,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == self.len() - 1 {
            return &mut self.object;
        }

        return self.next.index_mut(index);
    }
}

impl<V> ViewGroup for Tail<V>
where
    V: 'static + View,
{
    fn len(&self) -> usize {
        Tail::count(self) as usize
    }
}

impl<V> core::ops::Index<usize> for Tail<V>
where
    V: 'static + View,
{
    type Output = dyn View;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index == 0);

        return &self.object;
    }
}

impl<V> core::ops::IndexMut<usize> for Tail<V>
where
    V: 'static + View,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index == 0);

        return &mut self.object;
    }
}

#[cfg(test)]
mod test {
    use crate::{layout::ViewGroup, prelude::*};
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

        let chain = Tail::new(styled_rect).append(styled_circle);

        is_viewgroup(&chain);
        is_drawable(&chain);
    }
}
