0.4.0 (2023-10-10)
==================

## New

* `LinearLayout` now implements `ViewGroup`
* `LinearLayout::inner{_mut}`
* `LinearLayout::arrange_view_group`
* `Orientation::compute_offset`
* `ViewGroup::bounds_of`
* `ViewGroup::translate_child`

## Changed

* `Orientation::place` now returns the bounds of the placed View.

0.3.2 (2023-08-23)
==================

## New

* `ViewGroup` macro can now be used on enums
* `ViewGroup` macro now supports empty enum variants and unit structs
* `LinearLayout` can now be used in macro-generated view groups

## Fixed

* `embedded-layout` no longer panicks when accessing an out-of-bounds view in a `ViewGroup`
* Fixed aligning to a 0-sized reference object

0.3.1 (2023-05-14)
==================

## Changed

* Update `embedded-layout-macros` crate dependency
* Re-export the `ViewGroup` derive macro

0.3.0 (2023-05-14)
==================

## Changed

* **(breaking)** Update to `embedded-graphics` 0.8
* **(breaking)** Bump Minimum Supported Rust Version (MSRV) to 1.61.

0.2.0 (2021-07-20)
==================

## Added

* Add `chain!` macro to simplify working with object chains
* Include object chain types in `prelude`
* `View::translate_mut`
* `derive(ViewGroup)` to easily implmenet ViewGroup on a structure with named fields.
* `view_group::Views` which can wrap a slice of View objects into a ViewGroup.
* `LinearLayout::into_inner()`
* `LinearLayout` now has a position.

## Changed

* **breaking** Update to `embedded-graphics` 0.7
* **breaking** Changed `ViewGroup` to a trait. Layouts now operate on objects that implement `ViewGroup`.
* **breaking** LinearLayout now requires a view in their constructors.
* **breaking** Changed `View::translate` to take ownership and return ownership instead of working with references
* Changed `Link` to be public. Replaced private `Guard` with public `Chain` which now wraps an object.
* **breaking** Empty object chains are no longer possible.
* **breaking** Renamed `ChainElement::count()` to `ChainElement::len()` for consistency.

## Removed

* Removed the `ChainElement` trait from `prelude`.
* Removed `embedded-graphics` types from `prelude`.
* Removed most methods of `RectExt`.
* Removed `DisplayArea` extension trait.

0.1.0 (2020-07-08)
==================

## Added

* `ElementSpacing` for `LinearLayout`: distribute views in a given space or place them at a set distance from one another
* Example that shows how to create custom views (`examples/custom_view.rs`)

## Changed

* The `align` module is now public
* Secondary alignment implementations now require specifying the alignment of the first view
* Changed what is re-exported from the `embedded-graphics` prelude. This reduces function name collisions
* **breaking:** Renamed `layout_direction::LayoutDirection` to `orientation::Orientation`
* **breaking:** Renamed `layout_operation::LayoutOperation` to `layout_element::LayoutElement`
* **breaking:** `ViewLink` has been renamed to `Link` and `ChainTerminator` to `Guard`

## Fixed

* Fixed an issue with cascading alignments in `LinearLayout`

0.0.3 (2020-07-04)
==================

## Added

* `View` trait
* `ViewGroup` struct to allow working with multiple views
* `LinearLayout` to arrange `View`s sequentially along the horizontal or vertical axis

## Changed

* **breaking:** Change API to work with a reference to the reference box

0.0.2 (2020-06-26)
==================

* Initial release

## Added

* New alignments: `TopToBottom`, `BottomToTop`, `LeftToRight`, `RightToLeft`
* New method: `align_to_mut` to apply an alignment to a reference

## Changed

* Usability improvement: alignment mode objects can now be passed by value

0.0.1 (2020-06-23)
==================

* Initial release
