use crate::VerticalAlignment;
use embedded_graphics::geometry::{Dimensions, Point};

pub struct Center;

impl VerticalAlignment for Center {
    fn align(&self, what: &impl Dimensions, reference: &impl Dimensions) -> Point {
        let center_what = (what.top_left().y + what.bottom_right().y) / 2;
        let center_ref = (reference.top_left().y + reference.bottom_right().y) / 2;

        Point::new(center_ref - center_what, 0)
    }
}
