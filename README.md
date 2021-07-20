embedded-layout
===============

`embedded-layout` extends [`embedded-graphics`] with basic layout functions.

`embedded-layout` consists of three main parts:
 - alignments that can be used to position two objects relative to one another
   * `horizontal`
     * `NoAlignment`, `Left`, `Right`, `Center`
     * `LeftToRight`, `RightToLeft`
   * `vertical`
     * `NoAlignment`, `Top`, `Bottom`, `Center`
     * `TopToBottom`, `BottomToTop`
 - layouts that can be used to arrange multiple views
   * `LinearLayout`
 - view groups which are collections of view objects
   * `Chain` to create ad-hoc collections (can hold views of different types)
   * `Views` to create view groups from arrays and slices (can only hold views of a single  type)
   * `derive(ViewGroup)` to turn any plain old Rust struct into a view group

## Example

The examples are based on [the embedded-graphics simulator]. The simulator is built on top of
`SDL2`. See the [simulator README] for more information.

![embedded-layout example](assets/nested-layout-example.png)

```rust
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Triangle},
    text::Text,
};
use embedded_layout::{layout::linear::LinearLayout, prelude::*};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    // Create a Rectangle from the display's dimensions
    let display_area = display.bounding_box();

    // Style objects
    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let fill_on = PrimitiveStyle::with_fill(BinaryColor::On);
    let fill_off = PrimitiveStyle::with_fill(BinaryColor::Off);

    // Primitives to be displayed
    let triangle = Triangle::new(Point::new(0, 0), Point::new(12, 0), Point::new(6, 12))
        .into_styled(thin_stroke);

    let circle = Circle::new(Point::zero(), 11).into_styled(thick_stroke);
    let circle2 = Circle::new(Point::zero(), 15).into_styled(fill_on);
    let triangle2 =
        Triangle::new(Point::new(0, 0), Point::new(10, 0), Point::new(5, 8)).into_styled(fill_off);
    let text = Text::new("embedded-layout", Point::zero(), text_style);

    // The layout
    LinearLayout::vertical(
        Chain::new(text)
            .append(LinearLayout::horizontal(Chain::new(triangle).append(circle)).arrange())
            .append(
                Chain::new(triangle2.align_to(&circle2, horizontal::Center, vertical::Top))
                    .append(circle2),
            ),
    )
    .with_alignment(horizontal::Center)
    .arrange()
    .align_to(&display_area, horizontal::Center, vertical::Center)
    .draw(&mut display)
    .unwrap();

    Window::new("Layout example", &output_settings).show_static(&display);
    Ok(())
}
```

## Development setup

### Minimum supported Rust version
The minimum supported Rust version for embedded-layout is 1.40.0 or greater. However, the documentation uses the `intra-crate links` feature which requires nightly Rust. Ensure you have the latest stable version of Rust installed, preferably through https://rustup.rs.

### Installation

For setup in general, follow the installation instructions for [`embedded-graphics`].

To install SDL2 on Windows, see https://github.com/Rust-SDL2/rust-sdl2#windows-msvc

[`embedded-graphics`]: https://github.com/jamwaffles/embedded-graphics/
[the embedded-graphics simulator]: https://github.com/jamwaffles/embedded-graphics/tree/master/simulator
[simulator README]: https://github.com/jamwaffles/embedded-graphics/tree/master/simulator#usage-without-sdl2
