//! Utilities to create chains of objects with different types
//!
//! In general, the chain starts (or ends, depending on your view) with a `Guard` element
//! and is built up from `Link`s that contain objects. This basic structure only allows you
//! to query the number of elements, but you can implement a more useful trait for both `Link` and
//! `Guard` to make this structure more useful.

mod private {
    pub trait Sealed {}

    impl<V> Sealed for super::Tail<V> {}
    impl<V, C: super::ChainElement> Sealed for super::Link<V, C> {}
}

/// A generic chain element
pub trait ChainElement: Sized + private::Sealed {
    /// Return the number of objects linked to this chain element
    fn count(&self) -> usize;

    /// Append an object to the chain
    #[inline]
    fn append<T>(self, item: T) -> Link<T, Self> {
        Link {
            object: item,
            next: self,
        }
    }
}

/// This piece of the chain contains some object
pub struct Link<V, C: ChainElement> {
    /// The current object
    pub object: V,

    /// The rest of the object chain
    pub next: C,
}

impl<V, VC> ChainElement for Link<V, VC>
where
    VC: ChainElement,
{
    #[inline]
    fn count(&self) -> usize {
        self.next.count() + 1
    }
}

/// This piece marks the end of a chain
pub struct Tail<V> {
    pub object: V,
}

impl<V> Tail<V> {
    pub const fn new(object: V) -> Self {
        Self { object }
    }
}

impl<V> ChainElement for Tail<V> {
    #[inline]
    fn count(&self) -> usize {
        1
    }
}

/// Internal implementation of chain macro
#[doc(hide)]
#[macro_export(local_inner_macros)]
macro_rules! chain_impl {
    ($x:ty) => {
        Tail<$x>
    };
    ($x:ty,) => {
        Tail<$x>
    };
    ($x:ty, $($rest:tt)+) => {
        Link<$x, chain_impl! { $($rest)+ }>
    };
}

/// Reverse the argument list to generate object chain
#[doc(hide)]
#[macro_export(local_inner_macros)]
macro_rules! reverse {
    ([] $($reversed:tt)+) => {
        chain_impl! { $($reversed)+ }
    };
    ([$first:ty] $($reversed:tt)*) => {
        reverse! { [ ] $first, $($reversed)* }
    };
    ([$first:ty, $($rest:ty),*] $($reversed:tt)*) => {
        reverse! { [ $($rest),* ] $first, $($reversed)* }
    };
}

/// Helper macro to make working with object chains easier
///
/// Using this macro reduces the boilerplate required to describe the type of an object chain
///
/// # Example:
///
/// Instead of writing this...
///
/// ```
/// use embedded_layout::prelude::*;
/// use embedded_graphics::primitives::{Circle, Rectangle, Triangle};
/// type Chain = Link<Rectangle, Link<Circle, Link<Triangle, Guard>>>;
/// ```
///
/// ... the `chain!` macro allows you to write this:
///
/// ```
/// use embedded_layout::prelude::*;
/// use embedded_graphics::primitives::{Circle, Rectangle, Triangle};
/// type Chain = chain! { Triangle, Circle, Rectangle };
/// ```
///
/// Note also how the order of types follows the type of objects in the chain instead of being
/// reversed.
#[macro_export(local_inner_macros)]
macro_rules! chain {
    ( $($types:ty),+ ) => {
        reverse!{ [ $($types),+ ] }
    };
}

#[cfg(test)]
mod test {
    #![allow(dead_code)]
    use super::*;
    use core::marker::PhantomData;

    struct CompileTest {
        chain1: chain! {
            u8
        },
        generic_in_chain: chain! {
            Generic<'static, u32>
        },
        chain: chain! {
            u8, u16, u32
        },
    }

    struct Generic<'a, T> {
        field: PhantomData<&'a T>,
    }

    #[test]
    pub fn test() {
        fn f(_obj_chain: &chain! {u8, u16, u32}) {}

        let test = CompileTest {
            chain1: Tail::new(0),
            generic_in_chain: Tail::new(Generic { field: PhantomData }),
            chain: Tail::new(0u8).append(1u16).append(2u32),
        };

        f(&test.chain);
    }

    #[test]
    pub fn test_count() {
        assert_eq!(1, Tail::new(0).count());
        assert_eq!(3, Tail::new(0u8).append(1u16).append(2u32).count());
    }
}
