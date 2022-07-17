use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="container absolute place-self-end">
            <footer class="text-xs font-mono">
                <span>
                    { "© 2022. An project from " }
                    <strong>{ "PinYu1618" }</strong>
                    { ". Code licensed under MIT." }
                </span>
            </footer>
        </div>
    }
}