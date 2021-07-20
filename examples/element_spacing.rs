//! This example is an approximate reimplementation of the `embedded-graphics` Hello, World! example
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle, Triangle},
    text::Text,
};
use embedded_layout::{
    layout::linear::{
        spacing::{DistributeFill, FixedMargin},
        LinearLayout,
    },
    prelude::*,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    let display_area = display.bounding_box();

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    // Create the view objects
    let text = Text::new("embedded-layout", Point::zero(), text_style);
    let triangle = Triangle::new(Point::new(0, 16), Point::new(16, 16), Point::new(8, 0))
        .into_styled(thin_stroke);
    let rectangle = Rectangle::new(Point::zero(), Size::new(17, 17)).into_styled(fill);
    let circle = Circle::new(Point::zero(), 16).into_styled(thick_stroke);

    // Draw a 3px wide outline around the display.
    display_area
        .into_styled(thick_stroke)
        .draw(&mut display)
        .unwrap();

    // Lay out and draw the views
    LinearLayout::vertical(
        Chain::new(
            LinearLayout::horizontal(Chain::new(triangle).append(rectangle).append(circle))
                .with_spacing(DistributeFill(text.size().width))
                .arrange(),
        )
        .append(text),
    )
    .with_spacing(FixedMargin(10))
    .arrange()
    .align_to(&display_area, horizontal::Center, vertical::Center)
    .draw(&mut display)
    .unwrap();

    Window::new("Hello, element spacing!", &output_settings).show_static(&display);
    Ok(())
}
