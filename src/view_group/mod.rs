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
}

/// Utility struct to simplify implementing [`View`] operations for any [`ViewGroup`].
pub struct ViewGroupHelper;

impl ViewGroupHelper {
    /// Translates every [`View`] object in a view group.
    pub fn translate(vg: &mut impl ViewGroup, by: Point) {
        for i in 0..ViewGroup::len(vg) {
            vg.at_mut(i).translate_impl(by);
        }
    }

    /// Returns the smallest bounding box that envelopes all [`View`] objects in a view group.
    pub fn bounds(vg: &impl ViewGroup) -> Rectangle {
        let mut rect = vg.at(0).bounds();

        for i in 1..vg.len() {
            rect = rect.enveloping(&vg.at(i).bounds());
        }

        rect
    }
}
