use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub onclick: Callback<()>,
}

#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    let onclick: Callback<()> = props.onclick.clone();
    let button_onclick: Callback<MouseEvent> = Callback::from(move |_| {
        onclick.emit(());
    });

    html! {
        <button onclick={button_onclick}>{&props.label}</button>
    }
}
