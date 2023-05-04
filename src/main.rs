// extern crate image;
extern crate piston_window;
#[macro_use] extern crate gfx;
extern crate gfx_device_gl;

use piston_window::*;
use gfx::{traits::FactoryExt, Factory, memory::{Bind, Usage}, handle::{ShaderResourceView, Sampler}};

// Dimensions of simulation
const SIM_WIDTH: usize = 16;
const SIM_HEIGHT: usize = 16;

const PIXEL_SCALE: usize = 32; // How big every pixel should be (1 is normal)
const WINDOW_WIDTH: u32 = (SIM_WIDTH * PIXEL_SCALE) as u32;
const WINDOW_HEIGHT: u32 = (SIM_HEIGHT * PIXEL_SCALE) as u32;

pub struct App {
    mouse_pos: [f64; 2],
    mouse_movement: [f64; 2],
    left_mouse_state: ButtonState,
    data: pipe::Data<gfx_device_gl::Resources>, // Data shared with shaders
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        // Store time between rendered frames (dt in UpdateArgs uses time between monitor frames wtf)
        self.data.dt = args.ext_dt as f32;

        // TODO: GUI rendering
    }

    fn update(&mut self, _args: &UpdateArgs, window: &mut PistonWindow) {
        // Add density and velocity on cursor if mouse is pressed
        if self.left_mouse_state == ButtonState::Press {
            let x = (self.mouse_pos[0] / PIXEL_SCALE as f64) as usize;
            let y = (self.mouse_pos[1] / PIXEL_SCALE as f64) as usize;

            if x < SIM_WIDTH && y < SIM_HEIGHT {
                // self.fluid.add_density(x, y, 0.2);
                // self.fluid.add_velocity(x, y, self.mouse_movement[0] / 10.0, self.mouse_movement[1] / 10.0);
                window.encoder.update_texture(, cube_face, img, data);
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
        out_color: gfx::RenderTarget<gfx::format::Srgba8> = "o_Color",
        resolution: gfx::Global<[u32; 2]> = "resolution",
        dt: gfx::Global<f32> = "dt",
        density: gfx::TextureSampler<[f32; 4]> = "density",
        velocity: gfx::TextureSampler<[f32; 4]> = "velocity",
        diffusion: gfx::Global<f32> = "diffusion",
        viscosity: gfx::Global<f32> = "viscosity",
    }
);

// For storing data unique to every pixel
fn create_texture(window: &mut PistonWindow)
    -> (ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>, Sampler<gfx_device_gl::Resources>)
{
    let kind = gfx::texture::Kind::D2(SIM_WIDTH as u16, SIM_HEIGHT as u16, gfx::texture::AaMode::Single);

    let texture = window.factory.create_texture::<gfx::format::R32_G32_B32_A32>(kind, 1, 
        Bind::SHADER_RESOURCE.union(Bind::UNORDERED_ACCESS), Usage::Dynamic, Some(gfx::format::ChannelType::Float)).unwrap();

    let resource_view = window.factory.view_texture_as_shader_resource::<(gfx::format::R32_G32_B32_A32, gfx::format::Float)>(
        &texture, (0, 0), gfx::format::Swizzle::new()).unwrap();
    
    let texture_info = gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Scale, gfx::texture::WrapMode::Clamp);

    return (resource_view, window.factory.create_sampler(texture_info))
}

fn main() {
    let opengl = OpenGL::V3_2; // Change this to OpenGL::V2_1 if not working

    // Create a Glutin window
    let mut window: PistonWindow = WindowSettings::new("Fluid simulator", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    
    // Import shader files
    let pso = window.factory.create_pipeline_simple(
        include_bytes!("basic.vert"), include_bytes!("fluid_dynamics.frag"), pipe::new()).unwrap();
    
    // Create vertex buffer with two triangles covering the screen
    // i.e. every pixel gets rendered with the fragment shader
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
    
    // let (_, velocity_texture) = window.factory.create_texture_immutable::<gfx::format::Rgba32F>(kind, 
    //     gfx::texture::Mipmap::Provided, &[&[[0u32; 4]; SIM_WIDTH * SIM_HEIGHT]]).unwrap();
    
    // Crete pipeline data object
    let data = pipe::Data {
        vertex_buffer,
        out_color: window.output_color.clone(),
        resolution: [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32],
        dt: 0.0,
        density: create_texture(&mut window),
        velocity: create_texture(&mut window),
        diffusion: 0.1,
        viscosity: 0.001,
    };

    // Create app object
    let mut app = App {
        mouse_pos: [0.0, 0.0],
        mouse_movement: [0.0, 0.0],
        left_mouse_state: ButtonState::Release,
        data,
    };
    
    // Event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        window.draw_3d(&e, |window| {
            window.encoder.clear(&window.output_color, [0.0, 0.0, 0.0, 1.0]);
            window.encoder.draw(&slice, &pso, &app.data);
        });

        if let Some(args) = e.update_args() {
            app.update(&args, &window);
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
