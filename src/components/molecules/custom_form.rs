use std::ops::Deref;

use yew::prelude::*;

use crate::{
    components::atoms::{custom_button::CustomButton, text_input::TextInput},
    User,
};

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub language: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state: UseStateHandle<Data> = use_state(Data::default);
    let user_context: Option<User> = use_context::<User>();

    let cloned_state: UseStateHandle<Data> = state.clone();
    let username_changed: Callback<String> = Callback::from(move |username: String| {
        cloned_state.set(Data {
            username,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let language_changed: Callback<String> = Callback::from(move |language: String| {
        cloned_state.set(Data {
            language,
            ..cloned_state.deref().clone()
        });
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state;
    let onsubmit: Callback<SubmitEvent> = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });

    html! {
      <>
      <form onsubmit={onsubmit}>
        <TextInput name="username" handle_onchange={username_changed} />
        <TextInput name="favorite_language" handle_onchange={language_changed} />
        <CustomButton label="Submit" />
      </form>

      if !user_context.clone().unwrap_or_default().username.is_empty() {
        <p>{"Username: "}{user_context.clone().unwrap_or_default().username}</p>
      }

      if !user_context.clone().unwrap_or_default().language.is_empty() {
        <p>{"Favorite Language: "}{user_context.clone().unwrap_or_default().language}</p>
      }

      </>
    }
}
