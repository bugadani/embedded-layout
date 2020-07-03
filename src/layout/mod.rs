//! Layout module
//!
//! This module implements layouts that can be used to work with multiple `View`s easily.
//! Layouts are either `View` objects, or can be used to return `View` objects.
//!
//! The base of all layouts is the `ViewGroup` which binds multiple `View`s together.

use crate::prelude::*;
use embedded_graphics::primitives::Rectangle;

pub mod linear;

/// Implementation detail necessary to store multiple different types of `Views`
/// in a `ViewGroup`
pub trait ViewChainElement: View {
    /// `true` if this chain element marks the end of a chain
    const IS_TERMINATOR: bool;

    /// Return the number of `Views` linked to this chain element
    fn view_count() -> usize;

    /// Run an operation on each of the `Views` linked to this chain element
    fn for_each(&mut self, op: &mut impl FnMut(&mut dyn View));
}

/// Chain element that can store a `View` in a `ViewGroup`
///
/// You probably shouldn't ever use this struct
pub struct ViewLink<V: View, C: ViewChainElement> {
    pub(crate) view: V,
    pub(crate) next: C,
}

impl<'a, C, V, VC> Drawable<C> for &'a ViewLink<V, VC>
where
    C: PixelColor,
    V: View,
    &'a V: Drawable<C>,
    VC: ViewChainElement,
    &'a VC: Drawable<C>,
{
    #[inline]
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        self.view.draw(display)?;
        self.next.draw(display)?;

        Ok(())
    }
}

impl<V: View, VC: ViewChainElement> ViewChainElement for ViewLink<V, VC> {
    const IS_TERMINATOR: bool = false;

    #[inline]
    fn view_count() -> usize {
        1 + VC::view_count()
    }

    #[inline]
    fn for_each(&mut self, op: &mut impl FnMut(&mut dyn View)) {
        // Keep order of elements
        self.next.for_each(op);
        op(&mut self.view);
    }
}

impl<V: View, C: ViewChainElement> View for ViewLink<V, C> {
    #[inline]
    fn bounds(&self) -> Rectangle {
        let bounds = self.view.bounds();

        if !C::IS_TERMINATOR {
            bounds.enveloping(&self.next.bounds())
        } else {
            bounds
        }
    }

    #[inline]
    fn translate(&mut self, by: Point) {
        self.view.translate(by);
        self.next.translate(by);
    }
}

/// The last chain element that marks the end of a `ViewGroup`
///
/// You probably shouldn't ever use this struct
pub struct ChainTerminator;

impl ViewChainElement for ChainTerminator {
    const IS_TERMINATOR: bool = true;

    #[inline]
    fn view_count() -> usize {
        0
    }

    #[inline]
    fn for_each(&mut self, _op: &mut impl FnMut(&mut dyn View)) {
        // nothing to do
    }
}

impl<C: PixelColor> Drawable<C> for &ChainTerminator {
    #[inline]
    fn draw<D: DrawTarget<C>>(self, _display: &mut D) -> Result<(), D::Error> {
        Ok(())
    }
}

impl View for ChainTerminator {
    #[inline]
    fn bounds(&self) -> Rectangle {
        Rectangle::new(Point::zero(), Point::zero())
    }

    #[inline]
    fn translate(&mut self, _by: Point) {
        // nothing to do
    }
}

/// Group multiple `View`s together
///
/// `ViewGroup` takes ownership over the views, so make sure you set them up before creating
/// the group.
/// The bounds and size of a `ViewGroup` envelops all the contained `View`s.
///
/// Note: translating an empty `ViewGroup` has no effect
pub struct ViewGroup<C: ViewChainElement> {
    pub(crate) views: C,
}

impl ViewGroup<ChainTerminator> {
    /// Create a new, empty `ViewGroup` object
    #[inline]
    pub const fn new() -> Self {
        Self {
            views: ChainTerminator,
        }
    }
}

impl Default for ViewGroup<ChainTerminator> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<C: ViewChainElement> ViewGroup<C> {
    /// Bind a `View` to this `ViewGroup`
    ///
    /// The `View` remains at it's current location, until the `ViewGroup` is translated.
    #[inline]
    pub fn add_view<V: View>(self, view: V) -> ViewGroup<ViewLink<V, C>> {
        ViewGroup {
            views: ViewLink {
                view,
                next: self.views,
            },
        }
    }

    /// Run the callback on each included `View` objects
    #[inline]
    pub fn for_each(&mut self, op: &mut impl FnMut(&mut dyn View)) {
        self.views.for_each(op);
    }

    /// Returns the number of views in this `ViewGroup`
    #[inline]
    pub fn view_count(&self) -> usize {
        C::view_count()
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
    /// Draw the graphics object using the supplied DrawTarget.
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
    use embedded_graphics::mock_display::MockDisplay;
    use embedded_graphics::{
        pixelcolor::BinaryColor,
        primitives::{Circle, Rectangle},
        style::PrimitiveStyle,
    };

    #[test]
    fn sanity_check() {
        // Check if multiple different views can be included in the view group
        let vg = ViewGroup::new()
            .add_view(Rectangle::with_size(Point::zero(), Size::new(5, 10)))
            .add_view(Circle::new(Point::zero(), 5));

        fn check_vg<C: ViewChainElement>(vg: &ViewGroup<C>) {
            assert_eq!(2, vg.view_count());
        }

        check_vg(&vg);
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

    #[test]
    fn test_empty() {
        // This tests that the view group implements Drawable as expected
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        ViewGroup::new().draw(&mut disp).unwrap();
    }

    #[test]
    fn test_empty_nested() {
        // This tests that the view group implements Drawable as expected
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        ViewGroup::new()
            .add_view(ViewGroup::new())
            .draw(&mut disp)
            .unwrap();
    }

    #[test]
    fn test_nested() {
        // This tests that the view group implements Drawable as expected
        let mut disp: MockDisplay<BinaryColor> = MockDisplay::new();

        let style = PrimitiveStyle::with_fill(BinaryColor::On);
        ViewGroup::new()
            .add_view(Rectangle::new(Point::zero(), Point::new(10, 5)).into_styled(style))
            .add_view(
                ViewGroup::new()
                    .add_view(Rectangle::new(Point::zero(), Point::zero()).into_styled(style))
                    .add_view(Rectangle::new(Point::zero(), Point::zero()).into_styled(style)),
            )
            .add_view(Rectangle::new(Point::zero(), Point::zero()).into_styled(style))
            .draw(&mut disp)
            .unwrap();
    }
}
