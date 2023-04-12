use crate::*;

// Dimensions of simulation
const SIM_WIDTH: usize = (WINDOW_WIDTH / PIXEL_SCALE) as usize;
const SIM_HEIGHT: usize = (WINDOW_HEIGHT / PIXEL_SCALE) as usize;

pub struct Fluid {
    pub density: Array2<f64>,
    velocity_x: Array2<f64>,
    velocity_y: Array2<f64>,
    diffusion: f64,
    viscosity: f64,
}

impl Fluid {
    pub fn new(diffusion: f64, viscosity: f64) -> Self {
        Fluid {
            density: Array::zeros((SIM_WIDTH, SIM_HEIGHT)),
            velocity_x: Array::zeros((SIM_WIDTH, SIM_HEIGHT)),
            velocity_y: Array::zeros((SIM_WIDTH, SIM_HEIGHT)),
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

    pub fn step(&mut self, dt: f64) {
        let velocity_x0 = self.velocity_x.clone();
        let velocity_y0 = self.velocity_y.clone();
        let density0 = self.density.clone();
    
        diffuse(&mut self.velocity_x, &velocity_x0, self.viscosity, dt);
        diffuse(&mut self.velocity_y, &velocity_y0, self.viscosity, dt);
    
        advect(&mut self.velocity_x, &velocity_x0, &velocity_x0, &velocity_y0, dt);
        advect(&mut self.velocity_y, &velocity_y0, &velocity_x0, &velocity_y0, dt);
    
        diffuse(&mut self.density, &density0, self.diffusion, dt);
        advect(&mut self.density, &density0, &self.velocity_x, &self.velocity_y, dt);
    }
}

fn lin_solve(x: usize, y: usize, a: f64, c: f64, b: &mut Array2<f64>, x_prev: &Array2<f64>) {
    let a_inv = 1.0 / (1.0 + 4.0 * a);
    let iterations = 20; //can be changed depending on need

    for _ in 0..iterations {
        for j in 1..SIM_HEIGHT - 1 {
            for i in 1..SIM_WIDTH - 1 {
                b[[i, j]] = (x_prev[[i, j]] + a * (b[[i - 1, j]] + b[[i + 1, j]] + b[[i, j - 1]] + b[[i, j + 1]])) * a_inv;
            }
        }
    }
}

fn diffuse(b: &mut Array2<f64>, x: &Array2<f64>, a: f64, dt: f64) {
    let a = dt * a * (SIM_WIDTH - 2) as f64 * (SIM_HEIGHT - 2) as f64;
    lin_solve(1, 1, a, 1.0 + 4.0 * a, b, x);
}

fn advect(d: &mut Array2<f64>, d0: &Array2<f64>, veloc_x: &Array2<f64>, veloc_y: &Array2<f64>, dt: f64) {
    let dt0_x = dt * (SIM_WIDTH - 2) as f64;
    let dt0_y = dt * (SIM_HEIGHT - 2) as f64;

    for j in 1..SIM_HEIGHT - 1 {
        for i in 1..SIM_WIDTH - 1 {
            let x = (i as f64 - dt0_x * veloc_x[[i, j]]) as usize;
            let y = (j as f64 - dt0_y * veloc_y[[i, j]]) as usize;

            let x = x.max(1).min(SIM_WIDTH - 2);
            let y = y.max(1).min(SIM_HEIGHT - 2);

            d[[i, j]] = d0[[x, y]];
        }
    }
}