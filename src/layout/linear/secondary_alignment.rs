use crate::{align::Alignment, prelude::*};

/// Secondary alignment is used to align views perpendicular to the placement axis.
///
/// For example, use `horizontal::Right` to align views to the right in a vertical linear layout.
pub trait SecondaryAlignment: Alignment {
    /// Return the combined `Size` of two `View`s, based on their alignment
    fn measure(prev: Size, view: &impl View) -> Size;
}

fn max_width(prev_size: Size, view: &impl View) -> Size {
    let view_size = RectExt::size(&view.bounds());

    Size::new(
        prev_size.width.max(view_size.width),
        prev_size.height + view_size.height,
    )
}

fn cascading(prev_size: Size, view: &impl View) -> Size {
    let view_size = RectExt::size(&view.bounds());

    Size::new(
        prev_size.width + view_size.width,
        prev_size.height + view_size.height,
    )
}

impl SecondaryAlignment for horizontal::Left {
    #[inline]
    fn measure(prev_size: Size, view: &impl View) -> Size {
        max_width(prev_size, view)
    }
}
impl SecondaryAlignment for horizontal::Center {
    #[inline]
    fn measure(prev_size: Size, view: &impl View) -> Size {
        max_width(prev_size, view)
    }
}
impl SecondaryAlignment for horizontal::Right {
    #[inline]
    fn measure(prev_size: Size, view: &impl View) -> Size {
        max_width(prev_size, view)
    }
}
impl SecondaryAlignment for horizontal::RightToLeft {
    #[inline]
    fn measure(prev_size: Size, view: &impl View) -> Size {
        cascading(prev_size, view)
    }
}
impl SecondaryAlignment for horizontal::LeftToRight {
    #[inline]
    fn measure(prev_size: Size, view: &impl View) -> Size {
        cascading(prev_size, view)
    }
}

fn max_height(prev_size: Size, view: &impl View) -> Size {
    let view_size = RectExt::size(&view.bounds());

    Size::new(
        prev_size.width + view_size.width,
        prev_size.height.max(view_size.height),
    )
}

impl SecondaryAlignment for vertical::Top {
    #[inline]
    fn measure(prev_size: Size, view: &impl View) -> Size {
        max_height(prev_size, view)
    }
}
impl SecondaryAlignment for vertical::Center {
    #[inline]
    fn measure(prev_size: Size, view: &impl View) -> Size {
        max_height(prev_size, view)
    }
}
impl SecondaryAlignment for vertical::Bottom {
    #[inline]
    fn measure(prev_size: Size, view: &impl View) -> Size {
        max_height(prev_size, view)
    }
}
impl SecondaryAlignment for vertical::TopToBottom {
    #[inline]
    fn measure(prev_size: Size, view: &impl View) -> Size {
        cascading(prev_size, view)
    }
}
impl SecondaryAlignment for vertical::BottomToTop {
    #[inline]
    fn measure(prev_size: Size, view: &impl View) -> Size {
        cascading(prev_size, view)
    }
}