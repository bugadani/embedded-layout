//! ViewGroup definition and implementation for common types.

use crate::View;
use core::ops::{Index, IndexMut};

mod object_chain;

pub trait ViewGroup: View + Index<usize, Output = dyn View> + IndexMut<usize> {
    fn len(&self) -> usize;
}
