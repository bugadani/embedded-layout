Unreleased
==================

## Removed:

 * Removed `View::translate`. Use `View::translate_mut` instead.
 * Removed embedded-graphics' types from `prelude`.
 
## Added:

 * Add `chain!` macro to simplify working with object chains
 * Include object chain types in `prelude`
 * `View::translate_mut`

## Changed:

 * **breaking** Require embedded-graphics 0.7
 * Change fields of `Link` and `ViewGroup` to be public
 * **breaking:** Changed `View::translate` to take ownership and return ownership instead of working with references
 * Change `Link` and `Guard` to be public
 * Change `LayoutElement` to be public
 * `ViewGroup` is now a trait.
 * `LinearLayout` is no longer a `ViewGroup` but takes a `ViewGroup` in `arrange()`.

0.1.0 (2020-07-08)
==================

## Added:

 * `ElementSpacing` for `LinearLayout`: distribute views in a given space or place them at a set distance from one another
 * Example that shows how to create custom views (`examples/custom_view.rs`)

## Changed:

 * The `align` module is now public
 * Secondary alignment implementations now require specifying the alignment of the first view
 * Changed what is re-exported from the `embedded-graphics` prelude. This reduces function name collisions
 * **breaking:** Renamed `layout_direction::LayoutDirection` to `orientation::Orientation`
 * **breaking:** Renamed `layout_operation::LayoutOperation` to `layout_element::LayoutElement`
 * **breaking:** `ViewLink` has been renamed to `Link` and `ChainTerminator` to `Guard`

## Fixed:

 * Fixed an issue with cascading alignments in `LinearLayout`

0.0.3 (2020-07-04)
==================

## Added:

 * `View` trait
 * `ViewGroup` struct to allow working with multiple views
 * `LinearLayout` to arrange `View`s sequentially along the horizontal or vertical axis

## Changed:

 * **breaking:** Change API to work with a reference to the reference box

0.0.2 (2020-06-26)
==================
 * Initial release

## Added:

 * New alignments: `TopToBottom`, `BottomToTop`, `LeftToRight`, `RightToLeft`
 * New method: `align_to_mut` to apply an alignment to a reference

## Changed:

 * Usability improvement: alignment mode objects can now be passed by value

0.0.1 (2020-06-23)
==================
 * Initial release
