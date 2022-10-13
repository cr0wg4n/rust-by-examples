use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;


#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>
}


#[derive(Clone, Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src={format!("https://avatars.dicebear.com/api/bottts/{}.svg",&video.title.clone())} alt="video thumbnail" width="10%" />
        </div>
    }
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        };
        html! {
            <>
                // <p>{format!("{}: {}", video.speaker, video.title)}</p>
                <li>
                    <p onclick={on_video_select}>{format!("{}: {} (click here!)", video.speaker, video.title)}</p>
                </li>
            </>
        }
    }).collect()
}


#[function_component(App)]
fn app() -> Html {
    
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(move |_| {
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
        }, ());
    }

    let selected_video = use_state(|| None);
    
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };

    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>
                    { "Videos to watch" }
                </h3>
                <ul>
                    <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
                </ul>
            </div>
            <div>
                { for details }
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}