// extern crate image;
extern crate piston_window;
#[macro_use] extern crate gfx;
extern crate gfx_device_gl;

use piston_window::{*, texture::UpdateTexture};
use gfx::{traits::FactoryExt, Factory, memory::{Bind, Usage, Typed}, handle, format, texture};
use gfx_device_gl::Resources;

// Dimensions of simulation
const SIM_WIDTH: usize = 500;
const SIM_HEIGHT: usize = 500;

const PIXEL_SCALE: usize = 1; // How big every pixel should be (1 is normal)
const WINDOW_WIDTH: u32 = (SIM_WIDTH * PIXEL_SCALE) as u32;
const WINDOW_HEIGHT: u32 = (SIM_HEIGHT * PIXEL_SCALE) as u32;

type Texture4f = handle::Texture<Resources, format::R32_G32_B32_A32>;

pub struct App {
    mouse_pos: [f64; 2],
    mouse_movement: [f64; 2],
    left_mouse_state: ButtonState,
    data: pipe::Data<Resources>, // Data shared with shaders
    density: [Texture4f; 2], // Output buffer & previous values
    velocity: [Texture4f; 2],
}

impl App {
    fn render(&mut self, args: &RenderArgs, window: &mut PistonWindow) {
        // Store time between rendered frames (dt in UpdateArgs uses time between monitor frames wtf)
        self.data.dt = args.ext_dt as f32;

        let img_info = texture::ImageInfoCommon {
            xoffset: 0, yoffset: 0, zoffset: 0,
            width: SIM_WIDTH as u16, height: SIM_HEIGHT as u16,
            depth: 0,
            format: format::Format(format::SurfaceType::R32_G32_B32_A32, format::ChannelType::Float),
            mipmap: 0,
        };
        // window.encoder.copy_texture_to_texture_raw(
        //     self.density[0].raw(), None, img_info, 
        //     self.density[1].raw(), None, img_info
        // ).unwrap();
        
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
                // update_pixel(window, &self.density[1], x, y, [1.0 as u32; 4]);
            }
        }
    }
}

fn update_pixel(window: &mut PistonWindow, texture: &Texture4f, x: usize, y: usize, data: [u32; 4]) {
    let img_info = texture::ImageInfoCommon {
        xoffset: x as u16,
        yoffset: y as u16,
        zoffset: 0,
        width: 1,
        height: 1,
        depth: 0,
        format: (),
        mipmap: 0,
    };
    window.encoder.update_texture::<_, format::Rgba32F>(&texture, None, img_info, &[data]).unwrap();
}

// For storing data unique to every pixel
fn create_texture(window: &mut PistonWindow, usage: Usage, bind: Bind)
    -> (Texture4f, handle::ShaderResourceView<Resources, [f32; 4]>, handle::Sampler<Resources>)
{
    use format::*;
    use texture::*;
    let kind = Kind::D2(window.draw_size().width as u16, window.draw_size().height as u16, AaMode::Single);

    let texture = window.factory.create_texture::<R32_G32_B32_A32>(
        kind, 1, bind, usage, Some(ChannelType::Float)).unwrap();

    let resource_view = window.factory.view_texture_as_shader_resource::<Rgba32F>(
        &texture, (0, 0), Swizzle::new()).unwrap();
    
    let texture_info = SamplerInfo::new(FilterMethod::Scale, WrapMode::Clamp);
    
    return (texture, resource_view, window.factory.create_sampler(texture_info))
}

gfx_defines!(
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
    }

    pipeline pipe {
        vertex_buffer: gfx::VertexBuffer<Vertex> = (),

        resolution: gfx::Global<[u32; 2]> = "resolution",
        dt: gfx::Global<f32> = "dt",
        density: gfx::TextureSampler<[f32; 4]> = "density",
        velocity: gfx::TextureSampler<[f32; 4]> = "velocity",
        diffusion: gfx::Global<f32> = "diffusion",
        viscosity: gfx::Global<f32> = "viscosity",

        out_color: gfx::RenderTarget<format::Srgba8> = "o_Color",
        out_density: gfx::RenderTarget<format::Rgba32F> = "o_density",
        out_velocity: gfx::RenderTarget<format::Rgba32F> = "o_velocity",
    }
);

fn main() {
    let opengl = OpenGL::V3_3; // Change this to OpenGL::V2_1 if not working

    // Create a Glutin window
    let mut window: PistonWindow = WindowSettings::new("Fluid simulator", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    
    // Import shader files
    let pso = window.factory.create_pipeline_simple(
        include_bytes!("basic.vert"), include_bytes!("fluid_dynamics.frag"), pipe::new())
        .expect("Failed to compile shaders");
    
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

    // Create textures for storing per-pixel data
    let size = window.draw_size();
    let out_density_handles = create_texture(&mut window, Usage::Data, Bind::SHADER_RESOURCE | Bind::RENDER_TARGET | Bind::TRANSFER_SRC);
    let out_velocity_handles = create_texture(&mut window, Usage::Data, Bind::SHADER_RESOURCE | Bind::RENDER_TARGET | Bind::TRANSFER_SRC);

    let density_handles = create_texture(&mut window, Usage::Dynamic, Bind::SHADER_RESOURCE | Bind::TRANSFER_DST);
    let velocity_handles = create_texture(&mut window, Usage::Dynamic, Bind::SHADER_RESOURCE | Bind::TRANSFER_DST);
    
    // Create pipeline data object
    let data = pipe::Data {
        vertex_buffer,
        resolution: [size.width as u32, size.height as u32],
        dt: 0.0,
        density: (density_handles.1, density_handles.2),
        velocity: (velocity_handles.1, velocity_handles.2),
        diffusion: 0.1,
        viscosity: 0.001,
        out_color: window.output_color.clone(),
        out_density: window.factory.view_texture_as_render_target(&out_density_handles.0, 0, None).unwrap(),
        out_velocity: window.factory.view_texture_as_render_target(&out_velocity_handles.0, 0, None).unwrap(),
    };

    // Create app object
    let mut app = App {
        mouse_pos: [0.0, 0.0],
        mouse_movement: [0.0, 0.0],
        left_mouse_state: ButtonState::Release,
        data,
        density: [out_density_handles.0, density_handles.0],
        velocity: [out_velocity_handles.0, velocity_handles.0],
    };
    // unsafe { window.device.with_gl(|gl|{
    //     gl.GetAttribLocation(program, name);
    // }) };
    // Event loop
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        window.draw_3d(&e, |window| {
            window.encoder.clear(&window.output_color, [0.0, 0.0, 0.0, 1.0]);
            window.encoder.draw(&slice, &pso, &app.data);
        });
        if let Some(args) = e.render_args() {
            app.render(&args, &mut window);
        }

        if let Some(args) = e.update_args() {
            app.update(&args, &mut window);
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
