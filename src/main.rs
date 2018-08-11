//! A simple example that demonstrates using conrod within a basic `winit` window loop, using
//! `glium` to render the `conrod::render::Primitives` to screen.

#[macro_use] extern crate conrod;

use conrod::{Positionable, Colorable, Widget};
use conrod::backend::glium::glium::{self, Surface};

widget_ids! {
    pub struct Ids {
        text,
    }
}

/// events_loop: poll events from windows
/// ui: "where" to display
/// ids: Custom struct that does countain all our widget
/// renderer: Interface between conrod's Primitives && glium's "Surface"
/// image_map should contain all images widgets. None here.
struct GuiObject {
    events_loop:    glium::glutin::EventsLoop,
    display:        glium::Display,
    ui:             conrod::Ui,
    ids:            Ids,
    renderer:       conrod::backend::glium::Renderer,
    image_map:      conrod::image::Map<glium::texture::Texture2d>,
}

impl GuiObject {
    fn new() -> GuiObject {
        const WIDTH: u32 = 400;
        const HEIGHT: u32 = 200;
        let events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title("Hello Conrod!")
            .with_dimensions((WIDTH, HEIGHT).into());
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
        let ids = Ids::new(ui.widget_id_generator());
        const FONT_PATH: &'static str =
            concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(FONT_PATH).unwrap();
        let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        GuiObject {
            events_loop:    events_loop,
            display:        display,
            ui:             ui,
            ids:            ids,
            renderer:       renderer,
            image_map:      image_map,
        }
    }

    fn update(&mut self) {
        let ui = &mut self.ui.set_widgets();

        // add widgets to screen
        conrod::widget::Text::new("Hello World!")
            .middle_of(ui.window)
            .color(conrod::color::WHITE)
            .font_size(32)
            .set(self.ids.text, ui);
    }

    /// @return: false if user asked to close windows.
    fn process_event(&mut self, event: conrod::glium::glutin::Event) -> bool {
        match event.clone() {
            glium::glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glium::glutin::WindowEvent::CloseRequested |
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => return false,
                    _ => (),
                }
            }
            _ => (),
        };

        // convert winit events to conrod events
        match conrod::backend::winit::convert_event(event, &self.display) {
            None => return true,
            Some(input) => {
                self.ui.handle_event(input);
                self.update();
            }
        }
        true
    }

    /// Get all the new events since last frame.
    /// If there are none, wait for one.
    /// @return: false if user asked to close windows.
    fn handle_winit_events(&mut self) -> bool {
        let mut events = Vec::new();

        self.events_loop.poll_events(|event| { events.push(event); });
        if events.is_empty() {
            self.events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        for event in events.drain(..) {
            if self.process_event(event) == false {
                return false;
            }
        }
        true
    }

    fn draw(&mut self) {
        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer.fill(&self.display, primitives, &self.image_map);
            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

mod feature {
    use GuiObject;

    pub fn main() {
        let mut gui = GuiObject::new();

        while gui.handle_winit_events() != false {
            gui.draw();
        }
    }
}

fn main() {
    feature::main();
}
