use yew::prelude::*;

use crate::components::Card;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="container">
            <Card />
        </div>
    }
}