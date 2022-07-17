use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TitleProps {}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let TitleProps {} = props;
    html! {
        <div class="container">
            <h1 class="text-center text-xl font-bold">{ "Beauty With Care" }</h1>
        </div>
    }
}