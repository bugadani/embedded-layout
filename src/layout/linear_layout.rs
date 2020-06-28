//! Linear layout
//!
//! Lay out display objects either horizontally, or vertically.

use embedded_graphics::geometry::Point;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use crate::{prelude::*, HorizontalAlignment, VerticalAlignment};

pub trait LayoutDirection: Copy + Clone {
    type Alignment;
    fn new(alignment: Self::Alignment) -> Self;

    /// Measure the direction used for alignment
    fn measure(&self, view: &impl View, acc: Size) -> Size;

    /// Place the first view
    fn layout_first(&self, view: &mut impl View, bounds: &Rectangle);

    /// Place the view
    fn layout(&self, view: &mut impl View, bounds: &Rectangle, previous_view: &impl View);
}

pub trait LinearLayoutHorizontalAlignment: HorizontalAlignment {}
pub trait LinearLayoutVerticalAlignment: VerticalAlignment {}

impl LinearLayoutHorizontalAlignment for horizontal::Left {}
impl LinearLayoutHorizontalAlignment for horizontal::Center {}
impl LinearLayoutHorizontalAlignment for horizontal::Right {}

impl LinearLayoutVerticalAlignment for vertical::Top {}
impl LinearLayoutVerticalAlignment for vertical::Center {}
impl LinearLayoutVerticalAlignment for vertical::Bottom {}

#[derive(Copy, Clone)]
pub struct Horizontal<V: LinearLayoutVerticalAlignment> {
    alignment: V,
}

#[derive(Copy, Clone)]
pub struct Vertical<H: LinearLayoutHorizontalAlignment> {
    alignment: H,
}

impl Default for Horizontal<vertical::Top> {
    fn default() -> Self {
        Self::new(vertical::Top)
    }
}

impl Default for Vertical<horizontal::Left> {
    fn default() -> Self {
        Self::new(horizontal::Left)
    }
}

impl<V: LinearLayoutVerticalAlignment> LayoutDirection for Horizontal<V> {
    type Alignment = V;
    fn new(alignment: V) -> Self {
        Self { alignment }
    }

    fn measure(&self, view: &impl View, total_size: Size) -> Size
    {
        let Size { width, height } = RectExt::size(&view.bounds());

        Size::new(total_size.width + width, total_size.height.max(height))
    }

    fn layout_first(&self, view: &mut impl View, bounds: &Rectangle)
    {
        view.align_to(bounds, horizontal::Left, self.alignment);
    }

    fn layout(&self, view: &mut impl View, bounds: &Rectangle, previous_view: &impl View)
    {
        view.align_to(
            previous_view,
            horizontal::LeftToRight,
            vertical::NoAlignment,
        );
        view.align_to(bounds, horizontal::NoAlignment, self.alignment);
    }
}

impl<H: LinearLayoutHorizontalAlignment> LayoutDirection for Vertical<H> {
    type Alignment = H;
    fn new(alignment: H) -> Self {
        Self { alignment }
    }

    fn measure(&self, view: &impl View, total_size: Size) -> Size
    {
        let Size { width, height } = RectExt::size(&view.bounds());

        Size::new(total_size.width.max(width), total_size.height + height)
    }

    fn layout_first(&self, view: &mut impl View, bounds: &Rectangle)
    {
        view.align_to(bounds, self.alignment, vertical::Top);
    }

    fn layout(&self, view: &mut impl View, bounds: &Rectangle, previous_view: &impl View)
    {
        view.align_to(
            previous_view,
            horizontal::NoAlignment,
            vertical::TopToBottom,
        );
        view.align_to(bounds, self.alignment, vertical::NoAlignment);
    }
}

pub struct LinearLayout<DIR: LayoutDirection> {
    direction: DIR,
    top_left: Point,
}

impl LinearLayout<Horizontal<vertical::Top>> {
    pub fn horizontal(top_left: Point) -> Self {
        Self {
            direction: Horizontal::default(),
            top_left,
        }
    }
}

impl LinearLayout<Vertical<horizontal::Left>> {
    pub fn vertical(top_left: Point) -> Self {
        Self {
            direction: Vertical::default(),
            top_left,
        }
    }
}

impl<V: LinearLayoutVerticalAlignment> LinearLayout<Horizontal<V>> {
    pub fn with_vertical_alignment<VA>(self, v: VA) -> LinearLayout<Horizontal<VA>>
    where
        VA: LinearLayoutVerticalAlignment,
    {
        LinearLayout {
            direction: Horizontal::new(v),
            top_left: self.top_left,
        }
    }
}

impl<H: LinearLayoutHorizontalAlignment> LinearLayout<Vertical<H>> {
    pub fn with_horizontal_alignment<HA>(self, h: HA) -> LinearLayout<Vertical<HA>>
    where
        HA: LinearLayoutHorizontalAlignment,
    {
        LinearLayout {
            direction: Vertical::new(h),
            top_left: self.top_left,
        }
    }
}

impl<DIR: LayoutDirection> LinearLayout<DIR> {
    pub fn arrange(&self, views: &mut [&mut impl View]) -> Rectangle
    {
        if views.len() == 0 {
            return Rectangle::new(self.top_left, self.top_left);
        }

        let mut bounds = Size::new(0, 0);
        for view in views.iter() {
            bounds = self.direction.measure(*view, bounds);
        }

        let mut views_iter = views.iter_mut();

        let bounding_box = Rectangle::new(self.top_left, self.top_left + bounds - Size::new(1, 1));

        if let Some(first_view) = views_iter.next() {
            self.direction.layout_first(*first_view, &bounding_box);
            let mut previous_view = first_view;

            for view in views_iter {
                self.direction.layout(*view, &bounding_box, *previous_view);
                previous_view = view;
            }
        }

        bounding_box
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let mut views: [&mut Rectangle; 0] = [];

        let bounding = LinearLayout::horizontal(Point::new(-5, -5)).arrange(&mut views);

        assert_eq!(RectExt::size(&bounding), Size::new(1, 1));
    }

    #[test]
    fn test_vertical_left() {
        let mut r1 = Rectangle::new(Point::new(85, 55), Point::new(86, 56)); // 2x2
        let mut r2 = Rectangle::new(Point::new(5, 10), Point::new(10, 12)); // 6x3
        let mut r3 = Rectangle::new(Point::new(-5, 0), Point::new(-2, 4)); // 4x5

        let bounding =
            LinearLayout::vertical(Point::new(-5, -5)).arrange(&mut [&mut r1, &mut r2, &mut r3]);

        assert_eq!(RectExt::size(&bounding), Size::new(6, 10));

        assert_eq!(
            bounding,
            Rectangle::new(Point::new(-5, -5), Point::new(0, 4))
        );

        assert_eq!(RectExt::size(&r1), Size::new(2, 2));
        assert_eq!(r1, Rectangle::new(Point::new(-5, -5), Point::new(-4, -4)));

        assert_eq!(RectExt::size(&r2), Size::new(6, 3));
        assert_eq!(r2, Rectangle::new(Point::new(-5, -3), Point::new(0, -1)));

        assert_eq!(RectExt::size(&r3), Size::new(4, 5));
        assert_eq!(r3, Rectangle::new(Point::new(-5, 0), Point::new(-2, 4)));
    }

    #[test]
    fn test_vertical_right() {
        let mut r1 = Rectangle::new(Point::new(85, 55), Point::new(86, 56)); // 2x2
        let mut r2 = Rectangle::new(Point::new(5, 10), Point::new(10, 12)); // 6x3
        let mut r3 = Rectangle::new(Point::new(-5, 0), Point::new(-2, 4)); // 4x5

        let bounding =
            LinearLayout::vertical(Point::new(-5, -5))
                .with_horizontal_alignment(horizontal::Right)
                .arrange(&mut [&mut r1, &mut r2, &mut r3]);

        assert_eq!(RectExt::size(&bounding), Size::new(6, 10));

        assert_eq!(
            bounding,
            Rectangle::new(Point::new(-5, -5), Point::new(0, 4))
        );

        assert_eq!(RectExt::size(&r1), Size::new(2, 2));
        assert_eq!(r1, Rectangle::new(Point::new(-1, -5), Point::new(0, -4)));

        assert_eq!(RectExt::size(&r2), Size::new(6, 3));
        assert_eq!(r2, Rectangle::new(Point::new(-5, -3), Point::new(0, -1)));

        assert_eq!(RectExt::size(&r3), Size::new(4, 5));
        assert_eq!(r3, Rectangle::new(Point::new(-3, 0), Point::new(0, 4)));
    }
}
