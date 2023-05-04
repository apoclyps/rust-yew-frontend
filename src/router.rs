use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pages::{about::About, home::Home};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home />},
        Route::About => html! { <About /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
