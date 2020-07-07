//! # Example: Custom View - Progress bar
//!
//! ![Screenshot of progress bar example]( data:image/png;base64,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 )
//!
//! This example shows what's necessary to implement a reusable View.
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embedded_graphics::{
    pixelcolor::BinaryColor, primitives::Rectangle, style::PrimitiveStyle, DrawTarget,
};
use embedded_layout::{
    layout::linear::{spacing::FixedMargin, LinearLayout},
    prelude::*,
};

pub struct ProgressBar {
    progress: u32,
    bounds: Rectangle,
}
impl ProgressBar {
    /// The progress bar has a configurable position and size
    fn new(position: Point, size: Size) -> Self {
        Self {
            bounds: Rectangle::with_size(position, size),
            progress: 0,
        }
    }

    fn with_progress(self, progress: u32) -> Self {
        Self {
            bounds: self.bounds,
            progress,
        }
    }
}

/// Implementing `View` is required by the layout and alignment operations
/// `View` teaches `embedded-layout` where our object is, how big it is and how to move it.
impl View for ProgressBar {
    #[inline]
    fn translate(&mut self, by: Point) {
        self.bounds.translate(by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}

/// Need to implement `Drawable` for a _reference_ of our view
impl Drawable<BinaryColor> for &ProgressBar {
    fn draw<D: DrawTarget<BinaryColor>>(self, display: &mut D) -> Result<(), D::Error> {
        // Create styles
        let border_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let progress_style = PrimitiveStyle::with_fill(BinaryColor::On);

        // Create a 1px border
        let border = self.bounds.into_styled(border_style);

        // Create a rectangle that will indicate progress
        let mut progress = Rectangle::with_size(
            Point::zero(),
            // sizes are calculated so that the rectangle will have a 1px margin
            Size::new(
                (self.bounds.size().width - 4) * self.progress / 100,
                self.bounds.size().height - 4,
            ),
        )
        .into_styled(progress_style);

        // Align progress bar within border
        progress
            .align_to_mut(&border, horizontal::Left, vertical::Center)
            .translate(Point::new(2, 0));

        // Draw views
        border.draw(display)?;
        progress.draw(display)?;

        Ok(())
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    // Create a Rectangle from the display's dimensions
    let display_area = display.display_area();

    // Two bigger progress bars
    let progress1 = ProgressBar::new(Point::zero(), Size::new(64, 8)).with_progress(10);
    let progress2 = ProgressBar::new(Point::zero(), Size::new(64, 8)).with_progress(50);

    // Two smaller progress bars
    let progress3 = ProgressBar::new(Point::zero(), Size::new(32, 6)).with_progress(50);
    let progress4 = ProgressBar::new(Point::zero(), Size::new(32, 6)).with_progress(100);

    // Arrange on display and draw
    LinearLayout::vertical()
        .with_spacing(FixedMargin(4))
        .add_view(progress1)
        .add_view(progress2)
        .add_view(progress3)
        .add_view(progress4)
        .arrange()
        .align_to(&display_area, horizontal::Center, vertical::Center)
        .draw(&mut display)
        .unwrap();

    Window::new("Custom View example", &output_settings).show_static(&display);
    Ok(())
}
