use yew::prelude::*;

pub struct StructAbout {
    pub message: String,
}

impl Component for StructAbout {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            message: "About your structs!".to_owned(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ &self.message }</h1>
        }
    }
}
