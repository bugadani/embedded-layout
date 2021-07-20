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
use embedded_layout_macros::ViewGroup;

// We need to make our Layout generic over the pixel color, because `derive(ViewGroup)` implements
// `Drawable<C>` only if the struct has a PixelColor type parameter.
#[derive(ViewGroup)]
struct Layout<'txt, C: PixelColor> {
    text_vertical: Text<'txt, MonoTextStyle<'static, C>>,
    text_linear: Text<'txt, MonoTextStyle<'static, C>>,
    text_layout: Text<'txt, MonoTextStyle<'static, C>>,
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(64, 48));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    let display_area = display.bounding_box();

    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    let views = Layout {
        text_vertical: Text::new("Vertical", Point::zero(), text_style),
        text_linear: Text::new("Linear", Point::zero(), text_style),
        text_layout: Text::new("Layout", Point::zero(), text_style),
    };

    LinearLayout::vertical(views)
        .with_alignment(horizontal::Center)
        .arrange()
        .align_to(&display_area, horizontal::Center, vertical::Center)
        .draw(&mut display)
        .unwrap();

    Window::new("LinearLayout exmaple", &output_settings).show_static(&display);
    Ok(())
}
