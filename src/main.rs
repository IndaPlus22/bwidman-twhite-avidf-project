extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate image;

mod fluid_dynamics;

use fluid_dynamics::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings, Filter};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, MouseCursorEvent};
use piston::window::WindowSettings;

type ImgBuffer = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 500;
const PIXEL_SCALE: u32 = 1; // How big every pixel should be (1 is normal)

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    frame_buffer: ImgBuffer,
    time: f64,  // Time progressed since start
    mouse_pos: [f64; 2],
    fluid_properties: FluidProperties,
}

impl App {
    fn render(&mut self, args: &RenderArgs, screen_texture: &mut Texture) {
        use graphics::*;

        // const WHITE: [f32; 4] = [1.0;4];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        
        // Update screen texture with frame buffer pixel data
        screen_texture.update(&self.frame_buffer);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Draw image buffer
            image(screen_texture, c.transform.scale(PIXEL_SCALE as f64, PIXEL_SCALE as f64), gl);

            // DEMO: Visualize mouse position
            // ellipse([1.0, 0.0, 0.0, 1.0], // currently red
            //     [self.mouse_pos[0]-10.0, self.mouse_pos[1]-10.0, 20.0, 20.0], 
            //     c.transform,
            //     gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt; // Update time

        // Decide color for every pixel
        for (x, y, color) in self.frame_buffer.enumerate_pixels_mut() {
            *color = fluid_dynamics(x, y, &self.fluid_properties);
        }
        
        // DEMO: Paint every pixel from top to bottom
        // self.frame_buffer.put_pixel(
        //     (self.time / args.dt) as u32 % self.frame_buffer.width(), 
        //     (self.time / args.dt) as u32 / self.frame_buffer.width(), 
        //     image::Rgba([2 * self.time as u8, 255, 255, 255])
        // );
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Fluid simulator", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    
    // Create frame buffer that holds the pixel data before being rendered
    let frame_buffer = image::ImageBuffer::from_pixel(WINDOW_WIDTH / PIXEL_SCALE, WINDOW_HEIGHT / PIXEL_SCALE, image::Rgba([0, 0, 0, 255]));
    // Create screen texture that is rendered
    let mut screen_texture = Texture::from_image(&frame_buffer, &TextureSettings::new().mag(Filter::Nearest));

    // Create app object
    let mut app = App {
        gl: GlGraphics::new(opengl),
        frame_buffer,
        time: 0.0,
        mouse_pos: [0.0, 0.0],
        fluid_properties: FluidProperties::new(),
    };

    // Event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &mut screen_texture);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            app.mouse_pos = pos;
        }
    }
}