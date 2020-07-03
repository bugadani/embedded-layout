use crate::{
    align::{HorizontalAlignment, VerticalAlignment},
    layout::linear::secondary_alignment::SecondaryAlignment,
    prelude::*,
};

/// Helper trait that describes a layout direction.
pub trait LayoutDirection: Copy + Clone {}
/// Horizontal layout direction
#[derive(Copy, Clone)]
pub struct Horizontal<Secondary: SecondaryAlignment + VerticalAlignment> {
    pub(crate) secondary: Secondary,
}

impl Default for Horizontal<vertical::Bottom> {
    #[inline]
    fn default() -> Self {
        Self {
            secondary: vertical::Bottom,
        }
    }
}

impl<Secondary> LayoutDirection for Horizontal<Secondary> where
    Secondary: SecondaryAlignment + VerticalAlignment
{
}

/// Vertical layout direction
#[derive(Copy, Clone)]
pub struct Vertical<Secondary: SecondaryAlignment + HorizontalAlignment> {
    pub(crate) secondary: Secondary,
}

impl Default for Vertical<horizontal::Left> {
    #[inline]
    fn default() -> Self {
        Self {
            secondary: horizontal::Left,
        }
    }
}

impl<Secondary> LayoutDirection for Vertical<Secondary> where
    Secondary: SecondaryAlignment + HorizontalAlignment
{
}
