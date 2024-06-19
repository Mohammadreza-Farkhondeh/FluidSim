mod fluid_simulation;
mod web;

use wasm_bindgen::prelude::*;
use yew::Renderer;
use web::App;

#[wasm_bindgen(start)]
pub fn run() {
    Renderer::<App>::new().render();
}
