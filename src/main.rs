use std::time::Duration;
use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, RequestCache, Request, Response};
mod trains;
mod cal;
mod mensa;

fn main() {
    spawn_local(async move {
        cal::memes().await;
    });
    leptos::mount_to_body(move || view! { 

        <div style="height:100vh; width:80vw">
            <div style="height:5vh; width:100%">
                <Notification_Bar/>
            </div>
            <div style="height:75vh; width:100%;">
              <trains::App/>  
            </div>
            <div style="height:20vh; width:100%;">
              <mensa::App2/>
            </div>
        </div>
        <div style="height:100vh; width:20vw">
            <cal::App/>
        </div>
    })

}

#[component]
fn Notification_Bar() -> impl IntoView {
    let (localtime, set_localtime) = create_signal(String::new());

    set_interval(
        move || {
            let time = chrono::Local::now().format("%d.%m.%Y   %H:%M").to_string();
            set_localtime.set(time);
        },
        Duration::from_secs(1),
    );

    view! {
        <div class="center" style="height:100%;">
            <h1>{ move || localtime}</h1>
        </div>
    }
}

#[wasm_bindgen]
pub async fn fetch(url:String) -> String{
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    opts.cache(RequestCache::NoCache);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    let resp: Response = resp_value.dyn_into().unwrap();
    let text = JsFuture::from(resp.text().unwrap()).await.unwrap();
    let text = text.as_string().unwrap();
    text
}
