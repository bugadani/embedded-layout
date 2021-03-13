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

pub mod linear;

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
