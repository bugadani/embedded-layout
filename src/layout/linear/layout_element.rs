use crate::{
    layout::{
        linear::{orientation::Orientation, secondary_alignment::SecondaryAlignment},
        Link, Tail, ViewChainElement,
    },
    prelude::*,
};
use embedded_graphics::{prelude::Size, primitives::Rectangle};

/// This trait extends view chain elements with methods to build a [`LinearLayout`]
pub trait LayoutElement<LD: Orientation>: ViewChainElement {
    /// Return the size of the view chain
    fn measure(&self) -> Size;

    /// Arrange views on the display
    fn arrange(&mut self, bounds: Rectangle, orientation: &LD, count: u32) -> Rectangle;
}

impl<V, VCE, LD> LayoutElement<LD> for Link<V, VCE>
where
    V: View,
    VCE: LayoutElement<LD>,
    LD: Orientation,
{
    #[inline]
    fn measure(&self) -> Size {
        let current_el_size = self.object.bounds().size;
        let prev_size = self.next.measure();
        LD::Secondary::measure(prev_size, current_el_size)
    }

    #[inline]
    fn arrange(&mut self, bounds: Rectangle, orientation: &LD, count: u32) -> Rectangle {
        let previous = self.next.arrange(bounds, orientation, count);
        orientation.place_nth(&mut self.object, bounds.size, previous, VCE::count(), count);

        self.object.bounds()
    }
}

impl<V, LD: Orientation> LayoutElement<LD> for Tail<V>
where
    V: View,
{
    #[inline]
    fn measure(&self) -> Size {
        self.object.bounds().size
    }

    #[inline]
    fn arrange(&mut self, bounds: Rectangle, orientation: &LD, count: u32) -> Rectangle {
        // Nothing to do
        orientation.place_first(&mut self.object, bounds, count);

        self.object.bounds()
    }
}
