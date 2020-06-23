embedded-layout
===============

`embedded-layout` extends [`embedded-graphics`] with basic layout functions.

## Example

Draw some text to the center of the display:

```rust
use embedded_layout::prelude::*;
use embedded_graphics::{
    prelude::*,
    fonts::{Font6x8, Text},
    geometry::Point,
    primitives::Rectangle,
    pixelcolor::BinaryColor,
    style::TextStyleBuilder,
};

let display_area = disp.display_area();

let text_style = TextStyleBuilder::new(Font6x8)
                        .text_color(BinaryColor::On)
                        .build();

Text::new("Hello, world!", Point::zero())
     .into_styled(text_style)
     .align_to(display_area, horizontal::Center, vertical::Center)
     .draw(&mut disp)
     .unwrap();
```

[`embedded-graphics`]: https://github.com/jamwaffles/embedded-graphics/
