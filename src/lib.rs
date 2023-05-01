use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{style, yew::styled_component};
use yew::prelude::*;

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

    let stylesheet = style!(
        r#"
            color: white;
        "#
    )
    .expect("Failed to mount style");

    html! {
        <>
        <div class={stylesheet}>
            <h1>{ "Hello World!" }</h1>

            if class == "title" {
                <p>{"Hi there!"}</p>
            } else {
                <p>{"Not a title"}</p>
            }

            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"No message"}</p>
            }

            <ul>
                { render_list(tasks) }
            </ul>
        </div>
        </>
    }
}
