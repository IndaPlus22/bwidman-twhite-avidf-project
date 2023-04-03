use crate::*;
use graphics::types::*;

const SIM_WIDTH: usize = (WINDOW_WIDTH / PIXEL_SCALE) as usize;
const SIM_HEIGHT: usize = (WINDOW_HEIGHT / PIXEL_SCALE) as usize;

pub struct FluidProperties {
    velocity: [[Vec2d; SIM_WIDTH]; SIM_HEIGHT],
    force: [[Vec2d; SIM_WIDTH]; SIM_HEIGHT],
    pressure: f64,
    density: f64,
    viscocity: f64,
}

impl FluidProperties {
    // Starting state of simulation
    pub fn new() -> Self {
        FluidProperties {
            velocity: [[[0.0, 0.0]; SIM_WIDTH]; SIM_HEIGHT],
            force: [[[0.0, 0.0]; SIM_WIDTH]; SIM_HEIGHT],
            pressure: 1.0,
            density: 1.0,
            viscocity: 1.0,
        }
    }
}

pub fn fluid_dynamics(x: u32, y: u32, fluid_properties: &FluidProperties) -> image::Rgba<u8> {
    let mut new_color = image::Rgba([0, 0, 0, 255]);

    

    return new_color;
}