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

pub use crate::utils::object_chain::{Guard, Link};
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

        if VC::IS_TERMINATOR {
            bounds
        } else {
            bounds.enveloping(&self.next.bounds())
        }
    }

    #[inline]
    fn translate(&mut self, by: Point) {
        self.object.translate(by);
        self.next.translate(by);
    }
}

impl ViewChainElement for Guard {}

impl<C: PixelColor> Drawable<C> for &Guard {
    #[inline]
    fn draw<D: DrawTarget<C>>(self, _display: &mut D) -> Result<(), D::Error> {
        Ok(())
    }
}

impl View for Guard {
    #[inline]
    fn bounds(&self) -> Rectangle {
        Rectangle::new(Point::zero(), Point::zero())
    }

    #[inline]
    fn translate(&mut self, _by: Point) {
        // nothing to do
    }
}

/// Group multiple [`View`]s together
///
/// [`ViewGroup`] takes ownership over the views, so make sure you set them up before creating
/// the group.
/// The bounds and size of a [`ViewGroup`] envelops all the contained [`View`]s.
///
/// Note: translating an empty [`ViewGroup`] has no effect
pub struct ViewGroup<C: ViewChainElement = Guard> {
    pub(crate) views: C,
}

impl ViewGroup<Guard> {
    /// Create a new, empty [`ViewGroup`] object
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { views: Guard }
    }
}

impl Default for ViewGroup<Guard> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<C: ViewChainElement> ViewGroup<C> {
    /// Bind a [`View`] to this [`ViewGroup`]
    ///
    /// The [`View`] remains at it's current location, until the [`ViewGroup`] is translated.
    #[inline]
    pub fn add_view<V: View>(self, view: V) -> ViewGroup<Link<V, C>> {
        ViewGroup {
            views: Link {
                object: view,
                next: self.views,
            },
        }
    }

    /// Returns the number of views in this [`ViewGroup`]
    #[inline]
    pub fn view_count(&self) -> u32 {
        C::count()
    }
}

impl<C: ViewChainElement> View for ViewGroup<C> {
    #[inline]
    fn translate(&mut self, by: Point) {
        self.views.translate(by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        self.views.bounds()
    }
}

impl<'a, C, VC> Drawable<C> for &'a ViewGroup<VC>
where
    C: PixelColor,
    VC: ViewChainElement,
    &'a VC: Drawable<C>,
{
    /// Draw the graphics object using the supplied `DrawTarget`.
    #[inline]
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        self.views.draw(display)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        layout::{ViewChainElement, ViewGroup},
        prelude::*,
    };
    use embedded_graphics::{
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{Circle, Rectangle},
        style::PrimitiveStyle,
    };

    #[test]
    fn compile_check() {
        fn check_vg<C: ViewChainElement>(vg: &ViewGroup<C>) {
            assert_eq!(2, vg.view_count());
        }

        // Check if multiple different views can be included in the view group
        let vg = ViewGroup::new()
            .add_view(Rectangle::with_size(Point::zero(), Size::new(5, 10)))
            .add_view(Circle::new(Point::zero(), 5));

        check_vg(&vg);
    }

    #[test]
    fn compile_check_empty() {
        // This tests that the view group implements Drawable as expected
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        ViewGroup::new().draw(&mut disp).unwrap();
    }

    #[test]
    fn compile_check_empty_nested() {
        // This tests that the view group implements Drawable as expected
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        ViewGroup::new()
            .add_view(ViewGroup::new())
            .draw(&mut disp)
            .unwrap();
    }

    #[test]
    fn compile_check_complex_nested() {
        // This tests that the view group implements Drawable as expected
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        ViewGroup::new()
            .add_view(Rectangle::new(Point::zero(), Point::new(10, 5)).into_styled(style))
            .add_view(
                ViewGroup::new()
                    .add_view(Rectangle::new(Point::zero(), Point::zero()).into_styled(style))
                    .add_view(Circle::new(Point::zero(), 5).into_styled(style)),
            )
            .add_view(Rectangle::new(Point::zero(), Point::zero()).into_styled(style))
            .draw(&mut disp)
            .unwrap();
    }

    #[test]
    fn test() {
        // Check if multiple different views can be included in the view group
        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        let mut vg = ViewGroup::new()
            .add_view(Rectangle::with_size(Point::zero(), Size::new(5, 10)).into_styled(style))
            .add_view(Rectangle::with_size(Point::new(3, 5), Size::new(5, 10)).into_styled(style))
            .add_view(
                Rectangle::with_size(Point::new(-2, -5), Size::new(5, 10)).into_styled(style),
            );

        assert_eq!(Size::new(10, 20), vg.size());
        assert_eq!(
            Rectangle::new(Point::new(-2, -5), Point::new(7, 14)),
            vg.bounds()
        );

        vg.translate(Point::new(2, 3));

        assert_eq!(Size::new(10, 20), vg.size());
        assert_eq!(
            Rectangle::new(Point::new(0, -2), Point::new(9, 17)),
            vg.bounds()
        );

        // This tests that the view group implements Drawable as expected
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        vg.draw(&mut disp).unwrap();
    }

    #[test]
    fn test_align() {
        // This tests that the view group implements Drawable as expected
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        let rect3 = Rectangle::with_size(Point::new(-2, -5), Size::new(5, 10)).into_styled(style);
        ViewGroup::new()
            .add_view(Rectangle::with_size(Point::zero(), Size::new(5, 10)).into_styled(style))
            .add_view(Rectangle::with_size(Point::new(3, 5), Size::new(5, 10)).into_styled(style))
            .align_to(&rect3, horizontal::LeftToRight, vertical::TopToBottom)
            .draw(&mut disp)
            .unwrap();
    }
}
