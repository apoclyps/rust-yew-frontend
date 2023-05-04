use stylist::{style, Style};
use yew::prelude::*;

pub struct StructAbout {
    pub message: String,
    pub stylesheet: Style,
}

impl StructAbout {
    fn style() -> Style {
        style!(
            r#"
            font-size: 3rem;
        "#
        )
        .unwrap()
    }
}

impl Component for StructAbout {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            message: "About your structs!".to_owned(),
            stylesheet: Self::style(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1 class={self.stylesheet.clone()}>{ &self.message }</h1>
        }
    }
}
