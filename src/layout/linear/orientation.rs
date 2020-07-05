use crate::{
    align::{Alignment, HorizontalAlignment, VerticalAlignment},
    layout::linear::secondary_alignment::SecondaryAlignment,
    prelude::*,
};

/// Helper trait that describes a linear layout orientation.
pub trait Orientation: Copy + Clone {
    /// This horizontal alignment will be applied to the first view
    type FirstHorizontalAlignment: HorizontalAlignment;

    /// This horizontal alignment will be applied to the rest of the views
    type HorizontalAlignment: HorizontalAlignment;

    /// This vertical alignment will be applied to the first view
    type FirstVerticalAlignment: VerticalAlignment;

    /// This vertical alignment will be applied to the rest of the views
    type VerticalAlignment: VerticalAlignment;

    /// Secondary alignment that will be applied to all the views
    type Secondary: SecondaryAlignment + Alignment;
}

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

impl<Secondary> Orientation for Horizontal<Secondary>
where
    Secondary: SecondaryAlignment + VerticalAlignment,
{
    type FirstHorizontalAlignment = horizontal::Left;
    type HorizontalAlignment = horizontal::LeftToRight;
    type FirstVerticalAlignment = Secondary;
    type VerticalAlignment = Secondary;
    type Secondary = Secondary;
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

impl<Secondary> Orientation for Vertical<Secondary>
where
    Secondary: SecondaryAlignment + HorizontalAlignment,
{
    type FirstHorizontalAlignment = Secondary;
    type HorizontalAlignment = Secondary;
    type FirstVerticalAlignment = vertical::Top;
    type VerticalAlignment = vertical::TopToBottom;
    type Secondary = Secondary;
}
