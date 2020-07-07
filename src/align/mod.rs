//! Alignment operations
//!
//! Alignment operations are used to arrange two [`View`]s relative to each other. A single [`align_*`]
//! call requires both a `horizontal` and a `vertical` alignment parameter.
//!
//! The list of currently supported alignments:
//!  - [`horizontal`]
//!    - `NoAlignment`, `Left`, `Center`, `Right`
//!    - `LeftToRight`
//!    - `RightToLeft`
//!  - [`vertical`]
//!    - `NoAlignment`, `Top`, `Center`, `Bottom`
//!    - `TopToBottom`
//!    - `BottomToTop`
//!
//! Alignment works by calling [`align_to`] or [`align_to_mut`] on an object that implements
//! the [`Align`] trait. The call needs a second [`View`] to align to, called the reference [`View`],
//! and two alignment parameters. The second [`View`] will not be translated by the alignment
//! operation.
//!
//! [`horizontal`]: crate::align::horizontal
//! [`vertical`]: crate::align::vertical
//! [`align_*`]: crate::align::Align
//! [`align_to`]: crate::align::Align::align_to
//! [`align_to_mut`]: crate::align::Align::align_to_mut
use crate::prelude::*;
use embedded_graphics::primitives::Rectangle;

pub mod horizontal;
pub mod vertical;

/// This trait enables alignment operations for [`View`] objects
///
/// This trait is blanket-implemented for all objects that implement [`View`].
///
/// For more information, see the [module level documentation](crate::align)
pub trait Align {
    /// Return the object aligned to an other one using the alignment parameters as rules
    fn align_to<H, V>(self, reference: &impl View, horizontal: H, vertical: V) -> Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment;

    /// Align the object to an other one using the alignment parameters as rules
    fn align_to_mut<H, V>(
        &mut self,
        reference: &impl View,
        horizontal: H,
        vertical: V,
    ) -> &mut Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment;
}

impl<T> Align for T
where
    T: View,
{
    #[inline]
    fn align_to<H, V>(mut self, reference: &impl View, horizontal: H, vertical: V) -> Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment,
    {
        self.align_to_mut(reference, horizontal, vertical);
        self
    }

    #[inline]
    fn align_to_mut<H, V>(&mut self, reference: &impl View, horizontal: H, vertical: V) -> &mut Self
    where
        H: HorizontalAlignment,
        V: VerticalAlignment,
    {
        let self_bounds = self.bounds();
        let reference_bounds = reference.bounds();

        let h = horizontal.align(self_bounds, reference_bounds);
        let v = vertical.align(self_bounds, reference_bounds);
        self.translate(Point::new(h, v));
        self
    }
}

/// Base trait for alignment operations
///
/// An [`Alignment`] object modifies either the horizontal, or the vertical position of a [`View`].
/// Usually, these objects are passed to methods of [`Align`] or they can be used to parametrize
/// layouts.
///
/// Implementors should also implement either the [`HorizontalAlignment`] or [`VerticalAlignment`]
/// trait, otherwise the implementor can't be used as an alignment operation with `embedded-layout`.
pub trait Alignment: Copy + Clone + Default {
    /// Align one coordinate of `View` to the given reference
    #[inline]
    fn align(&self, what: Rectangle, reference: Rectangle) -> i32 {
        self.align_with_offset(what, reference, 0)
    }

    /// Align one coordinate of `View` to the given reference with some offset
    fn align_with_offset(&self, what: Rectangle, reference: Rectangle, offset: i32) -> i32;
}

/// Implement this trait for horizontal alignment operations
///
/// This trait does not provide any functionality other than that of [`Alignment`], but marks
/// implementors to be used as horizontal alignmenent operations.
///
/// For a list of available horizontal alignments, see the [`horizontal`] module.
///
/// [`horizontal`]: crate::align::horizontal
pub trait HorizontalAlignment: Alignment {}

/// Implement this trait for vertical alignment operations
///
/// Vertical alignment assumes lower coordinate values are higher up on the display.
///
/// This trait does not provide any functionality other than that of [`Alignment`], but marks
/// implementors to be used as vertical alignmenent operations.
///
/// For a list of available vertical alignments, see the [`vertical`] module.
///
/// [`vertical`]: crate::align::vertical
pub trait VerticalAlignment: Alignment {}
