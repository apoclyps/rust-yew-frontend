use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

mod components;

use components::atoms::main_title::MainTitle;

use crate::components::atoms::main_title::Color;
use crate::components::molecules::custom_form::{CustomForm, Data};

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

fn render_list(tasks: Vec<&str>) -> Vec<Html> {
    tasks
        .iter()
        .map(|task| html! { <li>{task}</li> })
        .collect::<Vec<Html>>()
}

const STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let name: &str = "Apoclyps";
    let my_object: MyObject = MyObject {
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };
    let class: &str = "title";
    let message: Option<&str> = Some("I am a message");

    log!("username is ", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let tasks: Vec<&str> = vec!["record video", "edit video", "upload video"];

    let stylesheet: Style = Style::new(STYLE_FILE).expect("An error occured with the stylesheet");

    let main_title_load = Callback::from(|message: String| log!(message));

    let custom_form_submit = Callback::from(|data: Data| {
        log!("username is", data.username);
        log!("favorite language is", data.language);
    });

    html! {
        <>
        <div class={stylesheet}>
            <MainTitle title="hello world!" color={Color::Normal} on_load={main_title_load} />

            if class == "title" {
                <p>{"Hi there!"}</p>
            } else {
                <p>{"Not a title"}</p>
            }

            if let Some(message) = message {
                <p class={css!("color: #efefef; front-size:75px;")}>{message}</p>
            } else {
                <p class={css!("color: #efefef; front-size:75px;")}>{"No message"}</p>
            }

            <ul>
                { render_list(tasks) }
            </ul>

            <CustomForm onsubmit={custom_form_submit} />

        </div>
        </>
    }
}
