use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <p>{"Home"}</p>
        <Link<Route> to={Route::About}>{"About"}</Link<Route>>
        </>
    }
}
