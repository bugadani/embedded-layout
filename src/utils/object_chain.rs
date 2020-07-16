//! Utilities to create chains of objects with different types
//!
//! In general, the chain starts (or ends, depending on your view) with a `Guard` element
//! and is built up from `Link`s that contain objects. This basic structure only allows you
//! to query the number of elements, but you can implement a more useful trait for both `Link` and
//! `Guard` to make this structure more useful.

/// A generic chain element
pub trait ChainElement: Sized {
    /// `true` if this chain element marks the end of a chain
    const IS_TERMINATOR: bool;

    /// Return the number of objects linked to this chain element
    fn count() -> u32;

    /// Append an object to the chain

    fn append<T>(self, item: T) -> Link<T, Self> {
        Link {
            object: item,
            next: self,
        }
    }
}

/// This piece of the chain contains some object
pub struct Link<V, C: ChainElement = Guard> {
    /// The current object
    pub object: V,

    /// The rest of the object chain
    pub next: C,
}

impl<V, VC: ChainElement> ChainElement for Link<V, VC> {
    const IS_TERMINATOR: bool = false;

    #[inline]
    fn count() -> u32 {
        1 + VC::count()
    }
}

/// This piece marks the end of a chain
pub struct Guard;

impl ChainElement for Guard {
    const IS_TERMINATOR: bool = true;

    #[inline]
    fn count() -> u32 {
        0
    }
}

/// Helper macro to make working with object chains easier
#[macro_export]
macro_rules! chain {
    () => {Guard};
    ($x:tt) => {Link<$x, Guard>};
    ($x:tt, $($rest:tt),*) => {Link<$x, chain! { $($rest),* }>};
}

#[cfg(test)]
mod test {
    use super::*;
    struct CompileTest {
        empty_chain: chain! {},
        chain1: chain! {
            u8
        },
        chain: chain! {
            u8, u16, u32
        },
    }

    #[test]
    pub fn test() {
        fn f(_obj_chain: &chain! {u8, u16, u32}) {}

        let test = CompileTest {
            empty_chain: Guard,
            chain1: Guard.append(0),
            chain: Guard.append(0).append(1).append(2),
        };

        f(&test.chain);
    }
}
