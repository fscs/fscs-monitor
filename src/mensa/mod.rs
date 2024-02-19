use anyhow::{anyhow, Result};
use chrono::prelude::*;
use chrono::Days;
use leptos::{leptos_dom::logging::console_log, *};
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Duration;

mod stw_d_parser;

#[derive(Clone, Debug)]
enum ViewState {
    Error,
    Closed,
    Open(Vec<Food>),
}

#[derive(Clone, Debug)]
pub struct Food {
    name: String,
    image_url: String,
    vegan: bool,
}

#[derive(Clone, Debug)]
enum Menu {
    Closed,
    Open(Vec<Food>),
}

/// food, as returned by openmensa. previously, we parsed json using string.split. lets not do
/// that
#[derive(Debug, serde::Deserialize)]
struct OpenMensaFood {
    name: String,
    notes: Vec<String>,
}

#[component]
pub fn MensaView() -> impl IntoView {
    view! {
        <Essen id=String::from("348")/>
    }
}
#[component]
fn Essen(id: String) -> impl IntoView {
    let (state, set_state) = create_signal(ViewState::Closed);

    let cloned_id = id.clone();

    spawn_local(async move {
        // urgh, this is so ugly
        let menu_result = get_menu(&cloned_id).await;

        console_log(format!("{:?}", menu_result).as_str());

        let new_state = match menu_result {
            Ok(ok) => match ok {
                Menu::Closed => ViewState::Closed,
                Menu::Open(x) => ViewState::Open(x),
            },
            Err(_) => ViewState::Error,
        };

        set_state.set(new_state)
    });

    set_interval(
        move || {
            let more_id_clone = id.clone();
            spawn_local(async move {
                // urgh, this is(again) so ugly
                let menu_result = get_menu(&more_id_clone).await;

                let new_state = match menu_result {
                    Ok(ok) => match ok {
                        Menu::Closed => ViewState::Closed,
                        Menu::Open(x) => ViewState::Open(x),
                    },
                    Err(_) => ViewState::Error,
                };

                set_state.set(new_state)
            });
        },
        Duration::from_secs(60 * 30),
    );

    view! {
        <table class="center" id="mensa" >
            <tr>
            {move || {
                match state.get() {
                    ViewState::Closed => view! {
                        <td class="error">
                            Mensa is closed
                        </td>
                    }.into_view(),
                    ViewState::Error => view! {
                        <td class="error">
                            Error :/
                        </td>
                    }.into_view(),
                    ViewState::Open(foodlist) => view! {
                        <For
                            each=move || foodlist.clone()
                            key=|food| food.name.clone()
                            children=move |food: Food| {
                                let style = format!("background-image: url({});
                                                    background-size: 110%;
                                                    background-repeat: no-repeat;
                                                    background-position: center;
                                                    height: 100%;
                                                    width: 100%;
                                                    padding:0px",
                                                    food.image_url.clone());

                                if food.vegan {
                                   view! {
                                        <td style=style>
                                            <div style="width:100%;
                                                        height:auto;
                                                        background:#3d3d3d;
                                                        color:white;">
                                                <div style="width:calc(90% - 20px);
                                                            background-color:#000000;
                                                            color:#ffffff;
                                                            margin:0px;
                                                            overflow:hidden;
                                                            text-overflow:ellipsis;
                                                            height:fit-content;
                                                            padding:10px">
                                                    {food.name.clone()}
                                                </div>
                                                <div style="width:10%;
                                                            padding-top:10px;
                                                            padding-bottom:10px;
                                                            color:white;">
                                                    "V"
                                                </div>
                                            </div>
                                        </td>
                                   }.into_view()
                                } else {
                                    view! {
                                        <td style=style>
                                            <p style="background-color:#000000;
                                                        color:#ffffff;
                                                        margin:0px;
                                                        width:calc(100% - 20px);
                                                        overflow:hidden;
                                                        text-overflow:ellipsis;
                                                        padding:10px;">
                                                {food.name.clone()}
                                            </p>
                                        </td>
                                    }.into_view()
                                }
                            }
                            />
                    }.into_view()
                }
                     }}
            </tr>
        </table>
    }
}

pub async fn get_food_pic(client: &Client, name: &str, date: DateTime<Local>) -> Result<String> {
    let url = String::from(
        "https://www.stw-d.de/gastronomie/speiseplaene/essenausgabe-sued-duesseldorf/",
    );

    let text = client.get(url).send().await?.text().await?;

    let html = Html::parse_document(text.as_str());

    let date_formatted = format!("div[data-date=\"{}\"]", date.format("%d.%m.%Y"));

    let selector =
        Selector::parse(&date_formatted).map_err(|_| anyhow!("failed to parse selector"))?;

    let day = html
        .select(&selector)
        .next()
        .ok_or(anyhow!("no day found"))?;

    let essens_selector =
        Selector::parse("div.counter").map_err(|_| anyhow!("failed to parse selector"))?;

    let url = day
        .select(&essens_selector)
        .map(|x| x.inner_html())
        .find(|x| x.contains(name))
        .ok_or(anyhow!("Could not find image for date {}", date))?
        .split("url(")
        .collect::<Vec<_>>()[1]
        .split(')')
        .collect::<Vec<_>>()[0]
        .replace('\"', "");
    Ok(url)
}

async fn get_menu(_id: &str) -> Result<Menu> {
    let client = reqwest::Client::new();

    let now = chrono::offset::Local::now();
    let current_time = now.time();

    // After 14 o'clock, show tomorrows food
    let current_hour = current_time.hour();
    let current_minute = current_time.minute();
    let mut target_date = if (current_hour >= 15 || (current_hour == 14 && current_minute > 30)) {
        now.checked_add_days(Days::new(1))
            .ok_or(anyhow!("failed to calculate date to fetch"))?
    } else {
        now
    };

    // If were to fetch a day of the weekend, fetch the next monday instead
    let target_weekday = target_date.weekday();
    target_date = match target_weekday {
        Weekday::Sat | Weekday::Sun => target_date
            .checked_add_days(Days::new(
                // num_days_from_sunday gets us the offset to sunday of the last week.
                // e.g. saturdays offset is 6. we want that to be 1, so we invert it by
                // substracting it from seven. we then add 1, to get the monday
                (8 - target_weekday.number_from_monday()).into(),
            ))
            .ok_or(anyhow!("failed to calculate date to fetch"))?,
        _ => target_date,
    };
    stw_d_parser::get_menu_data(&client, target_date).await
}
