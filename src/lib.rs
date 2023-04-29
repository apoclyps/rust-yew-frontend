use gloo::console::log;
use serde::{Serialize, Deserialize};
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

    log!("username is ", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    html! {
        <h1>{ "Hello World!" }</h1>
    }
}