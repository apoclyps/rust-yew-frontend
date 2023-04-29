use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name: &str = "Apoclyps";
    let my_object: MyObject = MyObject {
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };
    let class: &str = "title";

    log!("username is ", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    html! {
        <>
            <h1 class={class}>{ "Hello World!" }</h1>
            <p>{"Hi there!"}</p>
        </>
    }
}
