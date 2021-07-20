use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_layout::{layout::linear::LinearLayout, prelude::*};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(64, 48));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    let display_area = display.bounding_box();

    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    LinearLayout::vertical(
        Chain::new(Text::new("Vertical", Point::zero(), text_style))
            .append(Text::new("Linear", Point::zero(), text_style))
            .append(Text::new("Layout", Point::zero(), text_style)),
    )
    .with_alignment(horizontal::Center)
    .arrange()
    .align_to(&display_area, horizontal::Center, vertical::Center)
    .draw(&mut display)
    .unwrap();

    Window::new("LinearLayout exmaple", &output_settings).show_static(&display);
    Ok(())
}
