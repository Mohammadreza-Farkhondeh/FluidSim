use yew::prelude::*;
use web_sys::window;

struct Model;

enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                window().unwrap().alert_with_message("Hello, Yew!").unwrap();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::Click)}>{ "Click me" }</button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}
