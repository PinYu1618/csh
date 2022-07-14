use reqwasm::http::Request;
use yew::prelude::*;
use yew_router::{hooks::use_history, history::History};

use crate::components::{Video, VideoDetails, VideosList};
use crate::route::Route;

/* TODO: Remove this */
#[function_component(Demo)]
pub fn demo() -> Html {
    let history = use_history().unwrap();

    let on_back_home = Callback::once(move |_| history.push(Route::Home));

    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    videos.set(fetched_videos);
                });

                || ()
            },
            (),
        );
    }

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });
    
    html! {
        <>
            <button onclick={on_back_home}>{ "Go Home" }</button>
            <h1>{ "Demo" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}