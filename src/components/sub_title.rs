use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SubtitleProps {}

#[function_component(Subtitle)]
pub fn subtitle(props: &SubtitleProps) -> Html {
    let SubtitleProps {} = props;
    html! {
        <div class="container">
            <p class="text-center">{ "專業臉部醫美" }</p>
        </div>
    }
}