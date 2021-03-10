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

pub use crate::utils::object_chain::{Link, Tail};
use crate::{
    prelude::*,
    utils::{object_chain::ChainElement, rect_helper::RectExt},
    ViewGroup,
};
use embedded_graphics::{draw_target::DrawTarget, prelude::Point, primitives::Rectangle, Drawable};

pub mod linear;

/// Implementation detail necessary to store multiple different types of [`View`]s
/// in a [`ViewGroup`]
pub trait ViewChainElement: ChainElement + View {}

impl<'a, V, VC> Drawable for Link<V, VC>
where
    V: View + Drawable,
    VC: ViewChainElement + Drawable<Color = <V as Drawable>::Color>,
{
    type Color = <V as Drawable>::Color;

    #[inline]
    fn draw<D: DrawTarget<Color = Self::Color>>(&self, display: &mut D) -> Result<(), D::Error> {
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
    fn translate(&mut self, by: Point) {
        self.object.translate(by);
        self.next.translate(by);
    }
}

impl<V: View> ViewChainElement for Tail<V> {}

impl<V> View for Tail<V>
where
    V: View,
{
    #[inline]
    fn bounds(&self) -> Rectangle {
        self.object.bounds()
    }

    #[inline]
    fn translate(&mut self, by: Point) {
        self.object.translate(by);
    }
}

impl<V> Drawable for Tail<V>
where
    V: View + Drawable,
{
    type Color = <V as Drawable>::Color;

    #[inline]
    fn draw<D: DrawTarget<Color = Self::Color>>(&self, display: &mut D) -> Result<(), D::Error> {
        self.object.draw(display)
    }
}

impl<V, VC> ViewGroup for Link<V, VC>
where
    V: 'static + View,
    VC: 'static + ViewGroup + ViewChainElement,
{
    fn len() -> usize {
        Self::count() as usize
    }
}

impl<V, VC> core::ops::Index<usize> for Link<V, VC>
where
    V: 'static + View,
    VC: 'static + ViewGroup + ViewChainElement,
{
    type Output = dyn View;

    fn index(&self, index: usize) -> &Self::Output {
        if index == Self::len() - 1 {
            return &self.object;
        }

        return self.next.index(index - 1);
    }
}

impl<V, VC> core::ops::IndexMut<usize> for Link<V, VC>
where
    V: 'static + View,
    VC: 'static + ViewGroup + ViewChainElement,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == Self::len() - 1 {
            return &mut self.object;
        }

        return self.next.index_mut(index - 1);
    }
}

impl<V> ViewGroup for Tail<V>
where
    V: 'static + View,
{
    fn len() -> usize {
        Self::count() as usize
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
    use embedded_graphics::{
        pixelcolor::BinaryColor,
        prelude::{Primitive, Size},
        primitives::{Circle, PrimitiveStyleBuilder},
    };

    use super::*;

    fn compiles() {
        fn is_viewgroup(_v: &impl ViewGroup) {}
        fn is_drawable(_v: &impl Drawable<Color = BinaryColor>) {}

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .build();

        let rect = Rectangle::new(Point::zero(), Size::new(5, 10));
        let circle = Circle::new(Point::zero(), 12);

        let styled_rect = rect.into_styled(style);
        let styled_circle = circle.into_styled(style);

        let chain = Tail::new(styled_rect).append(styled_circle);

        is_viewgroup(&chain);
        is_drawable(&chain);
    }
}
