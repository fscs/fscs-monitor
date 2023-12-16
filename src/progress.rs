use leptos::{*, leptos_dom::logging::console_log};
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use std::time::Duration;
use wasm_bindgen::prelude::*;
#[component]
pub fn App() -> impl IntoView {
    return view! {
        <Progress/>
    }
}

#[component]
fn Progress() -> impl IntoView {
    let (progerss, set_progress) = create_signal(0.0);

    spawn_local(async move {
        let progress = get_progress().await;
        set_progress.set(progress);
    });

    set_interval(
        move || {
            spawn_local(async move {
                let progress = get_progress().await;
                set_progress.set(progress);
            });
        },
        Duration::from_secs(60 * 30),
    );


    
    return view! {
        {move || {
            let prog = format!("background-color:orange; 
                       margin: 1px 1px 1px 1px;
                       width: {}%;", 
                       progerss.get() * 100.0);
            console_log(&prog);
            return view! {
                <div style=prog>
                </div>
            }
        }}
    }

}

#[wasm_bindgen]
pub async fn get_progress() -> f64 {
    let current_semester = get_current_semester().await;
    let current_time_and_date = chrono::Local::now();

    let start = current_semester.start;
    let end = current_semester.end;

    let progress = (current_time_and_date - start).num_days() as f64 / (end - start).num_days() as f64;

    return progress;
}

struct Semester {
    start: chrono::DateTime<chrono::Local>,
    end: chrono::DateTime<chrono::Local>,
    name: String,
}

pub async fn get_current_semester() -> Semester {
    let json = reqwest::get("http//localhost:8080/zeiten.json").await.unwrap().text().await.unwrap();

    let current_time_and_date = chrono::Local::now();

    let json: Value = serde_json::from_str(&json).unwrap();

    let json = json["zeiten"].as_array().unwrap();

    let mut current_semester = json[0].clone();

    for semester in json {
        let start = semester["start"].as_str().unwrap().parse::<chrono::DateTime<chrono::Local>>().unwrap();
        let end = semester["end"].as_str().unwrap().to_string().parse::<chrono::DateTime<chrono::Local>>().unwrap();

        if current_time_and_date > start && current_time_and_date < end {
            current_semester = semester.clone();
        }
    }

    let res = Semester {
        start: current_semester["start"].as_str().unwrap().parse::<chrono::DateTime<chrono::Local>>().unwrap(),
        end: current_semester["end"].as_str().unwrap().to_string().parse::<chrono::DateTime<chrono::Local>>().unwrap(),
        name: current_semester["name"].as_str().unwrap().to_string(),
    };

    return res;
}
