//! Layouts - arrange multiple views
//!
//! A layout is a piece of code that arranges views on the display according to rules of the
//! particular layout algorithm.
//!
//! # Implementing layouts
//!
//! In embedded-layout, layout objects aren't special in any way. There is no `Layout` trait to
//! implement, no required API to provide. Instead, as a developer, you may decide how to make your
//! layout as comfortable as possible.
//!
//! A layout works on a group of [`View`] objects, in the form of a [`ViewGroup`]. The `ViewGroup`
//! provides an interface to access view objects of any type.
//!
//! As an example, a (admittedly not very useful) layout that aligns all views in a `ViewGroup` to
//! the `(0, 0)` point, would look something like this:
//!
//! ```rust
//! # use embedded_layout::prelude::*;
//! use embedded_layout::view_group::ViewGroup;
//!
//! struct MyLayout;
//!
//! impl MyLayout {
//!    pub fn arrange(view_group: &mut impl ViewGroup) {
//!        for idx in 0..view_group.len() {
//!            let view = view_group.at_mut(idx);
//!            let by = -view.bounds().top_left;
//!            // Because `ViewGroup` returns a `&[mut] dyn View`, we need to call the object-safe
//!            // version of `translate`.
//!            view.translate_impl(by);
//!        }
//!    }
//! }
//! ```
//!
//! As you can see, you can choose not to take ownership of the `ViewGroup`. If you do, make sure
//! you also implement `View` and `Drawable` (you probably don't want to implement `ViewGroup`) for
//! your layout object and you provide a way to retrieve the original `ViewGroup` object!
//!
//! For a more (but not really) complex example, you may check the source of [`LinearLayout`].
//!
//! [`View`]: crate::View
//! [`ViewGroup`]: crate::view_group::ViewGroup
//! [`LinearLayout`]: crate::layout::linear::LinearLayout

pub mod linear;
