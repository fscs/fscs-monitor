use leptos::{*, leptos_dom::logging::console_log};
use serde_json::Value;
use std::time::Duration;
use wasm_bindgen::prelude::*;
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <Station id="20018296".to_string()/>
        </div>
        <div>
            <Station id="20018804".to_string()/>
        </div>
        <div>
            <Station id="20018269".to_string()/>
        </div>
        <div>
            <Station id="20018249".to_string()/>
        </div>
    }
}

#[component]
fn Station(id: String) -> impl IntoView {
    let (state, set_state) = create_signal(vec![vec![String::new(); 10]; 10]);
    let (name, set_name) = create_signal(String::new());
    let id2 = id.clone();

    spawn_local(async move {
        let list = list(id2.clone());
        let list = match list.await {
            Ok(x) => x,
            Err(_) => {
                return;
            }
        };
        list.split('\n').for_each(|_y| {
            set_state.set(
                list.split('\n')
                    .map(|x| x.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            );
        });
        let name = get_station_name(id2.clone());
        let name = match name.await {
            Ok(x) => x,
            Err(_) => {
                return;
            }
        };
        console_log(&list);

        set_name.set(name);
    });

    set_interval(
        move || {
            let id = id.clone();
            spawn_local(async move {
                let list = list(id.clone());
                let list = match list.await {
                    Ok(x) => x,
                    Err(_) => {
                        return;
                    }
                };
                list.split('\n').for_each(|_y| {
                    set_state.set(
                        list.split('\n')
                            .map(|x| x.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>())
                            .collect::<Vec<_>>(),
                    );
                });
                let name = get_station_name(id.clone());
                let name = match name.await {
                    Ok(x) => x,
                    Err(_) => {
                        return;
                    }
                };
                set_name.set(name);
            });
        },
        Duration::from_secs(60),
    );
    

    view! {
        <div class="center" style="height:100%;  ">
            <h1>{name}</h1>
            <table class="center" style="padding-left:30px; padding-right:30px;">
            {move || state.get().iter().map(move |x| {

                 if x[0].is_empty() {
                     view! {
                         <tr class="hidden">
                         </tr>
                     }
                 }else if x[1].len() >= 27{
                     if x[3].clone() == "true" {
                         return view! {
                             <tr>
                                 <th style="color:#f00; text-align:left;">{x[0].clone()}</th>
                                 <th style="color:#f00; text-align:left; line-height:1; max-width:25vw; overflow:hidden;"><div class="scroll" style="color:#ff0000; width:auto;"><span>{x[1].clone()}</span><span>{x[1].clone()}</span><span>{x[1].clone()}</span></div></th>
                                 <th style="color:#f00; text-align:right;">{x[2].clone()}</th>
                             </tr>
                         }
                     }
                     if x[4].clone() == "true" {
                         return view! {
                             <tr>
                                 <th style="color:#ff0; text-align:left;">{x[0].clone()}</th>
                                 <th style="color:#ff0; text-align:left; line-height:1; max-width:25vw; overflow:hidden;"><div class="scroll" style="color:#ff0000; width:auto;"><span>{x[1].clone()}</span><span>{x[1].clone()}</span><span>{x[1].clone()}</span></div></th>
                                 <th style="color:#ff0; text-align:right;">{x[2].clone()}</th>
                             </tr>
                         }
                     }
                     return view! {
                         <tr>
                             <th style="text-align:left;">{x[0].clone()}</th>
                             <th style="text-align:left; line-height:1; max-width:25vw; overflow:hidden;"><div class="scroll" style="color:#00cc00; width:auto;"><span>{x[1].clone()}</span><span>{x[1].clone()}</span><span>{x[1].clone()}</span></div></th>
                             <th style="text-align:right;">{x[2].clone()}</th>
                         </tr>
                        }
                     }
                 else {
                     if x[3].clone() == "true" {
                         return view! {
                             <tr>
                                 <th style="color:#f00; text-align:left;">{x[0].clone()}</th>
                                 <th style="color:#f00; text-align:left; line-height:1; max-width:25vw">{x[1].clone()}</th>
                                 <th style="color:#f00; text-align:right;">{x[2].clone()}</th>
                             </tr>
                         }
                     }
                     if x[4].clone() == "true" {
                         return view! {
                             <tr>
                                 <th style="color:#ff0; text-align:left;">{x[0].clone()}</th>
                                 <th style="color:#ff0; text-align:left; line-height:1; max-width:25vw">{x[1].clone()}</th>
                                 <th style="color:#ff0; text-align:right;">{x[2].clone()}</th>
                             </tr>
                         }
                     }
                     return view! {
                         <tr>
                             <th style="text-align:left;">{x[0].clone()}</th>
                             <th style="text-align:left; line-height:1; max-width:25vw">{x[1].clone()}</th>
                             <th style="text-align:right;">{x[2].clone()}</th>

                         </tr>
                     }
                 }
             }).collect::<Vec<_>>()
            }
            </table>
        </div>
    }
}

#[wasm_bindgen]
pub async fn get_departures(id: String, limit: i32) -> Result<JsValue, JsValue> {
    let url = format!("https://app.vrr.de/vrrstd/XML_DM_REQUEST?outputFormat=JSON&commonMacro=dm&type_dm=any&name_dm={}&language=de&useRealtime=1&lsShowTrainsExplicit=1&mode=direct&typeInfo_dm=stopID&limit={}", id, limit);

    let text = match reqwest::get(url).await {
        Ok(x) => x.text().await,
        Err(_) => {
            return Err(JsValue::from_str("Error"));
        }
    };

    let text = match text {
        Ok(x) => x,
        Err(_) => {
            return Err(JsValue::from_str("Error"));
        }
    };
    Ok(JsValue::from_str(&text))
}

struct Train {
    line: String,
    direction: String,
    time: i32,
    train_type: String,
    canceled: bool,
    onplanned: bool,
}

#[wasm_bindgen]
pub async fn list(id: String) -> Result<String, JsValue> {
    let mut vec = Vec::new();

    let mut limit = 60;
    if id.clone() == "20018249" {
        limit = 200;
    }
    console_log(&id);
    let departures = match get_departures(id.clone(),limit).await {
        Ok(x) => x,
        Err(_) => {
            return Err(JsValue::from_str("Error"));
        } 
    };


    let departures = match departures.as_string() {
        Some(x) => x,
        None => {
            return Err(JsValue::from_str("Error"));
        }
    };

    let json: Value = match serde_json::from_str(&departures) {
        Ok(x) => x,
        Err(_) => {
            return Err(JsValue::from_str("Error"));
        }
    };

    console_log(&json.to_string());

    console_log("meme");


    let mut x = 0;
    while vec.len() < 9 && x < limit{
        let train = get_traindata(json.clone(), x as usize);
        console_log(&train.line);
        console_log(&train.direction.to_string());

        let time = train.time;

        if time >= 3 {
            if id.clone() == "20018249" {
                if ((train.train_type == "S-Bahn") || (train.train_type == "Regionalzug"))
                    && (time > 15)
                {
                    vec.push(train);
                }
            } else if !train.direction.contains("Uni") {
                vec.push(train);
            }
        }
        x += 1;
    }
    

    vec.sort_by(|a, b| a.time.cmp(&b.time));

    Ok(vec
        .iter()
        .map(|x| {
            format!(
                "{} && {} && {}min && {} && {}",
                x.line, x.direction, x.time, x.canceled, x.onplanned
            )
        })
        .collect::<Vec<_>>()
        .join("\n"))
}

#[wasm_bindgen]
pub async fn get_station_name(id: String) -> Result<String, JsValue> {
    let raw = match get_departures(id,4).await {
        Ok(x) => x,
        Err(_) => {
            return Err(JsValue::from_str("Error"));
        }
    };

    let raw = match raw.as_string() {
        Some(x) => x,
        None => {
            return Err(JsValue::from_str("Error"));
        }
    };

    let json: Value = match serde_json::from_str(&raw) {
        Ok(x) => x,
        Err(_) => {
            return Err(JsValue::from_str("Error"));
        }
    };

    let name = json["departureList"][0]["stopName"]
        .to_string()
        .replace('\"', "");
    Ok(name)
}

fn get_traindata(json: Value, id: usize) -> Train {
    let mut path = "realDateTime";
    if json["departureList"][&id]["realDateTime"].to_string().contains("null") {
        path = "dateTime";
    }


    console_log(&path);


    let est_day_train = json["departureList"][&id]["dateTime"]["day"]
        .to_string()
        .replace('\"', "");


    let est_hour_train = json["departureList"][&id]["dateTime"]["hour"]
        .to_string()
        .replace('\"', "");

    let est_minute_train = json["departureList"][&id]["dateTime"]["minute"]
        .to_string()
        .replace('\"', "");

    let est_day_train = match est_day_train.parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            return Train {
                line: String::new(),
                direction: String::new(),
                time: 0,
                train_type: String::new(),
                canceled: false,
                onplanned: false,
            };
        }
    };

    let est_hour_train = match est_hour_train.parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            return Train {
                line: String::new(),
                direction: String::new(),
                time: 0,
                train_type: String::new(),
                canceled: false,
                onplanned: false,
            };
        }
    };

    let est_minute_train = match est_minute_train.parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            return Train {
                line: String::new(),
                direction: String::new(),
                time: 0,
                train_type: String::new(),
                canceled: false,
                onplanned: false,
            };
        }
    };



    console_log(&json["departureList"][&id][path].to_string());
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

    let hour_train = match hour_train.parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            return Train {
                line: String::new(),
                direction: String::new(),
                time: 0,
                train_type: String::new(),
                canceled: false,
                onplanned: false,
            };
        }
    };
    
    let hour_now = match hour_now.parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            return Train {
                line: String::new(),
                direction: String::new(),
                time: 0,
                train_type: String::new(),
                canceled: false,
                onplanned: false,
            };
        }
    };

    let minute_train = match minute_train.parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            return Train {
                line: String::new(),
                direction: String::new(),
                time: 0,
                train_type: String::new(),
                canceled: false,
                onplanned: false,
            };
        }
    };

    let minute_now = match minute_now.parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            return Train {
                line: String::new(),
                direction: String::new(),
                time: 0,
                train_type: String::new(),
                canceled: false,
                onplanned: false,
            };
        }
    };

    let day_train = match day_train.parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            return Train {
                line: String::new(),
                direction: String::new(),
                time: 0,
                train_type: String::new(),
                canceled: false,
                onplanned: false,
            };
        }
    };


    let day_now = match day_now.parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            return Train {
                line: String::new(),
                direction: String::new(),
                time: 0,
                train_type: String::new(),
                canceled: false,
                onplanned: false,
            };
        }
    };

    let mut _real_times = String::new();
    let day_off = day_train - day_now;
    let hour_off = hour_train - hour_now;
    let minute_off = minute_train - minute_now;
    let diff;
    if day_off == 1 {
        diff = 1440 - (hour_now * 60 + minute_now) + (hour_train * 60 + minute_train);
    } else {
        diff = hour_off * 60 + minute_off;
    }
    _real_times = diff.to_string();

    let mut _est_times = String::new();
    let day_off = est_day_train - day_now;
    let hour_off = est_hour_train - hour_now;
    let minute_off = est_minute_train - minute_now;
    let diff;
    if day_off == 1 {
        diff = 1440 - (hour_now * 60 + minute_now) + (hour_train * 60 + minute_train);
    } else {
        diff = hour_off * 60 + minute_off;
    }
    _est_times = diff.to_string();

    let mut _onplanned = false;
    if (_real_times.parse::<i32>().unwrap() - _est_times.parse::<i32>().unwrap()) > 5 {
        _onplanned = true;
    }
    console_log(&json["departureList"][&id]["servingLine"]["number"].to_string());
    console_log("times");
    console_log(&_est_times);
    console_log(&_real_times);



    console_log(&_real_times);

    Train {
        line: json["departureList"][&id]["servingLine"]["number"]
            .to_string()
            .replace('\"', "")
            .replace(" (RRX)", ""),
        direction: json["departureList"][&id]["servingLine"]["direction"]
            .to_string()
            .replace('\"', "")
            +" ",
        time: _real_times.parse::<i32>().unwrap(),
        train_type: json["departureList"][&id]["servingLine"]["name"]
            .to_string()
            .replace('\"', ""),
        canceled: json["departureList"][&id]["realtimeTripStatus"]
            .to_string()
            .contains("TRIP_CANCELLED"),
        onplanned: _onplanned,
    }


}
