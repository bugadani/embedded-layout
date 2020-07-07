use crate::{align::Alignment, prelude::*};

/// Secondary alignment is used to align views perpendicular to the placement axis.
///
/// For example, use [`horizontal::Right`] to align views to the right in a vertical linear layout.
///
/// `SecondaryAlignment` should be implemented by custom `Alignment` types, otherwise they won't be
/// compatible with [`LinearLayout`].
///
/// [`LinearLayout`]: crate::layout::linear::LinearLayout
pub trait SecondaryAlignment: Alignment {
    /// The secondary alignment of the first view
    type First: Alignment;

    /// Return the combined `Size` occupied by both `Views` after they are arranged.
    ///
    /// I.e. [`horizontal::Left`] returns the maximum width, while [`horizontal::LeftToRight`]
    /// returns the sum of the two widths.
    fn measure(prev: Size, view_size: Size) -> Size;
}

fn max_width(prev_size: Size, view_size: Size) -> Size {
    Size::new(
        prev_size.width.max(view_size.width),
        prev_size.height + view_size.height,
    )
}

const fn cascading(prev_size: Size, view_size: Size) -> Size {
    Size::new(
        prev_size.width + view_size.width,
        prev_size.height + view_size.height,
    )
}

impl SecondaryAlignment for horizontal::Left {
    type First = horizontal::Left;
    #[inline]
    fn measure(prev_size: Size, view_size: Size) -> Size {
        max_width(prev_size, view_size)
    }
}
impl SecondaryAlignment for horizontal::Center {
    type First = horizontal::Center;
    #[inline]
    fn measure(prev_size: Size, view_size: Size) -> Size {
        max_width(prev_size, view_size)
    }
}
impl SecondaryAlignment for horizontal::Right {
    type First = horizontal::Right;
    #[inline]
    fn measure(prev_size: Size, view_size: Size) -> Size {
        max_width(prev_size, view_size)
    }
}
impl SecondaryAlignment for horizontal::RightToLeft {
    type First = horizontal::Right;
    #[inline]
    fn measure(prev_size: Size, view_size: Size) -> Size {
        cascading(prev_size, view_size)
    }
}
impl SecondaryAlignment for horizontal::LeftToRight {
    type First = horizontal::Left;
    #[inline]
    fn measure(prev_size: Size, view_size: Size) -> Size {
        cascading(prev_size, view_size)
    }
}

fn max_height(prev_size: Size, view_size: Size) -> Size {
    Size::new(
        prev_size.width + view_size.width,
        prev_size.height.max(view_size.height),
    )
}

impl SecondaryAlignment for vertical::Top {
    type First = vertical::Top;
    #[inline]
    fn measure(prev_size: Size, view_size: Size) -> Size {
        max_height(prev_size, view_size)
    }
}
impl SecondaryAlignment for vertical::Center {
    type First = vertical::Center;
    #[inline]
    fn measure(prev_size: Size, view_size: Size) -> Size {
        max_height(prev_size, view_size)
    }
}
impl SecondaryAlignment for vertical::Bottom {
    type First = vertical::Bottom;
    #[inline]
    fn measure(prev_size: Size, view_size: Size) -> Size {
        max_height(prev_size, view_size)
    }
}
impl SecondaryAlignment for vertical::TopToBottom {
    type First = vertical::Top;
    #[inline]
    fn measure(prev_size: Size, view_size: Size) -> Size {
        cascading(prev_size, view_size)
    }
}
impl SecondaryAlignment for vertical::BottomToTop {
    type First = vertical::Bottom;
    #[inline]
    fn measure(prev_size: Size, view_size: Size) -> Size {
        cascading(prev_size, view_size)
    }
}
