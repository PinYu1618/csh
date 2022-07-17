use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Footer;
use crate::route::{self, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="bg-csh h-screen w-screen flex items-center justify-center font-serif container">
                <Switch<Route> render={Switch::render(route::switch)} />
                <Footer />
            </div>
        </BrowserRouter>
    }
}