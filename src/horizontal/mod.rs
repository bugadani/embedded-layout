use crate::HorizontalAlignment;
use embedded_graphics::geometry::{Dimensions, Point};

pub struct Center;

impl HorizontalAlignment for Center {
    fn align(&self, what: &impl Dimensions, reference: &impl Dimensions) -> Point {
        let center_what = (what.top_left().x + what.bottom_right().x) / 2;
        let center_ref = (reference.top_left().x + reference.bottom_right().x) / 2;

        Point::new(center_ref - center_what, 0)
    }
}
