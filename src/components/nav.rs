use yew::prelude::*;
use yew_router::{prelude::*};

use crate::route::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    let history = use_history().unwrap();

    let go_to_about_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(Route::About));
        html! {
            <button class="border" {onclick}>{"About"}</button>
        }
    };

    let go_to_contact_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(Route::Contact));
        html! {
            <button {onclick}>{"Contact"}</button>
        }
    };

    let go_to_demo_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_| history.push(Route::Demo));
        html! {
            <button {onclick}>{"Demo"}</button>
        }
    };

    html! {
        <div class="flex gap-4 flex-col sm:flex-row">
            {go_to_about_button}
            {go_to_contact_button}
            {go_to_demo_button}
        </div>
    }
}