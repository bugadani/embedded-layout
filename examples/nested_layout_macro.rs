use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Styled, Triangle},
    text::Text,
};
use embedded_layout::{
    layout::linear::{spacing::Tight, Horizontal, LinearLayout, Vertical},
    prelude::{horizontal::Center, vertical::Bottom, *},
};
use embedded_layout_macros::ViewGroup;

// We need to make our Layout generic over the pixel color, because `derive(ViewGroup)` implements
// `Drawable<C>` only if the struct has a PixelColor type parameter.
#[derive(ViewGroup)]
struct Layout<'txt, C: PixelColor> {
    layout: LinearLayout<
        Vertical<Center, Tight>,
        chain! {
            Text<'txt, MonoTextStyle<'static, C>>,
            LinearLayout<Horizontal<Bottom, Tight>, chain! {
                Styled<Triangle, PrimitiveStyle<C>>,
                Styled<Circle, PrimitiveStyle<C>>
            }>,
            chain! {
                Styled<Circle, PrimitiveStyle<C>>,
                Styled<Triangle, PrimitiveStyle<C>>
            }
        },
    >,
}

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
    let layout = Layout {
        layout: LinearLayout::vertical(
            Chain::new(text)
                .append(LinearLayout::horizontal(Chain::new(triangle).append(circle)).arrange())
                .append(Chain::new(circle2).append(triangle2.align_to(
                    &circle2,
                    horizontal::Center,
                    vertical::Top,
                ))),
        )
        .with_alignment(horizontal::Center)
        .arrange(),
    };

    layout
        .align_to(&display_area, horizontal::Center, vertical::Center)
        .draw(&mut display)
        .unwrap();

    Window::new("Layout example", &output_settings).show_static(&display);
    Ok(())
}
