//! TODO: Modify it to real content

use yew::prelude::*;

use crate::components::Nav;


#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <h1 class="text-center sm:text-left">{ "Shih Hong Chen" }</h1>
            <h2>{ "A human." }</h2>
            <Nav />
        </>
    }
}