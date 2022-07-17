use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CardProps {}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let CardProps {} = props;
    html! {
        <div class="w-card shadow-csh-card rounded-card container pt-5">
            <div class="flex flex-col items-center">
                <img src="/images/doctor.png" alt="doctor image" class="bg-white rounded-full mx-auto mt-10 h-32 shadow-csh-button"/>
                <span class="mt-6 font-bold text-lg">{ "陳世鴻" }</span>
                <div class="container">
                    <ul class="text-xs">
                        <li>{ "台灣顏面整形外科專科醫師" }</li>
                        <li>{ "台灣微整形醫學會專科醫師" }</li>
                        <li>{ "中華民國美容醫學會醫師" }</li>
                        <li>{ "台灣耳鼻喉及頭頸外科專科醫師" }</li>
                        <li>{ "美國紐約Mount Sinai醫學中心研究醫師" }</li>
                        <li>{ "台灣形體美容整合醫學會醫師" }</li>
                        <li>{ "中華民國醫用雷射光電醫學會醫師" }</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}