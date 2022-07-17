use yew::prelude::*;
use crate::components::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <div class="container pt-48 pb-48">
                <Title />
                <Subtitle />
            </div>
            <Nav />
        </div>
    }
}