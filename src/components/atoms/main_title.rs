use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Color {
    Normal,
    Ok,
    Error,
}

impl Color {
    pub fn to_str(&self) -> String {
        match self {
            Color::Normal => "normal".to_owned(),
            Color::Ok => "ok".to_owned(),
            Color::Error => "error".to_owned(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub color: Color,
    pub on_load: Callback<String>,
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let stylesheet: Style = style!(
        r#"
        .normal {
            color: greenyellow;
        }
        .ok {
            color: blue;
        }
        .error {
            color: red;
        }
        "#
    )
    .unwrap();

    props.on_load.emit("Main Title loaded".to_owned());

    html! {
      <div class={stylesheet}>
        <h1 class={&props.color.to_str()}>{&props.title}</h1>
      </div>
    }
}
