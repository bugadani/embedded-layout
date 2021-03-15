use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    style::{Styled, TextStyle, TextStyleBuilder},
};
use embedded_layout::{layout::linear::LinearLayout, prelude::*};
use embedded_layout_macros::ViewGroup;

// We need to make our Layout generic over the pixel color, because `derive(ViewGroup)` implements
// `Drawable<C>` only if the struct has a PixelColor type parameter.
#[derive(ViewGroup)]
struct Layout<'txt, C: PixelColor> {
    text_vertical: Styled<Text<'txt>, TextStyle<C, Font6x8>>,
    text_linear: Styled<Text<'txt>, TextStyle<C, Font6x8>>,
    text_layout: Styled<Text<'txt>, TextStyle<C, Font6x8>>,
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(64, 48));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    let display_area = display.display_area();

    let text_style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .build();

    let views = Layout {
        text_vertical: Text::new("Vertical", Point::zero()).into_styled(text_style),
        text_linear: Text::new("Linear", Point::zero()).into_styled(text_style),
        text_layout: Text::new("Layout", Point::zero()).into_styled(text_style),
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
