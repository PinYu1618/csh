mod components;
mod pages;
mod route;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(route::switch)} />
        </BrowserRouter>
    }
}

pub fn start() {
    yew::start_app::<App>();
}
