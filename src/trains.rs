use anyhow::Result;
use leptos::*;
use serde_json::Value;
use std::{fmt::Debug, time::Duration};

#[derive(Clone, Debug, Default)]
pub struct Train {
    line: String,
    direction: String,
    time: i32,
    train_type: String,
    canceled: bool,
    onplanned: bool,
    delay: i32,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <Station id=String::from("20018296") limit=60/>
        </div>
        <div>
            <Station id=String::from("20018804") limit=60/>
        </div>
        <div>
            <Station id=String::from("20018269") limit=60/>
        </div>
        <div>
            <Station id=String::from("20018249") limit=200/>
        </div>
    }
}

#[component]
fn Station(id: String, limit: i32) -> impl IntoView {
    let (state, set_state) = create_signal(vec![Train::default()]);
    let (name, set_name) = create_signal(String::new());
    let id2 = id.clone();

    spawn_local(async move {
        let name = get_station_name(id2.clone());
        let name = match name.await {
            Ok(x) => x,
            Err(_) => {
                return;
            }
        };

        set_name.set(name);
        let station = match list(id2.clone(), limit).await {
            Ok(x) => x,
            Err(_) => {
                return;
            }
        };
        set_state.set(station);
    });

    set_interval(
        move || {
            let id = id.clone();
            spawn_local(async move {
                let name = get_station_name(id.clone());
                let name = match name.await {
                    Ok(x) => x,
                    Err(_) => {
                        return;
                    }
                };
                set_name.set(name);

                let station = match list(id.clone(), limit).await {
                    Ok(x) => x,
                    Err(_) => {
                        return;
                    }
                };
                set_state.set(station);
            });
        },
        Duration::from_secs(60),
    );

    view! {
        <div class="center" style="height:100%;  ">
            <h1>{name}</h1>
            <table class="center" style="padding-left:30px; padding-right:30px;">
            {move || state.get().iter_mut().map(move |x| {

                 if x.direction.len() >= 20{
                     x.direction = x.direction.chars().take(19).collect::<String>();
                }
                     if x.canceled {
                         return view! {
                             <tr class="hidden">
                             </tr>
                         }
                     }
                     if x.onplanned {
                         return view! {
                             <tr>
                                 <th style="color:#ff0; text-align:left;">{x.line.clone()}</th>
                                 <th style="color:#ff0; text-align:left; line-height:1; max-width:25vw">{x.direction.clone()}</th>
                                 <th style="color:#ff0; text-align:right;">"(+"{x.delay}") " {x.time}m</th>
                             </tr>
                         }
                     }
                     view! {
                         <tr>
                             <th style="text-align:left;">{x.line.clone()}</th>
                             <th style="text-align:left; line-height:1; max-width:25vw">{x.direction.clone()}</th>
                             <th style="text-align:right;">{x.time}min</th>

                         </tr>
                     }
             }).collect::<Vec<_>>()
            }
            </table>
        </div>
    }
}

pub async fn get_departures(id: String, limit: i32) -> Result<String> {
    let url = format!("https://app.vrr.de/vrrstd/XML_DM_REQUEST?outputFormat=JSON&commonMacro=dm&type_dm=any&name_dm={}&language=de&useRealtime=1&lsShowTrainsExplicit=1&mode=direct&typeInfo_dm=stopID&limit={}", id, limit);

    let text = reqwest::get(&url).await?.text().await?;

    Ok(text)
}

pub async fn list(id: String, limit: i32) -> Result<Vec<Train>> {
    let mut vec = Vec::new();

    let departures = get_departures(id.clone(), limit).await?;
    let json: Value = serde_json::from_str(&departures)?;

    for count in 0..limit {
        let train = get_traindata(&json, count as usize)?;
        let time = train.time;
        if !train.canceled && time >= 3 {
            if id.clone() == "20018269" {
                if time > 15 {
                    vec.push(train);
                }
            } else if id.clone() == "20018249" {
                if ((train.train_type == "S-Bahn") || (train.train_type == "Regionalzug"))
                    && (time > 15)
                {
                    vec.push(train);
                }
            } else if !train.direction.contains("Uni") {
                vec.push(train);
            }
        }
    }

    vec.truncate(9);

    Ok(vec)
}

pub async fn get_station_name(id: String) -> Result<String> {
    let raw = get_departures(id, 4).await?;
    let json: Value = serde_json::from_str(&raw)?;
    let name = json["departureList"][0]["stopName"]
        .to_string()
        .replace('\"', "");
    Ok(name)
}

fn get_traindata(json: &Value, id: usize) -> Result<Train> {
    let mut path = "realDateTime";
    if json["departureList"][&id]["realDateTime"]
        .to_string()
        .contains("null")
    {
        path = "dateTime";
    }

    let est_day_train = json["departureList"][&id]["dateTime"]["day"]
        .to_string()
        .replace('\"', "");

    let est_hour_train = json["departureList"][&id]["dateTime"]["hour"]
        .to_string()
        .replace('\"', "");

    let est_minute_train = json["departureList"][&id]["dateTime"]["minute"]
        .to_string()
        .replace('\"', "");

    let est_day_train = est_day_train.parse::<i32>()?;

    let est_hour_train = est_hour_train.parse::<i32>()?;

    let est_minute_train = est_minute_train.parse::<i32>()?;

    let day_train = json["departureList"][&id][path]["day"]
        .to_string()
        .replace('\"', "");
    let day_now = chrono::Local::now().format("%d").to_string();

    let hour_train = json["departureList"][&id][path]["hour"]
        .to_string()
        .replace('\"', "");
    let hour_now = chrono::Local::now().format("%H").to_string();

    let minute_train = json["departureList"][&id][path]["minute"]
        .to_string()
        .replace('\"', "");
    let minute_now = chrono::Local::now().format("%M").to_string();

    let hour_train = hour_train.parse::<i32>()?;

    let hour_now = hour_now.parse::<i32>()?;

    let minute_train = minute_train.parse::<i32>()?;

    let minute_now = minute_now.parse::<i32>()?;

    let day_train = day_train.parse::<i32>()?;

    let day_now = day_now.parse::<i32>()?;

    let mut _real_times = 0;
    let day_off = day_train - day_now;
    let hour_off = hour_train - hour_now;
    let minute_off = minute_train - minute_now;
    if day_off == 1 {
        _real_times = 1440 - (hour_now * 60 + minute_now) + (hour_train * 60 + minute_train);
    } else {
        _real_times = hour_off * 60 + minute_off;
    }

    let mut _est_times = 0;
    let day_off = est_day_train - day_now;
    let hour_off = est_hour_train - hour_now;
    let minute_off = est_minute_train - minute_now;
    let mut _diff = 0;
    if day_off == 1 {
        _diff = 1440 - (hour_now * 60 + minute_now) + (hour_train * 60 + minute_train);
    } else {
        _diff = hour_off * 60 + minute_off;
    }
    _est_times = _diff;

    let mut _onplanned = false;
    if (_real_times - _est_times) > 5 {
        _onplanned = true;
    }

    let _delay = _real_times - _est_times;

    Ok(Train {
        line: json["departureList"][&id]["servingLine"]["number"]
            .to_string()
            .replace('\"', "")
            .replace(" (RRX)", ""),
        direction: json["departureList"][&id]["servingLine"]["direction"]
            .to_string()
            .replace('\"', "")
            + " ",
        time: _real_times,
        delay: _delay,

        train_type: json["departureList"][&id]["servingLine"]["name"]
            .to_string()
            .replace('\"', ""),
        canceled: json["departureList"][&id]["realtimeTripStatus"]
            .to_string()
            .contains("TRIP_CANCELLED"),
        onplanned: _onplanned,
    })
}
