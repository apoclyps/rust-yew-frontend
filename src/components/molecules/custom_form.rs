use yew::prelude::*;

use crate::components::atoms::{custom_button::CustomButton, text_input::TextInput};

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let username_state: UseStateHandle<String> = use_state(|| "no username set".to_owned());
    let cloned_username_state: UseStateHandle<String> = username_state.clone();

    let username_changed: Callback<String> = Callback::from(move |username: String| {
        cloned_username_state.set(username);
    });

    html! {
      <form>
        <TextInput name="username" handle_onchange={username_changed} />
        <CustomButton label="Submit" />

        if &*username_state != "no username set" {
          <p>{"Username: "}{&*username_state}</p>
        }
      </form>
    }
}
