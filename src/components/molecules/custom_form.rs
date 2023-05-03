use std::ops::Deref;

use yew::prelude::*;

use crate::components::atoms::{custom_button::CustomButton, text_input::TextInput};

#[derive(Default, Clone)]
struct Data {
    pub username: String,
    pub count: u32,
}

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let state: UseStateHandle<Data> = use_state(Data::default);

    let cloned_state: UseStateHandle<Data> = state.clone();
    let username_changed: Callback<String> = Callback::from(move |username: String| {
        cloned_state.set(Data {
            username,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let button_clicked: Callback<_> = Callback::from(move |_| {
        let mut data: Data = cloned_state.deref().clone();
        data.count += 1_u32;

        cloned_state.set(data);
    });

    html! {
      <div>
        <TextInput name="username" handle_onchange={username_changed} />
        <CustomButton label="Submit" onclick={button_clicked} />

        if !state.username.is_empty() {
          <p>{"Username: "}{&state.username}</p>
        }

        if state.count >= 1_u32 {
          <p>{"Button clicked: "}{state.count} {" times"}</p>
        }

      </div>
    }
}
