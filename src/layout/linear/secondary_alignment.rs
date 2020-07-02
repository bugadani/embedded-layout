use crate::{align::Alignment, prelude::*};

/// Secondary alignment is used to align views perpendicular to the placement axis.
///
/// For example, use `horizontal::Right` to align views to the right in a vertical linear layout.
pub trait SecondaryAlignment: Alignment {
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
    fn measure(prev_size: Size, view: &impl View) -> Size {
        max_width(prev_size, view)
    }
}
impl SecondaryAlignment for horizontal::Center {
    fn measure(prev_size: Size, view: &impl View) -> Size {
        max_width(prev_size, view)
    }
}
impl SecondaryAlignment for horizontal::Right {
    fn measure(prev_size: Size, view: &impl View) -> Size {
        max_width(prev_size, view)
    }
}
impl SecondaryAlignment for horizontal::RightToLeft {
    fn measure(prev_size: Size, view: &impl View) -> Size {
        cascading(prev_size, view)
    }
}
impl SecondaryAlignment for horizontal::LeftToRight {
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
    fn measure(prev_size: Size, view: &impl View) -> Size {
        max_height(prev_size, view)
    }
}
impl SecondaryAlignment for vertical::Center {
    fn measure(prev_size: Size, view: &impl View) -> Size {
        max_height(prev_size, view)
    }
}
impl SecondaryAlignment for vertical::Bottom {
    fn measure(prev_size: Size, view: &impl View) -> Size {
        max_height(prev_size, view)
    }
}
impl SecondaryAlignment for vertical::TopToBottom {
    fn measure(prev_size: Size, view: &impl View) -> Size {
        cascading(prev_size, view)
    }
}
impl SecondaryAlignment for vertical::BottomToTop {
    fn measure(prev_size: Size, view: &impl View) -> Size {
        cascading(prev_size, view)
    }
}
