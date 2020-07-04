use crate::{
    align::{Alignment, HorizontalAlignment, VerticalAlignment},
    layout::linear::secondary_alignment::SecondaryAlignment,
    prelude::*,
};

/// Helper trait that describes a layout direction.
pub trait LayoutDirection: Copy + Clone {
    ///
    type FirstHorizontalAlignment: HorizontalAlignment;

    ///
    type HorizontalAlignment: HorizontalAlignment;

    ///
    type FirstVerticalAlignment: VerticalAlignment;

    ///
    type VerticalAlignment: VerticalAlignment;

    ///
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

impl<Secondary> LayoutDirection for Horizontal<Secondary>
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

impl<Secondary> LayoutDirection for Vertical<Secondary>
where
    Secondary: SecondaryAlignment + HorizontalAlignment,
{
    type FirstHorizontalAlignment = Secondary;
    type HorizontalAlignment = Secondary;
    type FirstVerticalAlignment = vertical::Top;
    type VerticalAlignment = vertical::TopToBottom;
    type Secondary = Secondary;
}
