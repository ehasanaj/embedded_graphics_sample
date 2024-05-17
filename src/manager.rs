use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, StrokeAlignment},
    text::Alignment,
};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};

use crate::{
    renderer::Renderer,
    InfalliableResult,
};

pub struct Manager<'a> {
    display: SimulatorDisplay<BinaryColor>,
    window: Window,
    character_style: MonoTextStyle<'a, BinaryColor>,
    pub renderer: Renderer<'a, BinaryColor>,
}

impl<'a> Manager<'a> {
    pub fn new(title: &str, width: u32, height: u32) -> Manager<'a> {
        let display = SimulatorDisplay::<BinaryColor>::new(Size::new(width, height));

        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledBlue)
            .build();
        let window = Window::new(title, &output_settings);

        let border_stroke_3 = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(3)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();

        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

        let renderer = Renderer::new();
        renderer.add_rectangle(display.bounding_box(), border_stroke_3);

        Manager {
            display,
            window,
            character_style,
            renderer,
        }
    }

    fn update_display<F: Fn(&mut Manager) -> InfalliableResult>(
        &mut self,
        dynamic_drawer: F,
    ) -> InfalliableResult {
        self.display.clear(BinaryColor::Off)?;

        dynamic_drawer(self)?;

        self.renderer.render(&mut self.display)?;
        self.window.update(&self.display);

        Ok(())
    }

    pub fn run<F: Fn(&mut Manager) -> InfalliableResult>(
        &mut self,
        dynamic_drawer: F,
    ) -> InfalliableResult {
        'running: loop {
            self.update_display(&dynamic_drawer)?;
            for event in self.window.events() {
                match event {
                    SimulatorEvent::Quit => break 'running,
                    SimulatorEvent::KeyUp {
                        keycode: Keycode::Space,
                        keymod: _,
                        repeat: false,
                    } => {
                        let position: Point =
                            self.display.bounding_box().center() + Point::new(0, -13);
                        self.renderer.add_text_with_alignment(
                            "Hello, Embedded",
                            position,
                            self.character_style,
                            Alignment::Center,
                        );
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn character_style(&self) -> MonoTextStyle<'a, BinaryColor> {
        self.character_style
    }

    pub fn bounding_box(&self) -> Rectangle {
        self.display.bounding_box()
    }

    pub fn display(&mut self) -> &mut SimulatorDisplay<BinaryColor> {
        &mut self.display
    }
}
