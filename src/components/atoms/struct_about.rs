use stylist::{style, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: String,
}

pub struct StructAbout {
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

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            stylesheet: Self::style(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1 class={self.stylesheet.clone()}>{ &ctx.props().message }</h1>
        }
    }
}
