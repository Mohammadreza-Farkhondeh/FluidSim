use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FluidSimulation {
    width: usize,
    height: usize,
    density: Vec<f32>,
    velocity_x: Vec<f32>,
    velocity_y: Vec<f32>,
}

#[wasm_bindgen]
impl FluidSimulation {
    pub fn new(width: usize, height: usize) -> FluidSimulation {
        FluidSimulation {
            width,
            height,
            density: vec![0.0; width * height],
            velocity_x: vec![0.0; width * height],
            velocity_y: vec![0.0; width * height],
        }
    }

    pub fn step(&mut self, dt: f32, diff: f32) {
        self.advect(dt);
        
        self.diffuse(diff, dt);
        
        // Apply a downward gravitational force uniformly
        self.apply_forces(0.0, 9.8, dt); 
        
        self.project();
        
    }


    pub fn add_velocity(&mut self, x: usize, y: usize, amount_x: f32, amount_y: f32) {
        let index = self.index(x, y);
        if index < self.velocity_x.len() && index < self.velocity_y.len() {
            self.velocity_x[index] += amount_x;
            self.velocity_y[index] += amount_y;
        }
    }

    pub fn apply_forces(&mut self, force_x: f32, force_y: f32, dt: f32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                self.velocity_x[idx] += force_x * dt;
                self.velocity_y[idx] += force_y * dt;
            }
        }
    }
    

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_density_data(&self) -> *const f32 {
        self.density.as_ptr()
    }

    pub fn add_density(&mut self, x: usize, y: usize, amount: f32) {
        let index = self.index(x, y);
        if index < self.density.len() {
            self.density[index] += amount;
        }
    }

    pub fn density_at(&self, x: usize, y: usize) -> f32 {
        let index = self.index(x, y);
        if index < self.density.len() {
            self.density[index]
        } else {
            0.0
        }
    }
}


impl FluidSimulation {
    fn advect(&mut self, dt: f32) {
        let width = self.width as isize;
        let height = self.height as isize;
        
        let mut new_density = vec![0.0; self.width * self.height];
        
        for y in 0..height {
            for x in 0..width {
                let idx = (y * self.width as isize + x) as usize;

                // Backtrace to find source position
                let x_src = x as f32 - self.velocity_x[idx] * dt;
                let y_src = y as f32 - self.velocity_y[idx] * dt;
                
                // Clamp source position to the grid boundaries
                let x_src_clamped = x_src.max(0.0).min(width as f32 - 1.0);
                let y_src_clamped = y_src.max(0.0).min(height as f32 - 1.0);
                
                // Bilinear interpolation
                let x0 = x_src_clamped.floor() as isize;
                let x1 = x0 + 1;
                let y0 = y_src_clamped.floor() as isize;
                let y1 = y0 + 1;
                
                let sx = x_src_clamped - x0 as f32;
                let sy = y_src_clamped - y0 as f32;
                
                let d00 = self.density[(y0 * width + x0) as usize];
                let d10 = if x1 < width { self.density[(y0 * width + x1) as usize] } else { d00 };
                let d01 = if y1 < height { self.density[(y1 * width + x0) as usize] } else { d00 };
                let d11 = if x1 < width && y1 < height { self.density[(y1 * width + x1) as usize] } else { d00 };

                let d0 = d00 + sx * (d10 - d00);
                let d1 = d01 + sx * (d11 - d01);
                new_density[idx] = d0 + sy * (d1 - d0);
            }
        }

        self.density = new_density;
    }
    

    fn diffuse(&mut self, diff: f32, dt: f32) {
        let a = dt * diff * (self.width as f32 - 2.0) * (self.height as f32 - 2.0);
        let iter = 20;

        for _ in 0..iter {
            for y in 1..self.height - 1 {
                for x in 1..self.width - 1 {
                    let idx = y * self.width + x;
                    self.density[idx] = (self.density[idx]
                        + a * (self.density[idx - 1] + self.density[idx + 1]
                                + self.density[idx - self.width] + self.density[idx + self.width]))
                        / (1.0 + 4.0 * a);
                }
            }
            self.set_bnd();
        }
    }

    fn set_bnd(&mut self) {
        // Set boundary conditions (for simplicity, assume density at boundary is zero)
        for i in 1..self.width - 1 {
            self.density[i] = 0.0;
            self.density[(self.height - 1) * self.width + i] = 0.0;
        }
        for j in 1..self.height - 1 {
            self.density[j * self.width] = 0.0;
            self.density[j * self.width + self.width - 1] = 0.0;
        }
    }


    fn project(&mut self) {
        let mut div = vec![0.0; self.width * self.height];
        let mut p = vec![0.0; self.width * self.height];
        
        // Step 1: Compute Divergence
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let idx = y * self.width + x;
                div[idx] = -0.5 * (
                    self.velocity_x[idx + 1] - self.velocity_x[idx - 1] +
                    self.velocity_y[idx + self.width] - self.velocity_y[idx - self.width]
                ) / self.width as f32;
                p[idx] = 0.0;
            }
        }

        self.set_bnd_p(&mut p);
        self.set_bnd_p(&mut div);
        
        // Step 2: Solve Poisson Equation for Pressure
        for _ in 0..20 {
            for y in 1..self.height - 1 {
                for x in 1..self.width - 1 {
                    let idx = y * self.width + x;
                    p[idx] = (div[idx] + p[idx - 1] + p[idx + 1] + p[idx - self.width] + p[idx + self.width]) / 4.0;
                }
            }
            self.set_bnd_p(&mut p);
        }

        // Step 3: Subtract Gradient of Pressure from Velocity
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let idx = y * self.width + x;
                self.velocity_x[idx] -= 0.5 * (p[idx + 1] - p[idx - 1]) * self.width as f32;
                self.velocity_y[idx] -= 0.5 * (p[idx + self.width] - p[idx - self.width]) * self.height as f32;
            }
        }

        self.set_bnd_velocity();
    }

    fn set_bnd_p(&self, array: &mut [f32]) {
        for i in 1..self.width - 1 {
            array[i] = array[self.width + i];
            array[(self.height - 1) * self.width + i] = array[(self.height - 2) * self.width + i];
        }
        for j in 1..self.height - 1 {
            array[j * self.width] = array[j * self.width + 1];
            array[j * self.width + self.width - 1] = array[j * self.width + self.width - 2];
        }
    }

    fn set_bnd_velocity(&mut self) {
        for i in 1..self.width - 1 {
            self.velocity_x[i] = self.velocity_x[self.width + i];
            self.velocity_x[(self.height - 1) * self.width + i] = self.velocity_x[(self.height - 2) * self.width + i];
            self.velocity_y[i] = self.velocity_y[self.width + i];
            self.velocity_y[(self.height - 1) * self.width + i] = self.velocity_y[(self.height - 2) * self.width + i];
        }
        for j in 1..self.height - 1 {
            self.velocity_x[j * self.width] = self.velocity_x[j * self.width + 1];
            self.velocity_x[j * self.width + self.width - 1] = self.velocity_x[j * self.width + self.width - 2];
            self.velocity_y[j * self.width] = self.velocity_y[j * self.width + 1];
            self.velocity_y[j * self.width + self.width - 1] = self.velocity_y[j * self.width + self.width - 2];
        }
    }
}
