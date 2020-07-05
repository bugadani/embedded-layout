//! Utilities to create chains of objects with different types
//!
//! In general, the chain starts (or ends, depending on your view) with a `Guard` element
//! and is built up from `Link`s that contain objects. This basic structure only allows you
//! to query the number of elements, but you can implement a more useful trait for both `Link` and
//! `Guard` to make this structure more useful.

/// A generic chain element
pub trait ChainElement {
    /// `true` if this chain element marks the end of a chain
    const IS_TERMINATOR: bool;

    /// Return the number of objects linked to this chain element
    fn count() -> usize;
}

/// This piece of the chain contains some object
pub struct Link<V, C: ChainElement> {
    pub(crate) object: V,
    pub(crate) next: C,
}

impl<V, VC: ChainElement> ChainElement for Link<V, VC> {
    const IS_TERMINATOR: bool = false;

    #[inline]
    fn count() -> usize {
        1 + VC::count()
    }
}

/// This piece marks the end of a chain
pub struct Guard;

impl ChainElement for Guard {
    const IS_TERMINATOR: bool = true;

    #[inline]
    fn count() -> usize {
        0
    }
}
