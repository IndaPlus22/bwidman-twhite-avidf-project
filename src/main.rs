// extern crate image;
extern crate piston_window;
#[macro_use] extern crate gfx;
extern crate ndarray;

mod fluid_dynamics;

use fluid_dynamics::*;
use ndarray::{Array, Array2};

use piston_window::*;
use gfx::{traits::FactoryExt, Factory};

// Dimensions of simulation
const SIM_WIDTH: usize = 16;
const SIM_HEIGHT: usize = 16;

const PIXEL_SCALE: usize = 32; // How big every pixel should be (1 is normal)
const WINDOW_WIDTH: u32 = (SIM_WIDTH * PIXEL_SCALE) as u32;
const WINDOW_HEIGHT: u32 = (SIM_HEIGHT * PIXEL_SCALE) as u32;

pub struct App {
    dt: f64,
    mouse_pos: [f64; 2],
    mouse_movement: [f64; 2],
    left_mouse_state: ButtonState,
    fluid: Fluid,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        self.dt = args.ext_dt; // Store time between rendered frames (dt in UpdateArgs uses time between monitor frames wtf)
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // Add density and velocity on cursor if mouse is pressed
        if self.left_mouse_state == ButtonState::Press {
            let x = (self.mouse_pos[0] / PIXEL_SCALE as f64) as usize;
            let y = (self.mouse_pos[1] / PIXEL_SCALE as f64) as usize;

            if x < SIM_WIDTH && y < SIM_HEIGHT {
                self.fluid.add_density(x, y, 0.2);
                self.fluid.add_velocity(x, y, self.mouse_movement[0] / 10.0, self.mouse_movement[1] / 10.0);
            }
        }
    }
}

gfx_defines!(
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
    }

    pipeline pipe {
        vertex_buffer: gfx::VertexBuffer<Vertex> = (),
        resolution: gfx::Global<[u32; 2]> = "resolution",
        out_color: gfx::RenderTarget<gfx::format::Srgba8> = "o_Color",
        density: gfx::TextureSampler<[f32; 4]> = "density",
        velocity: gfx::TextureSampler<[f32; 4]> = "velocity",
        diffusion: gfx::Global<f32> = "diffusion",
        viscosity: gfx::Global<f32> = "viscosity",
    }
);

fn main() {
    let opengl = OpenGL::V3_2; // Change this to OpenGL::V2_1 if not working

    // Create a Glutin window
    let mut window: PistonWindow = WindowSettings::new("Fluid simulator", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    
    let pso = window.factory.create_pipeline_simple(
        include_bytes!("basic.vert"), include_bytes!("fluid_dynamics.frag"), pipe::new()).unwrap();

    const SCREEN_VERTICES: [Vertex; 4] = [
        Vertex { pos: [ 1.0,  1.0, 0.0, 1.0] },
        Vertex { pos: [-1.0, -1.0, 0.0, 1.0] },
        Vertex { pos: [-1.0,  1.0, 0.0, 1.0] },
        Vertex { pos: [ 1.0, -1.0, 0.0, 1.0] },
    ];
    const SCREEN_INDICES: &[u16] = &[
        0, 1, 2,
        0, 1, 3
    ];
    let (vertex_buffer, slice) = window.factory.create_vertex_buffer_with_slice(&SCREEN_VERTICES, SCREEN_INDICES);

    let kind = gfx::texture::Kind::D2(SIM_WIDTH as u16, SIM_HEIGHT as u16, gfx::texture::AaMode::Single);

    let (_, density_texture) = window.factory.create_texture_immutable::<gfx::format::Rgba32F>(kind, gfx::texture::Mipmap::Provided, 
        &[&[[0u32; 4]; SIM_WIDTH * SIM_HEIGHT]]).unwrap();
    let (_, velocity_texture) = window.factory.create_texture_immutable::<gfx::format::Rgba32F>(kind, gfx::texture::Mipmap::Provided, 
        &[&[[0u32; 4]; SIM_WIDTH * SIM_HEIGHT]]).unwrap();

    let tex_info = gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Scale, gfx::texture::WrapMode::Clamp);

    let mut data = pipe::Data {
        vertex_buffer,
        resolution: [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32],
        out_color: window.output_color.clone(),
        density: (density_texture, window.factory.create_sampler(tex_info)),
        velocity: (velocity_texture, window.factory.create_sampler(tex_info)),
        diffusion: 0.1,
        viscosity: 0.001,
    };

    // Create app object
    let mut app = App {
        dt: 0.0,
        mouse_pos: [0.0, 0.0],
        mouse_movement: [0.0, 0.0],
        left_mouse_state: ButtonState::Release,
        fluid: Fluid::new(0.1, 0.001),
    };
    
    // Event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        window.draw_3d(&e, |window| {
            window.encoder.clear(&window.output_color, [0.0, 0.0, 0.0, 1.0]);
            window.encoder.draw(&slice, &pso, &data);
        });

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
