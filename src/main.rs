extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate image;
extern crate ndarray;

mod fluid_dynamics;

use fluid_dynamics::*;
use ndarray::{Array, Array2};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings, Filter};
use piston::{Button, MouseButton, ButtonState};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, MouseCursorEvent, ButtonEvent, MouseRelativeEvent};
use piston::window::WindowSettings;

type ImgBuffer = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 500;
const PIXEL_SCALE: u32 = 1; // How big every pixel should be (1 is normal)

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    frame_buffer: ImgBuffer,
    mouse_pos: [f64; 2],
    mouse_movement: [f64; 2],
    left_mouse_state: ButtonState,
    fluid: Fluid,
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
        // DEMO: Paint every pixel from top to bottom
        // self.frame_buffer.put_pixel(
        //     (self.time / args.dt) as u32 % self.frame_buffer.width(), 
        //     (self.time / args.dt) as u32 / self.frame_buffer.width(), 
        //     image::Rgba([2 * self.time as u8, 255, 255, 255])
        // );

        // Add density and velocity on cursor if mouse is pressed
        if self.left_mouse_state == ButtonState::Press {
            let x = (self.mouse_pos[0] / PIXEL_SCALE as f64) as usize;
            let y = (self.mouse_pos[1] / PIXEL_SCALE as f64) as usize;
            self.fluid.add_density(x, y, 100.0);
            self.fluid.add_velocity(x, y, self.mouse_movement[0], self.mouse_movement[1]);
        }

        // Perform fluid simulation step
        self.fluid.step(args.dt);

        // Update the frame buffer with fluid density data
        for (x, y, pixel) in self.frame_buffer.enumerate_pixels_mut() {
            let value = (self.fluid.density[[x as usize, y as usize]] * 255.0) as u8;
            *pixel = image::Rgba([value, value, value, 255]);
        }
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
        mouse_pos: [0.0, 0.0],
        mouse_movement: [0.0, 0.0],
        left_mouse_state: ButtonState::Release,
        fluid: Fluid::new(0.1, 0.001),
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

        if let Some(movement) = e.mouse_relative_args() {
            app.mouse_movement = movement;
        }

        if let Some(button) = e.button_args() {
            if button.button == Button::Mouse(MouseButton::Left) {
                app.left_mouse_state = button.state;
            }
        }
    }
}