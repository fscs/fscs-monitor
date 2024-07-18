use anyhow::Result;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_dom::logging::console_warn;
use std::time::Duration;
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
        let progress = get_progress().await.unwrap_or(0.0);
        set_progress.set(progress);
    });

    set_interval(
        move || {
            spawn_local(async move {
                let progress = get_progress().await.unwrap_or(0.0);
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

pub async fn get_progress() -> Result<f64> {
    let current_semester = get_current_semester().await?;

    let now = chrono::Local::now().naive_local().date();

    let days = (current_semester.end - current_semester.start).num_days();
    let days_passed = (now - current_semester.start).num_days();

    if days > 0 {
        return Ok(days_passed as f64 / days as f64);
    }

    Ok(0.0)
}

pub struct Semester {
    pub start: chrono::NaiveDate,
    pub end: chrono::NaiveDate,
    pub name: String,
}

pub async fn get_current_semester() -> Result<Semester> {
    console_warn("sd");
    let semesters = get_list_of_semesters().await;

    for i in 0..semesters.len() {
        console_log(&semesters[i].name);
    }

    let now = chrono::Local::now().naive_local().date();

    for i in 0..semesters.len() {
        if semesters[i].start <= now && semesters[i].end >= now {
            return Ok(Semester {
                start: semesters[i].start,
                end: semesters[i].end,
                name: semesters[i].name.clone(),
            });
        }
        if semesters[i].end <= now && semesters[i + 1].start >= now {
            return Ok(Semester {
                start: semesters[i].end,
                end: semesters[i + 1].start,
                name: semesters[i].name.clone(),
            });
        }
    }
    Err(anyhow::anyhow!("No semester found"))
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
    let regex = regex::Regex::new(r"([A-z]+)(?:&nbsp;)?<\/td>").unwrap();
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

    semesters
}
