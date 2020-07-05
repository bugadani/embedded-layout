use crate::{
    align::{Alignment, HorizontalAlignment, VerticalAlignment},
    layout::linear::{secondary_alignment::SecondaryAlignment, spacing::ElementSpacing},
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

    /// Adjust measured size based on element spacing
    fn adjust_size(size: Size, objects: usize, spacing: &impl ElementSpacing) -> Size;

    /// Adjust object position in layout, based on element spacing
    fn adjust_placement(
        view: &mut impl View,
        spacing: &impl ElementSpacing,
        n: usize,
        size: Size,
        count: usize,
    );
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

    #[inline]
    fn adjust_size(size: Size, objects: usize, spacing: &impl ElementSpacing) -> Size {
        Size::new(spacing.modify_measurement(size.width, objects), size.height)
    }

    #[inline]
    fn adjust_placement(
        view: &mut impl View,
        spacing: &impl ElementSpacing,
        n: usize,
        size: Size,
        count: usize,
    ) {
        view.translate(Point::new(
            spacing.modify_placement(n, count, size.width),
            0,
        ));
    }
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

    #[inline]
    fn adjust_size(size: Size, objects: usize, spacing: &impl ElementSpacing) -> Size {
        Size::new(size.width, spacing.modify_measurement(size.height, objects))
    }

    #[inline]
    fn adjust_placement(
        view: &mut impl View,
        spacing: &impl ElementSpacing,
        n: usize,
        size: Size,
        count: usize,
    ) {
        view.translate(Point::new(
            0,
            spacing.modify_placement(n, count, size.height),
        ));
    }
}
