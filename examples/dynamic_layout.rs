use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{PixelColor, Point, Size},
    primitives::{Circle, Primitive, PrimitiveStyle, Rectangle, Styled, Triangle},
    Drawable,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use embedded_layout::view_group::Views;
use embedded_layout_macros::ViewGroup;

#[derive(ViewGroup)]
enum LayoutViews<C: PixelColor> {
    TriangleView(Styled<Triangle, PrimitiveStyle<C>>),
    CircleView(Styled<Circle, PrimitiveStyle<C>>),
    RectangleView(Styled<Rectangle, PrimitiveStyle<C>>),
    CombinedView(
        Styled<Circle, PrimitiveStyle<C>>,
        Styled<Rectangle, PrimitiveStyle<C>>,
    ),
    CombinedViewStruct {
        circle: Styled<Circle, PrimitiveStyle<C>>,
        square: Styled<Rectangle, PrimitiveStyle<C>>,
    },
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(100, 50));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    let mut views = Vec::new();

    (0..5).for_each(|col_idx| {
        (0..2).for_each(|row_idx| {
            let view = match (col_idx + (row_idx * 5)) % 5 {
                0 => {
                    let p1 = Point::new(col_idx * 20, row_idx * 30);
                    let p2 = Point::new((col_idx + 1) * 20, row_idx * 30);
                    let p3 = Point::new((col_idx * 20) + 10, 20 + (row_idx * 30));

                    LayoutViews::TriangleView(
                        Triangle::new(p1, p2, p3)
                            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1)),
                    )
                }
                1 => LayoutViews::CircleView(
                    Circle::new(Point::new(col_idx * 20, row_idx * 30), 20)
                        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1)),
                ),
                2 => LayoutViews::RectangleView(
                    Rectangle::new(Point::new(col_idx * 20, row_idx * 30), Size::new(20, 20))
                        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1)),
                ),
                3 => {
                    let circle = Circle::new(Point::new(col_idx * 20, row_idx * 30), 20)
                        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
                    let square =
                        Rectangle::new(Point::new(col_idx * 20, row_idx * 30), Size::new(20, 20))
                            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));

                    LayoutViews::CombinedView(circle, square)
                }
                4 => {
                    let circle = Circle::new(Point::new(col_idx * 20, row_idx * 30), 20)
                        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
                    let square =
                        Rectangle::new(Point::new(col_idx * 20, row_idx * 30), Size::new(20, 20))
                            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));

                    LayoutViews::CombinedViewStruct { circle, square }
                }
                _ => panic!("Out of bounds number"),
            };

            views.push(view);
        });
    });

    Views::new(views.as_mut_slice()).draw(&mut display).unwrap();

    Window::new("Dynamic Layout Example", &output_settings).show_static(&display);
    Ok(())
}
