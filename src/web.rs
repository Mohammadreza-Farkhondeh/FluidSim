use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use gloo_timers::callback::Interval;

use crate::fluid_simulation::FluidSimulation;

pub struct App {
    simulation: FluidSimulation,
    link: html::Scope<Self>,
}

pub enum Msg {
    Tick,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            simulation: FluidSimulation::new(100, 100),
            link: ctx.link().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.simulation.step();
                self.draw();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <canvas id="fluidCanvas" width="1000" height="1000"></canvas>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
            // Initialize canvas and start the animation loop
            let link = ctx.link().clone();
            Interval::new(16, move || {
                link.send_message(Msg::Tick);
            }).forget();
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

        context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

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
