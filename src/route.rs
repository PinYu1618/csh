// TODO: Remove this line
#![allow(unused)]

use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,

    #[at("/demo")]
    Demo,
}

pub fn switch(route: &Route) -> Html {
    match route {
        // TODO: Remove this
        Route::Demo => html! {
            <Demo />
        },

        Route::Home => html! {
            <Home />
        },
        Route::NotFound => html! {
            <NotFound />
        },
        _ => html! {}
    }
}