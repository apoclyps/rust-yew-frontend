mod components;

use std::ops::Deref;

use crate::components::atoms::main_title::Color;
use crate::components::molecules::custom_form::{CustomForm, Data};
use components::atoms::main_title::MainTitle;
use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;
use yew::ContextProvider;

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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub language: String,
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
    let user_state = use_state(User::default);

    log!("username is ", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let tasks: Vec<&str> = vec!["record video", "edit video", "upload video"];

    let stylesheet: Style = Style::new(STYLE_FILE).expect("An error occured with the stylesheet");

    let main_title_load: Callback<String> = Callback::from(|message: String| log!(message));

    let custom_form_submit: Callback<Data> = {
        let user_state: UseStateHandle<User> = user_state.clone();

        Callback::from(move |data: Data| {
            let mut user: User = user_state.deref().clone();

            user.username = data.username;
            user.language = data.language;

            user_state.set(user);
        })
    };

    let first_load = use_state(|| true);

    use_effect(move || {
        // this code will run on first render and all subsequent renders

        // if auth token is exists and it's our first render then:
        // get all users todo tasks

        if *first_load {
            // this is only run when the component loads for the first time
            first_load.set(false);
            log!("first load")
        }

        || {}
    });

    html! {
        <>
        <div class={stylesheet}>

        <ContextProvider<User> context={user_state.deref().clone()}>
            <MainTitle title="hello world!" color={Color::Normal} on_load={main_title_load} />
            <CustomForm onsubmit={custom_form_submit} />
        </ContextProvider<User>>

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

        </div>
        </>
    }
}
