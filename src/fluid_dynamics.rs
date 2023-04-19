use crate::*;

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
        const PROJECTION_ITERATIONS: usize = 20;

        diffuse(&mut self.velocity_x, &velocity_x0, self.viscosity, dt);
        diffuse(&mut self.velocity_y, &velocity_y0, self.viscosity, dt);

        advect(&mut self.velocity_x, &velocity_x0, &velocity_x0, &velocity_y0, dt);
        advect(&mut self.velocity_y, &velocity_y0, &velocity_x0, &velocity_y0, dt);

        for _ in 0..PROJECTION_ITERATIONS {
            project(&mut self.velocity_x, &mut self.velocity_y);
        }

        diffuse(&mut self.density, &density0, self.diffusion, dt);
        advect(&mut self.density, &density0, &self.velocity_x, &self.velocity_y, dt);
    }
}

fn project(u: &mut Array2<f64>, v: &mut Array2<f64>) {
    let mut div = Array::zeros((SIM_WIDTH, SIM_HEIGHT));
    let mut p = Array::zeros((SIM_WIDTH, SIM_HEIGHT));

    // Compute the divergence of the velocity field
    for j in 1..SIM_HEIGHT - 1 {
        for i in 1..SIM_WIDTH - 1 {
            div[[i, j]] = -0.5
                * ((u[[i + 1, j]] - u[[i - 1, j]]) / SIM_WIDTH as f64
                    + (v[[i, j + 1]] - v[[i, j - 1]]) / SIM_HEIGHT as f64);
            p[[i, j]] = 0.0;
        }
    }

    // Set the velocity to zero at the boundaries
    for j in 0..SIM_HEIGHT {
        u[[0, j]] = 0.0;
        u[[SIM_WIDTH - 1, j]] = 0.0;
    }

    for i in 0..SIM_WIDTH {
        v[[i, 0]] = 0.0;
        v[[i, SIM_HEIGHT - 1]] = 0.0;
    }

    // Solve the Poisson equation for pressure
    lin_solve(1, 1, 1.0, 4.0, &mut p, &div);

    // Update the velocity field to make it divergence-free
    for j in 1..SIM_HEIGHT - 1 {
        for i in 1..SIM_WIDTH - 1 {
            u[[i, j]] -= 0.5 * SIM_WIDTH as f64 * (p[[i + 1, j]] - p[[i - 1, j]]);
            v[[i, j]] -= 0.5 * SIM_HEIGHT as f64 * (p[[i, j + 1]] - p[[i, j - 1]]);
        }
    }
}


fn lin_solve(x: usize, y: usize, a: f64, c: f64, b: &mut Array2<f64>, x_prev: &Array2<f64>) {
    let a_inv = 1.0 / (1.0 + 4.0 * a);
    const ITERATIONS: usize = 20; // Can be changed depending on need

    for _ in 0..ITERATIONS {
        for j in 1..SIM_HEIGHT - 1 {
            for i in 1..SIM_WIDTH - 1 {
                b[[i, j]] = (x_prev[[i, j]]
                    + a * (b[[i - 1, j]] + b[[i + 1, j]] + b[[i, j - 1]] + b[[i, j + 1]]))
                    * a_inv;
            }
        }
    }
}

fn diffuse(b: &mut Array2<f64>, x: &Array2<f64>, a: f64, dt: f64) {
    let a = dt * a * (SIM_WIDTH as f64 - 2.0) * (SIM_HEIGHT  as f64 - 2.0);
    lin_solve(1, 1, a, 1.0 + 4.0 * a, b, x);
}

fn advect(d: &mut Array2<f64>, d0: &Array2<f64>, veloc_x: &Array2<f64>, veloc_y: &Array2<f64>, dt: f64) {
    let dt0_x = dt * (SIM_WIDTH as f64 - 2.0);
    let dt0_y = dt * (SIM_HEIGHT as f64 - 2.0);

    for j in 1..SIM_HEIGHT - 1 {
        for i in 1..SIM_WIDTH - 1 {
            let mut x = (i as f64 - dt0_x * veloc_x[[i, j]]) as usize;
            let mut y = (j as f64 - dt0_y * veloc_y[[i, j]]) as usize;

            // Clamp coordinates within bounds
            x = x.clamp(1, SIM_WIDTH - 2);
            y = y.clamp(1, SIM_HEIGHT - 2);

            d[[i, j]] = d0[[x, y]];
        }
    }
}
