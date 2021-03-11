//! ViewGroup definition and implementation for common types.

use embedded_graphics::{prelude::Point, primitives::Rectangle};

use crate::{prelude::RectExt, View};

mod object_chain;

pub trait ViewGroup: View {
    fn len(&self) -> usize;

    fn at(&self, idx: usize) -> &dyn View;

    fn at_mut(&mut self, idx: usize) -> &mut dyn View;
}

pub struct ViewGroupHelper;

impl ViewGroupHelper {
    pub fn translate(vg: &mut impl ViewGroup, by: Point) {
        for i in 0..ViewGroup::len(vg) {
            vg.at_mut(i).translate_impl(by);
        }
    }

    pub fn bounds(vg: &impl ViewGroup) -> Rectangle {
        let mut rect = vg.at(0).bounds();

        for i in 1..vg.len() {
            rect = rect.enveloping(&vg.at(i).bounds());
        }

        rect
    }
}
