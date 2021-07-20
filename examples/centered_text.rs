use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_layout::prelude::*;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(129, 129));

    // Create a Rectangle from the display's dimensions
    let display_area = display.bounding_box();

    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    Text::new("Hello, World!", Point::zero(), text_style)
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
