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

    pub fn step(&mut self) {
        // TODO: Implement the simulation step here (Navier-Stokes)
    }

    pub fn add_density(&mut self, x: usize, y: usize, amount: f32) {
        let index = self.index(x, y);
        self.density[index] += amount;
    }

    pub fn add_velocity(&mut self, x: usize, y: usize, amount_x: f32, amount_y: f32) {
        let index = self.index(x, y);
        self.velocity_x[index] += amount_x;
        self.velocity_y[index] += amount_y;
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

    pub fn density_at(&self, x: usize, y: usize) -> f32 {
        let index = self.index(x, y);
        self.density[index]
    }
}
