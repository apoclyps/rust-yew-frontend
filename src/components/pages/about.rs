use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(About)]
pub fn about() -> Html {
    let navigator: Navigator = use_navigator().unwrap();
    let onclick: Callback<MouseEvent> = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <>
        <p>{"About"}</p>
        <button onclick={onclick}>{"Go home"}</button>
        </>
    }
}
