use leptos::{leptos_dom::logging::console_log, *};
use std::{string, time::Duration};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Progress/>
    }
}

#[component]
fn Progress() -> impl IntoView {
    let (progress, set_progress) = create_signal(0.0);

    spawn_local(async move {
        let progress = get_progress().await;
        console_log(&format!("progress: {}", progress));
        set_progress.set(progress);
    });

    set_interval(
        move || {
            spawn_local(async move {
                let progress = get_progress().await;
                set_progress.set(progress);
            });
        },
        Duration::from_secs(60 * 60 * 24),
    );

    {
        move || {
            view! {
                <div class="progress" style="width:100%; height:100%">
                    <div class="progress-bar" style=format!("background-color:fuchsia;width: {}%; height:100%", progress.get() * 100.0)>
                    </div>
                    <span class="progress-text">{format!("{}%", (progress.get()*100.0).round())}</span>
                </div>
            }
        }
    }
}

#[wasm_bindgen]
pub async fn get_progress() -> f64 {
    let current_semester = get_current_semester().await.as_string().unwrap();

    let parts: Vec<&str> = current_semester.split("&&").collect();
    let _name = parts[0].to_string();
    let _start = chrono::NaiveDate::parse_from_str(parts[1], "%Y-%m-%d").unwrap();
    let _end = chrono::NaiveDate::parse_from_str(parts[2], "%Y-%m-%d").unwrap();

    let now = chrono::Local::now().naive_local().date();

    let days = (_end - _start).num_days();
    let days_passed = (now - _start).num_days();

    if days > 0 {
        return days_passed as f64 / days as f64;
    }

    0.0
}

pub struct Semester {
    pub start: chrono::NaiveDate,
    pub end: chrono::NaiveDate,
    pub name: String,
}

pub async fn get_current_semester() -> JsValue {
    let semesters = get_list_of_semesters().await;

    let now = chrono::Local::now().naive_local().date();

    for i in 0..semesters.len() {
        if semesters[i].start <= now && semesters[i].end >= now {
            let string = semesters[i].name.clone()
                + "&&"
                + &semesters[i].start.to_string()
                + "&&"
                + &semesters[i].end.to_string();
            return JsValue::from_str(&string);
        }
        if semesters[i].end <= now && semesters[i + 1].start >= now {
            let string = semesters[i].name.clone()
                + "&&"
                + &semesters[i].end.to_string()
                + "&&"
                + &semesters[i + 1].start.to_string();
            return JsValue::from_str(&string);
        }
    }

    JsValue::from_str("No semester found")
}

pub async fn get_list_of_semesters() -> Vec<Semester> {
    let url = "https://www.mkw.nrw/hochschule-und-forschung/studium-und-lehre/vorlesungszeiten";
    let resp = reqwest::get(url).await.unwrap();
    let body = resp.text().await.unwrap();

    console_log(&body);

    let html = scraper::Html::parse_document(&body);

    console_log(&html.html());

    //split html into two parts at <h2>Vorlesungszeiten 2022–2030<span class="sub-title">Studiengänge an Universitäten</span></h2>
    let table = html
        .select(&scraper::Selector::parse("table").unwrap())
        .next()
        .unwrap();

    console_log(&table.html());

    // use regex to get name of semester
    let regex = regex::Regex::new(r"<p>([A-z]+)(?:&nbsp;)?<\/p>").unwrap();
    let names: Vec<String> = regex
        .captures_iter(&table.html())
        .map(|x| x[1].to_string())
        .collect();

    //use regex to get start and end date of semester
    let dates = regex::Regex::new(r"(\d{2}.\d{2}.\d{4}) – (\d{2}.\d{2}.\d{4})").unwrap();
    let dates: Vec<(String, String)> = dates
        .captures_iter(&table.html())
        .map(|x| (x[1].to_string(), x[2].to_string()))
        .collect();

    let mut semesters = Vec::new();

    for i in 0..names.len() {
        console_log(&names[i]);
        console_log(&dates[i].0);
        console_log(&dates[i].1);

        let _name = names[i].clone();
        let _start = chrono::NaiveDate::parse_from_str(&dates[i].0, "%d.%m.%Y").unwrap();
        let _end = chrono::NaiveDate::parse_from_str(&dates[i].1, "%d.%m.%Y").unwrap();

        console_log("meme");

        let semester = Semester {
            start: _start,
            end: _end,
            name: _name,
        };

        semesters.push(semester);
    }

    let mut string = "".to_string();

    for i in 0..semesters.len() {
        string = string
            + &semesters[i].name.to_string()
            + "&&"
            + &semesters[i].start.to_string()
            + "&&"
            + &semesters[i].end.to_string()
            + "\n";
    }

    semesters
}
