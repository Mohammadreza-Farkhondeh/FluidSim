use gloo_timers::callback::Interval;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use crate::fluid_simulation::FluidSimulation;

pub struct App {
    simulation: FluidSimulation,
    running: bool,
    viscosity: f32,
    diffusion: f32,
    interval: Option<Interval>,
}

pub enum Msg {
    Start,
    Pause,
    Reset,
    Step,
    SetViscosity(InputEvent),
    SetDiffusion(InputEvent),
    SetSpeed(InputEvent),
    AddDensity(usize, usize),
    Tick,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            simulation: FluidSimulation::new(100, 100),
            running: false,
            viscosity: 0.1,
            diffusion: 0.1,
            interval: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                self.running = true;
                let link = _ctx.link().clone();
                self.interval = Some(Interval::new(16, move || {
                    link.send_message(Msg::Tick);
                }));
                true
            }
            Msg::Pause => {
                self.running = false;
                self.interval = None;
                true
            }
            Msg::Reset => {
                self.simulation = FluidSimulation::new(100, 100);
                true
            }
            Msg::Step => {
                self.simulation.step(self.viscosity, self.diffusion);
                self.draw();
                true
            }
            Msg::SetViscosity(e) => {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    self.viscosity = input.value_as_number() as f32;
                }
                true
            }
            Msg::SetDiffusion(e) => {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    self.diffusion = input.value_as_number() as f32;
                }
                true
            }
            Msg::SetSpeed(e) => {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let speed = input.value_as_number() as u32;
                    if self.running {
                        let link = _ctx.link().clone();
                        self.interval = Some(Interval::new(1000 / speed, move || {
                            link.send_message(Msg::Tick);
                        }));
                    }
                }
                true
            }
            Msg::AddDensity(x, y) => {
                self.simulation.apply_forces(0.0, 0.0, 1.0, x, y);
                true
            }
            Msg::Tick => {
                self.simulation.step(self.viscosity, self.diffusion);
                self.draw();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div id="control-panel">
                    <button onclick={_ctx.link().callback(|_| Msg::Start)}>{"Start"}</button>
                    <button onclick={_ctx.link().callback(|_| Msg::Pause)}>{"Pause"}</button>
                    <button onclick={_ctx.link().callback(|_| Msg::Reset)}>{"Reset"}</button>
                    <button onclick={_ctx.link().callback(|_| Msg::Step)}>{"Step"}</button>
                    <label for="viscosity">{"Viscosity: "}</label>
                    <input type="range" id="viscosity" min="0.0" max="1.0" step="0.01" value="0.1"
                           oninput={_ctx.link().callback(|e: InputEvent| Msg::SetViscosity(e))} />
                    <label for="diffusion">{"Diffusion: "}</label>
                    <input type="range" id="diffusion" min="0.0" max="1.0" step="0.01" value="0.1"
                           oninput={_ctx.link().callback(|e: InputEvent| Msg::SetDiffusion(e))} />
                    <label for="speed">{"Speed: "}</label>
                    <input type="range" id="speed" min="1" max="100" value="50"
                           oninput={_ctx.link().callback(|e: InputEvent| Msg::SetSpeed(e))} />
                </div>
                <canvas id="fluidCanvas" width="800" height="600"
                        onmousedown={_ctx.link().callback(|e: MouseEvent| Msg::AddDensity(e.offset_x() as usize, e.offset_y() as usize))}
                        onmousemove={_ctx.link().callback(|e: MouseEvent| Msg::AddDensity(e.offset_x() as usize, e.offset_y() as usize))}></canvas>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.draw();
        }
    }
}

impl App {
    fn draw(&self) {
        let window = web_sys::window().expect("should have a Window");
        let document = window.document().expect("should have a Document");
        let canvas = document.get_element_by_id("fluidCanvas").unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();
        let context: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        // Clear the canvas
        context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        // Draw the density field
        for y in 0..self.simulation.height() {
            for x in 0..self.simulation.width() {
                let density = self.simulation.density_at(x, y);
                let color = format!("rgba(0, 0, 255, {})", density);
                context.set_fill_style(&color.into());
                context.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
}
