//! ViewGroup definition and implementation for common types.

use embedded_graphics::{prelude::Point, primitives::Rectangle};

use crate::{prelude::RectExt, View};

mod object_chain;
mod views;

pub use views::Views;

/// A set of operations required to implement [`View`] containers.
pub trait ViewGroup: View {
    /// Returns the number of [`View`] objects in this view group.
    fn len(&self) -> usize;

    /// Returns a shared reference the [`View`] object at position `idx`.
    fn at(&self, idx: usize) -> &dyn View;

    /// Returns an exclusive reference to the [`View`] object at position `idx`.
    fn at_mut(&mut self, idx: usize) -> &mut dyn View;

    /// Returns the bounding box of the given View.
    fn bounds_of(&self, idx: usize) -> Rectangle {
        self.at(idx).bounds()
    }

    /// Translates the given View.
    fn translate_child(&mut self, idx: usize, by: Point) {
        self.at_mut(idx).translate_impl(by)
    }
}

/// A [`ViewGroup`] that contains no [`View`] objects.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EmptyViewGroup;

/// A single instance of [`EmptyViewGroup`].
pub static mut EMPTY_VIEW_GROUP: EmptyViewGroup = EmptyViewGroup;

impl View for EmptyViewGroup {
    fn translate_impl(&mut self, _by: Point) {}

    fn bounds(&self) -> Rectangle {
        Rectangle::zero()
    }
}

impl ViewGroup for EmptyViewGroup {
    fn len(&self) -> usize {
        0
    }

    fn at(&self, _idx: usize) -> &dyn View {
        self
    }

    fn at_mut(&mut self, _idx: usize) -> &mut dyn View {
        self
    }
}

/// Utility struct to simplify implementing [`View`] operations for any [`ViewGroup`].
pub struct ViewGroupHelper;

impl ViewGroupHelper {
    /// Translates every [`View`] object in a view group.
    #[inline]
    pub fn translate(vg: &mut impl ViewGroup, by: Point) {
        for i in 0..ViewGroup::len(vg) {
            vg.translate_child(i, by);
        }
    }

    /// Returns the smallest bounding box that envelopes all [`View`] objects in a view group.
    #[inline]
    pub fn bounds(vg: &impl ViewGroup) -> Rectangle {
        if ViewGroup::len(vg) == 0 {
            return EmptyViewGroup.bounds();
        }

        let mut rect = vg.bounds_of(0);

        for i in 1..vg.len() {
            rect = rect.enveloping(&vg.bounds_of(i));
        }

        rect
    }
}
