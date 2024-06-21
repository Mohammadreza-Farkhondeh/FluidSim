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
        println!("Starting step function with dt = {} and diff = {}", dt, diff);

        self.advect(dt);
        self.diffuse(diff, dt);
        self.apply_forces(0.0, -9.8, 0.1, 50, 50);
        self.project();
    }

    pub fn add_density(&mut self, x: usize, y: usize, amount: f32) {
        if let Some(index) = self.index(x, y) {
            self.density[index] += amount;
        }
    }

    pub fn add_velocity(&mut self, x: usize, y: usize, amount_x: f32, amount_y: f32) {
        if let Some(index) = self.index(x, y) {
            self.velocity_x[index] += amount_x;
            self.velocity_y[index] += amount_y;
        }
    }

    pub fn apply_forces(
        &mut self,
        force_x: f32,
        force_y: f32,
        source_density: f32,
        x: usize,
        y: usize,
    ) {
        if let Some(idx) = self.index(x, y) {
            self.velocity_x[idx] += force_x;
            self.velocity_y[idx] += force_y;
            self.density[idx] += source_density;
        }
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        assert!(x < self.width && y < self.height, "Index out of bounds: ({}, {})", x, y);

        if x < self.width && y < self.height {
            Some(x + y * self.width)
        } else {
            None
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn density_at(&self, x: usize, y: usize) -> f32 {
        self.index(x, y).map_or(0.0, |index| self.density[index])
    }
}

impl FluidSimulation {
    fn advect(&mut self, dt: f32) {
        let w = self.width as isize;
        let h = self.height as isize;
        let mut new_density = vec![0.0; self.density.len()];

        for y in 0..h {
            for x in 0..w {
                if let Some(idx) = self.index(x as usize, y as usize) {
                    let mut x0 = x as f32 - dt * self.velocity_x[idx];
                    let mut y0 = y as f32 - dt * self.velocity_y[idx];

                    if x0 < 0.0 {
                        x0 = 0.0;
                    }
                    if x0 >= w as f32 {
                        x0 = w as f32 - 1.0;
                    }
                    if y0 < 0.0 {
                        y0 = 0.0;
                    }
                    if y0 >= h as f32 {
                        y0 = h as f32 - 1.0;
                    }

                    let x1 = x0.floor() as usize;
                    let y1 = y0.floor() as usize;
                    let x2 = x1 + 1;
                    let y2 = y1 + 1;

                    let s1 = x0 - x1 as f32;
                    let s2 = 1.0 - s1;
                    let t1 = y0 - y1 as f32;
                    let t2 = 1.0 - t1;

                    if let (Some(idx1), Some(idx2), Some(idx3), Some(idx4)) = (
                        self.index(x1, y1),
                        self.index(x1, y2),
                        self.index(x2, y1),
                        self.index(x2, y2),
                    ) {
                        new_density[idx] = s2 * (t2 * self.density[idx1] + t1 * self.density[idx2])
                            + s1 * (t2 * self.density[idx3] + t1 * self.density[idx4]);
                    }
                }
            }
        }
        self.density = new_density;
    }

    fn diffuse(&mut self, diff: f32, dt: f32) {
        let a = dt * diff * (self.width * self.height) as f32;
        for _ in 0..20 {
            for y in 1..(self.height - 1) {
                for x in 1..(self.width - 1) {
                    if let (Some(idx), Some(idx1), Some(idx2), Some(idx3), Some(idx4)) = (
                        self.index(x, y),
                        self.index(x + 1, y),
                        self.index(x - 1, y),
                        self.index(x, y + 1),
                        self.index(x, y - 1),
                    ) {
                        self.density[idx] = (self.density[idx]
                            + a * (self.density[idx1]
                                + self.density[idx2]
                                + self.density[idx3]
                                + self.density[idx4]))
                            / (1.0 + 4.0 * a);
                    }
                }
            }
        }
    }

    fn project(&mut self) {
        let h = 1.0 / self.width as f32;
        let mut div = vec![0.0; self.density.len()];
        let mut p = vec![0.0; self.density.len()];

        for y in 1..(self.height - 1) {
            for x in 1..(self.width - 1) {
                if let (Some(idx), Some(idx1), Some(idx2), Some(idx3), Some(idx4)) = (
                    self.index(x, y),
                    self.index(x + 1, y),
                    self.index(x - 1, y),
                    self.index(x, y + 1),
                    self.index(x, y - 1),
                ) {
                    div[idx] = -0.5
                        * h
                        * (self.velocity_x[idx1] - self.velocity_x[idx2] + self.velocity_y[idx3]
                            - self.velocity_y[idx4]);
                    p[idx] = 0.0;
                }
            }
        }

        for _ in 0..20 {
            for y in 1..(self.height - 1) {
                for x in 1..(self.width - 1) {
                    if let (Some(idx), Some(idx1), Some(idx2), Some(idx3), Some(idx4)) = (
                        self.index(x, y),
                        self.index(x + 1, y),
                        self.index(x - 1, y),
                        self.index(x, y + 1),
                        self.index(x, y - 1),
                    ) {
                        p[idx] = (div[idx] + p[idx1] + p[idx2] + p[idx3] + p[idx4]) / 4.0;
                    }
                }
            }
        }

        for y in 1..(self.height - 1) {
            for x in 1..(self.width - 1) {
                if let (Some(idx), Some(idx1), Some(idx2), Some(idx3), Some(idx4)) = (
                    self.index(x, y),
                    self.index(x + 1, y),
                    self.index(x - 1, y),
                    self.index(x, y + 1),
                    self.index(x, y - 1),
                ) {
                    self.velocity_x[idx] -= 0.5 * (p[idx1] - p[idx2]) / h;
                    self.velocity_y[idx] -= 0.5 * (p[idx3] - p[idx4]) / h;
                }
            }
        }
    }
}
