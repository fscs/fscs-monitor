use anyhow::{anyhow, Result};
use chrono::prelude::*;
use chrono::Days;
use leptos::{leptos_dom::logging::console_log, *};
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

#[component]
pub fn MensaView() -> impl IntoView {
    view! {
        <Essen id=String::from("essenausgabe-sued-duesseldorf")/>
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

async fn get_menu(id: &str) -> Result<Menu> {
    let client = reqwest::Client::new();

    let now = chrono::offset::Local::now();
    stw_d_parser::get_menu_data(id, &client, get_target_day(now)).await
}

fn get_target_day(now: DateTime<Local>) -> DateTime<Local> {
    let current_time = now.time();
    // After 14 o'clock, show tomorrows food
    let current_hour = current_time.hour();
    let current_minute = current_time.minute();
    let mut target_date = if (current_hour >= 15 || (current_hour == 14 && current_minute > 30)) {
        now.checked_add_days(Days::new(1))
            .expect("failed to calculate date to fetch")
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
            .expect("failed to calculate date to fetch"),
        _ => target_date,
    };
    target_date
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_menu() {
        let id = "essenausgabe-sued-duesseldorf";
        let menu = get_menu(id).await;
        assert!(menu.is_ok());
    }
    #[test]
    fn test_get_target_day() {
        let test_date = "2024-02-22T14:31:00+01:00";
        let now = DateTime::parse_from_rfc3339(test_date).unwrap();
        let target_day = get_target_day(now.into());
        assert_eq!(target_day.date_naive().day(), 23);
    }
    #[test]
    fn test_get_target_day_with_weekend_skip_fr() {
        let test_date = "2024-02-23T14:31:00+01:00";
        let now = DateTime::parse_from_rfc3339(test_date).unwrap();
        let target_day = get_target_day(now.into());
        assert_eq!(target_day.date_naive().day(), 26);
    }
    #[test]
    fn test_get_target_day_with_weekend_skip_sa() {
        let test_date = "2024-02-24T14:31:00+01:00";
        let now = DateTime::parse_from_rfc3339(test_date).unwrap();
        let target_day = get_target_day(now.into());
        assert_eq!(target_day.date_naive().day(), 26);
    }
    #[test]
    fn test_get_target_day_with_weekend_skip_so() {
        let test_date = "2024-02-25T14:31:00+01:00";
        let now = DateTime::parse_from_rfc3339(test_date).unwrap();
        let target_day = get_target_day(now.into());
        assert_eq!(target_day.date_naive().day(), 26);
    }
}
