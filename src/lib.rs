mod fluid_simulation;
mod web;

use console_error_panic_hook::hook;
use wasm_bindgen::prelude::*;
use web::App;
use yew::Renderer;

#[wasm_bindgen(start)]
pub fn run() {
    std::panic::set_hook(Box::new(hook));

    Renderer::<App>::new().render();
}
