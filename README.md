embedded-layout
===============

`embedded-layout` extends [`embedded-graphics`] with basic layout functions.

**Note:** This library is currently highly experimental. Expect API breakage with every update.

## Example

The examples are based on [the embedded-graphics simulator](https://github.com/jamwaffles/embedded-graphics/tree/master/simulator). The simulator is built on top of `SDL2`. If you don't have that installed, set the `EG_SIMULATOR_DUMP="screenshot.png"` environment variable so that running the examples produce a screenshot image instead of a window.

### Draw some text to the center of the display

```rust
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
};
use embedded_layout::prelude::*;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(129, 129));

    // Create a Rectangle from the display's dimensions
    let display_area = display.display_area();

    let text_style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .build();

    Text::new("Hello, World!", Point::zero())
        .into_styled(text_style)
        // align text to the display
        .align_to(&display_area, horizontal::Center, vertical::Center)
        .draw(&mut display)
        .unwrap();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);
    Ok(())
}
```

### Use `LinearLayout` to arrange multiple objects

![LinearLayout example](assets/linear_layout.png)

```rust
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
};
use embedded_layout::layout::linear::LinearLayout;
use embedded_layout::prelude::*;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(64, 48));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    let display_area = display.display_area();

    let text_style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .build();

    LinearLayout::vertical()
        .with_alignment(horizontal::Center)
        .add_view(Text::new("Vertical", Point::zero()).into_styled(text_style))
        .add_view(Text::new("Linear", Point::zero()).into_styled(text_style))
        .add_view(Text::new("Layout", Point::zero()).into_styled(text_style))
        .arrange()
        .align_to(&display_area, horizontal::Center, vertical::Center)
        .draw(&mut display)
        .unwrap();

    Window::new("LinearLayout exmaple", &output_settings).show_static(&display);
    Ok(())
}
```

[`embedded-graphics`]: https://github.com/jamwaffles/embedded-graphics/
