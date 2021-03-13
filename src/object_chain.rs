//! Create static chains of objects with different types.
//!
//! In general, the chain starts (or ends, depending on your view) with a `Chain` element
//! and is built up from any number of `Link`s. This basic structure only allows you
//! to query the number of elements, but you can implement a more useful trait for both `Link` and
//! `Chain` to make this structure more useful.

mod private {
    pub trait Sealed {}

    impl<V> Sealed for super::Chain<V> {}
    impl<V, C: super::ChainElement> Sealed for super::Link<V, C> {}
}

/// A generic chain element
pub trait ChainElement: Sized + private::Sealed {
    /// Return the number of objects linked to this chain element
    fn count(&self) -> usize;
}

/// This piece of the chain contains some object
pub struct Link<V, C: ChainElement> {
    /// The current object
    pub object: V,

    /// The rest of the object chain
    pub parent: C,
}

impl<V, C: ChainElement> Link<V, C> {
    /// Append an object to the chain
    #[inline]
    pub fn append<T>(self, item: T) -> Link<T, Self> {
        Link {
            object: item,
            parent: self,
        }
    }
}

impl<V, VC> ChainElement for Link<V, VC>
where
    VC: ChainElement,
{
    #[inline]
    fn count(&self) -> usize {
        self.parent.count() + 1
    }
}

/// This piece marks the end of a chain
pub struct Chain<V> {
    /// The wrapped object.
    pub object: V,
}

impl<V> Chain<V> {
    /// Append an object to the chain
    #[inline]
    pub fn append<T>(self, item: T) -> Link<T, Self> {
        Link {
            object: item,
            parent: self,
        }
    }
}

impl<V> Chain<V> {
    /// Create a new [`Chain`] by wrapping the given object.
    pub const fn new(object: V) -> Self {
        Self { object }
    }
}

impl<V> ChainElement for Chain<V> {
    #[inline]
    fn count(&self) -> usize {
        1
    }
}

/// Internal implementation of chain macro
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! chain_impl {
    ($x:ty) => {
        Chain<$x>
    };
    ($x:ty,) => {
        Chain<$x>
    };
    ($x:ty, $($rest:tt)+) => {
        Link<$x, chain_impl! { $($rest)+ }>
    };
}

/// Reverse the argument list to generate object chain
#[doc(hidden)]
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
/// ```rust
/// use embedded_layout::prelude::*;
/// use embedded_graphics::primitives::{Circle, Rectangle, Triangle};
/// type Views = Link<Rectangle, Link<Circle, Chain<Triangle>>>;
/// ```
///
/// ... the `chain!` macro allows you to write this:
///
/// ```rust
/// use embedded_layout::prelude::*;
/// use embedded_graphics::primitives::{Circle, Rectangle, Triangle};
/// type Views = chain! { Triangle, Circle, Rectangle };
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
            chain1: Chain::new(0),
            generic_in_chain: Chain::new(Generic { field: PhantomData }),
            chain: Chain::new(0u8).append(1u16).append(2u32),
        };

        f(&test.chain);
    }

    #[test]
    pub fn test_count() {
        assert_eq!(1, Chain::new(0).count());
        assert_eq!(3, Chain::new(0u8).append(1u16).append(2u32).count());
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test_macro {
    use embedded_graphics::primitives::{Rectangle, Triangle};

    use crate::prelude::*;

    type Views = chain! {
        Rectangle,
        Rectangle,
        Triangle
    };
}
