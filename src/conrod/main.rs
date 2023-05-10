use conrod::{
    glium::{self, Surface},
    widget, widget_ids, Colorable, Positionable, Widget,
};
extern crate find_folder;

pub fn main() {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Hello Conrod")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    widget_ids!(struct Ids { text });
    let ids = Ids::new(ui.widget_id_generator());

/*     let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap(); */

    pub struct EventLoop {
        ui_needs_update: bool,
        last_update: std::time::Instant,
    }
    
    impl EventLoop {
        pub fn new() -> Self {
            EventLoop { last_update: std::time::Instant::now(),
                        ui_needs_update: true,
                      }
        }
    
        /// Produce an iterator yielding all available events.
        pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop) ->
                    Vec<glium::glutin::Event> {
    
            // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
            // since the last yield.
            let last_update = self.last_update;
            let sixteen_ms = std::time::Duration::from_millis(16);
            let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }
    
            // Collect all pending events.
            let mut events = Vec::new();
            events_loop.poll_events(|event| events.push(event));
    
            // If there are no events and the UI does not need updating, wait
            // for the next event.
            if events.is_empty() && !self.ui_needs_update {
                events_loop.run_forever(|event| { events.push(event);
                                        glium::glutin::ControlFlow::Break });
            }
    
            self.ui_needs_update = false;
            self.last_update = std::time::Instant::now();
    
            events
        }
    
        /// Notifies the event loop that the `Ui` requires another update whether
        /// or not there are any pending events.
        ///
        /// This is primarily used on the occasion that some part of the UI is
        /// still animating and requires further updates to do so.
        pub fn needs_update(&mut self) {
            self.ui_needs_update = true;
        }
    }

    let mut event_loop = EventLoop::new();

    loop {
        {
            let ui = &mut ui.set_widgets();

            // "Hello World!" in the middle of the screen.
            widget::Text::new("Hello World!")
                .middle_of(ui.window)
                .color(conrod::color::WHITE)
                .font_size(32)
                .set(ids.text, ui);
        }

        // Then, call `draw_if_changed` using a mutable reference to `ui`.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 1.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }


        for event in event_loop.next(&mut events_loop) {
            if let Some(event) = conrod::backend::winit::convert_event(
                event.clone(),
                &display
            ) {
                ui.handle_event(event);
            }

            match event {
                glium::glutin::Event::WindowEvent { event, ..} => match event {
                    glium::glutin::WindowEvent::Closed |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => (),
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
