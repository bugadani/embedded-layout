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

    /// Adjust measured size based on element spacing
    fn adjust_size(self, size: Size, objects: usize) -> Size;

    ///
    fn place_first(&self, view: &mut impl View, bounds: &Rectangle, count: usize);

    ///
    fn place_nth(
        &self,
        view: &mut impl View,
        size: Size,
        previous: &Rectangle,
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
    ///
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

    ///
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
    fn place_first(&self, view: &mut impl View, bounds: &Rectangle, count: usize) {
        view.align_to_mut(bounds, horizontal::Left, Secondary::default());
        view.translate(Point::new(
            self.spacing
                .modify_placement(0, count, RectExt::size(bounds).width),
            0,
        ));
    }

    #[inline]
    fn place_nth(
        &self,
        view: &mut impl View,
        size: Size,
        previous: &Rectangle,
        n: usize,
        count: usize,
    ) {
        view.align_to_mut(
            previous,
            horizontal::LeftToRight::default(),
            Secondary::default(),
        );
        view.translate(Point::new(
            self.spacing.modify_placement(n, count, size.width),
            0,
        ));
    }

    #[inline]
    fn adjust_size(self, size: Size, objects: usize) -> Size {
        Size::new(
            self.spacing.modify_measurement(size.width, objects),
            size.height,
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
    ///
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

    ///
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
    fn place_first(&self, view: &mut impl View, bounds: &Rectangle, count: usize) {
        view.align_to_mut(bounds, Secondary::default(), vertical::Top);
        view.translate(Point::new(
            self.spacing
                .modify_placement(0, count, RectExt::size(bounds).width),
            0,
        ));
    }

    #[inline]
    fn place_nth(
        &self,
        view: &mut impl View,
        size: Size,
        previous: &Rectangle,
        n: usize,
        count: usize,
    ) {
        view.align_to_mut(previous, Secondary::default(), vertical::TopToBottom);
        view.translate(Point::new(
            0,
            self.spacing.modify_placement(n, count, size.height),
        ));
    }

    #[inline]
    fn adjust_size(self, size: Size, objects: usize) -> Size {
        Size::new(
            size.width,
            self.spacing.modify_measurement(size.height, objects),
        )
    }
}
