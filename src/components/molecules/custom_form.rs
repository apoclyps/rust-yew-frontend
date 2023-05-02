use yew::prelude::*;

use crate::components::atoms::{custom_button::CustomButton, text_input::TextInput};

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    html! {
      <form>
        <TextInput name="username" />
        <CustomButton label="Submit" />
      </form>
    }
}
