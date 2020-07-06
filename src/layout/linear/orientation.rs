use crate::{
    align::{Alignment, HorizontalAlignment, VerticalAlignment},
    layout::linear::{
        secondary_alignment::SecondaryAlignment,
        spacing::{ElementSpacing, Tight},
    },
    prelude::*,
};
use embedded_graphics::primitives::Rectangle;

/// Helper trait that describes a linear layout orientation.
pub trait Orientation: Copy + Clone {
    /// Secondary alignment that will be applied to all the views
    type Secondary: SecondaryAlignment + Alignment;

    /// Destructure `Size` into (primary_size, secondary_size)
    fn destructure_size(size: Size) -> (u32, u32);

    /// Create a `Size` from primary and secondary size values
    fn create_size(primary: u32, secondary: u32) -> Size;

    /// Adjust measured size based on element spacing
    fn adjust_size(self, size: Size, objects: usize) -> Size;

    /// Place first view
    fn place_first(&self, view: &mut impl View, bounds: Rectangle, count: usize);

    /// Place nth view
    fn place_nth(
        &self,
        view: &mut impl View,
        size: Size,
        previous: Rectangle,
        n: usize,
        count: usize,
    );
}

/// Horizontal layout direction
#[derive(Copy, Clone)]
pub struct Horizontal<Secondary, Spacing>
where
    Secondary: SecondaryAlignment + VerticalAlignment,
    Spacing: ElementSpacing,
{
    pub(crate) secondary: Secondary,
    pub(crate) spacing: Spacing,
}

impl<Secondary, Spacing> Horizontal<Secondary, Spacing>
where
    Secondary: SecondaryAlignment + VerticalAlignment,
    Spacing: ElementSpacing,
{
    /// Change secondary alignment
    #[inline]
    pub fn with_secondary_alignment<Sec: SecondaryAlignment + VerticalAlignment>(
        self,
        secondary: Sec,
    ) -> Horizontal<Sec, Spacing> {
        Horizontal {
            secondary,
            spacing: self.spacing,
        }
    }

    /// Change element spacing
    #[inline]
    pub fn with_spacing<ElSpacing: ElementSpacing>(
        self,
        spacing: ElSpacing,
    ) -> Horizontal<Secondary, ElSpacing> {
        Horizontal {
            secondary: self.secondary,
            spacing,
        }
    }
}

impl Default for Horizontal<vertical::Bottom, Tight> {
    #[inline]
    fn default() -> Self {
        Self {
            secondary: vertical::Bottom,
            spacing: Tight,
        }
    }
}

impl<Secondary, Spacing> Orientation for Horizontal<Secondary, Spacing>
where
    Secondary: SecondaryAlignment + VerticalAlignment,
    Spacing: ElementSpacing,
{
    type Secondary = Secondary;

    #[inline]
    fn destructure_size(size: Size) -> (u32, u32) {
        (size.width, size.height)
    }

    #[inline]
    fn create_size(primary: u32, secondary: u32) -> Size {
        Size::new(primary, secondary)
    }

    #[inline]
    fn place_first(&self, view: &mut impl View, bounds: Rectangle, count: usize) {
        let (primary_size, _) = Self::destructure_size(RectExt::size(&bounds));
        let view_bounds = view.bounds();

        view.translate(Point::new(
            self.spacing.align(
                horizontal::Left,
                view_bounds,
                bounds,
                0,
                count,
                primary_size,
            ),
            Secondary::default().align(view_bounds, bounds),
        ));
    }

    #[inline]
    fn place_nth(
        &self,
        view: &mut impl View,
        size: Size,
        previous: Rectangle,
        n: usize,
        count: usize,
    ) {
        let (primary_size, _) = Self::destructure_size(size);
        let view_bounds = view.bounds();

        view.translate(Point::new(
            self.spacing.align(
                horizontal::LeftToRight,
                view_bounds,
                previous,
                n,
                count,
                primary_size,
            ),
            Secondary::default().align(view_bounds, previous),
        ));
    }

    #[inline]
    fn adjust_size(self, size: Size, objects: usize) -> Size {
        let (primary_size, secondary_size) = Self::destructure_size(size);
        Self::create_size(
            self.spacing.modify_measurement(primary_size, objects),
            secondary_size,
        )
    }
}

/// Vertical layout direction
#[derive(Copy, Clone)]
pub struct Vertical<Secondary, Spacing>
where
    Secondary: SecondaryAlignment + HorizontalAlignment,
    Spacing: ElementSpacing,
{
    pub(crate) secondary: Secondary,
    pub(crate) spacing: Spacing,
}

impl Default for Vertical<horizontal::Left, Tight> {
    #[inline]
    fn default() -> Self {
        Self {
            secondary: horizontal::Left,
            spacing: Tight,
        }
    }
}

impl<Secondary, Spacing> Vertical<Secondary, Spacing>
where
    Secondary: SecondaryAlignment + HorizontalAlignment,
    Spacing: ElementSpacing,
{
    /// Change secondary alignment
    #[inline]
    pub fn with_secondary_alignment<Sec: SecondaryAlignment + HorizontalAlignment>(
        self,
        secondary: Sec,
    ) -> Vertical<Sec, Spacing> {
        Vertical {
            secondary,
            spacing: self.spacing,
        }
    }

    /// Change element spacing
    #[inline]
    pub fn with_spacing<ElSpacing: ElementSpacing>(
        self,
        spacing: ElSpacing,
    ) -> Vertical<Secondary, ElSpacing> {
        Vertical {
            secondary: self.secondary,
            spacing,
        }
    }
}

impl<Secondary, Spacing> Orientation for Vertical<Secondary, Spacing>
where
    Secondary: SecondaryAlignment + HorizontalAlignment,
    Spacing: ElementSpacing,
{
    type Secondary = Secondary;

    #[inline]
    fn destructure_size(size: Size) -> (u32, u32) {
        (size.height, size.width)
    }

    #[inline]
    fn create_size(primary: u32, secondary: u32) -> Size {
        Size::new(secondary, primary)
    }

    #[inline]
    fn place_first(&self, view: &mut impl View, bounds: Rectangle, count: usize) {
        let (primary_size, _) = Self::destructure_size(RectExt::size(&bounds));
        let view_bounds = view.bounds();

        view.translate(Point::new(
            Secondary::default().align(view_bounds, bounds),
            self.spacing
                .align(vertical::Top, view_bounds, bounds, 0, count, primary_size),
        ));
    }

    #[inline]
    fn place_nth(
        &self,
        view: &mut impl View,
        size: Size,
        previous: Rectangle,
        n: usize,
        count: usize,
    ) {
        let (primary_size, _) = Self::destructure_size(size);
        let view_bounds = view.bounds();

        view.translate(Point::new(
            Secondary::default().align(view_bounds, previous),
            self.spacing.align(
                vertical::TopToBottom,
                view_bounds,
                previous,
                n,
                count,
                primary_size,
            ),
        ));
    }

    #[inline]
    fn adjust_size(self, size: Size, objects: usize) -> Size {
        let (primary_size, secondary_size) = Self::destructure_size(size);
        Self::create_size(
            self.spacing.modify_measurement(primary_size, objects),
            secondary_size,
        )
    }
}
