extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate image;
extern crate ndarray;
//added
use ndarray::{Array, Array2};
use std::f64::consts::PI;


use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings, Filter};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, MouseCursorEvent};
use piston::window::WindowSettings;

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 500;
const PIXEL_SCALE: u32 = 1; // How big every pixel should be (1 is normal)

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    frame_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    time: f64,  // Time progressed since start
    mouse_pos: [f64; 2],
    fluid: Fluid,
}

// Fluid struct
pub struct Fluid {
    density: Array2<f64>,
    velocity_x: Array2<f64>,
    velocity_y: Array2<f64>,
    width: usize,
    height: usize,
    diffusion: f64,
    viscosity: f64,
}

// Fluid methods
impl Fluid {
    pub fn new(width: usize, height: usize, diffusion: f64, viscosity: f64) -> Self {
        Fluid {
            density: Array::zeros((width, height)),
            velocity_x: Array::zeros((width, height)),
            velocity_y: Array::zeros((width, height)),
            width,
            height,
            diffusion,
            viscosity,
        }
    }

    pub fn add_density(&mut self, x: usize, y: usize, amount: f64) {
        self.density[[x, y]] += amount;
    }

    pub fn add_velocity(&mut self, x: usize, y: usize, amount_x: f64, amount_y: f64) {
        self.velocity_x[[x, y]] += amount_x;
        self.velocity_y[[x, y]] += amount_y;
    }

    fn lin_solve(&mut self, x: usize, y: usize, a: f64, c: f64, b: &mut Array2<f64>, x_prev: &Array2<f64>) {
        let a_inv = 1.0 / (1.0 + 4.0 * a);
        let width = self.width;
        let height = self.height;
        let iterations = 20; //can be changed depending on need

        for _ in 0..iterations {
            for j in 1..height - 1 {
                for i in 1..width - 1 {
                    b[[i, j]] = (x_prev[[i, j]] + a * (b[[i - 1, j]] + b[[i + 1, j]] + b[[i, j - 1]] + b[[i, j + 1]])) * a_inv;
                }
            }
        }
    }

    fn diffuse(&mut self, b: &mut Array2<f64>, x: &mut Array2<f64>, a: f64, dt: f64) {
        let a = dt * a * (self.width - 2) as f64 * (self.height - 2) as f64;
        self.lin_solve(1, 1, a, 1.0 + 4.0 * a, b, x);
    }

    fn advect(&mut self, d: &mut Array2<f64>, d0: &Array2<f64>, veloc_x: &Array2<f64>, veloc_y: &Array2<f64>, dt: f64) {
        let width = self.width;
        let height = self.height;

        let dt0_x = dt * (width - 2) as f64;
        let dt0_y = dt * (height - 2) as f64;
        for j in 1..height - 1 {
            for i in 1..width - 1 {
                let x = (i as f64 - dt0_x * veloc_x[[i, j]]) as usize;
                let y = (j as f64 - dt0_y * veloc_y[[i, j]]) as usize;

                let x = x.max(1).min(width - 2);
                let y = y.max(1).min(height - 2);

                d[[i, j]] = d0[[x, y]];
            }
        }
    }

    fn step(&mut self, dt: f64) {
        let mut velocity_x0 = self.velocity_x.clone();
        let mut velocity_y0 = self.velocity_y.clone();
        let mut density0 = self.density.clone();
    
        self.diffuse(&mut self.velocity_x, &mut velocity_x0, self.viscosity, dt);
        self.diffuse(&mut self.velocity_y, &mut velocity_y0, self.viscosity, dt);
    
        self.advect(&mut self.velocity_x, &velocity_x0, &velocity_x0, &velocity_y0, dt);
        self.advect(&mut self.velocity_y, &velocity_y0, &velocity_x0, &velocity_y0, dt);
    
        self.diffuse(&mut self.density, &density0, self.diffusion, dt);
        self.advect(&mut self.density, &density0, &self.velocity_x, &self.velocity_y, dt);
    }
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

        // FLUID LOGIC GOES HERE

        // DEMO: Paint every pixel from top to bottom
        // self.frame_buffer.put_pixel(
        //     (self.time / args.dt) as u32 % self.frame_buffer.width(), 
        //     (self.time / args.dt) as u32 / self.frame_buffer.width(), 
        //     image::Rgba([2 * self.time as u8, 255, 255, 255])
        // );

        // Add density and velocity (using the mouse position as an example)
        let x = (self.mouse_pos[0] / PIXEL_SCALE as f64) as usize;
        let y = (self.mouse_pos[1] / PIXEL_SCALE as f64) as usize;
        self.fluid.add_density(x, y, 100.0);
        self.fluid.add_velocity(x, y, 1.0, 1.0);

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

    // Create fluid instance
    let mut fluid = Fluid::new((WINDOW_WIDTH / PIXEL_SCALE) as usize, (WINDOW_HEIGHT / PIXEL_SCALE) as usize, 0.1, 0.001);

    // Create app object
    let mut app = App {
        gl: GlGraphics::new(opengl),
        frame_buffer,
        time: 0.0,
        mouse_pos: [0.0, 0.0],
        fluid,
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