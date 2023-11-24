use std::time::Duration;
use leptos::{*, leptos_dom::logging::console_log};
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Request, Response, RequestCache};


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
fn Station(id:String) -> impl IntoView {
   
    let (state,set_state) = create_signal(vec![vec![String::new();10]; 10]);
    let (name, set_name) = create_signal(String::new());
    let id2 = id.clone(); 

    spawn_local(async move {
        let list = list(id2.clone());
        let list = list.await.unwrap();
        list.split("\n").for_each(|_y| {
            set_state.set(list.split("\n").map(|x| x.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>());                });
        let name = get_station_name(id2.clone());
        let name = name.await.unwrap();
        set_name.set(name);
    });

    set_interval(
        move || {
            let id = id.clone();
            spawn_local(async move {
                let list = list(id.clone());
                let list = list.await.unwrap();
                list.split("\n").for_each(|_y| {
                    set_state.set(list.split("\n").map(|x| x.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>());                });
                let name = get_station_name(id.clone());
                let name = name.await.unwrap();
                set_name.set(name);
            });
        },
        Duration::from_secs(60),
    );

    return view! {
        <div class="center" style="height:100%;  ">
            <h1>{name}</h1>
            <table class="center" style="padding-left:30px; padding-right:30px;">
            {move || state.get().iter().map(move |x| {
                 if x[0].is_empty() {
                     return view! {
                         <tr class="hidden">
                             </tr>
                     };
                 }else{
                     return view! {
                         <tr>
                             <th>{x[0].clone()}</th>
                             <th style="text-align:left; line-height:1;">{x[1].clone()}</th>
                             <th>{x[2].clone()}</th>

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
pub async fn get_departures(id:String) -> Result<JsValue, JsValue> {

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.cache(RequestCache::NoStore);
    console_log(&id);
    let mut url = format!("https://app.vrr.de/vrrstd/XML_DM_REQUEST?outputFormat=JSON&commonMacro=dm&type_dm=any&name_dm={}&language=de&realtime=1&lsShowTrainsExplicit=1&mode=direct&typeInfo_dm=stopID", id); 
    if id=="20018249" {
       url = format!("https://app.vrr.de/vrrstd/XML_DM_REQUEST?outputFormat=JSON&commonMacro=dm&type_dm=any&name_dm={}&language=de&realtime=1&lsShowTrainsExplicit=1&mode=direct&typeInfo_dm=stopID&limit=100", id); 
    }
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?.as_string().unwrap();

    Ok(JsValue::from_str(&text))

}


struct Train {
    line: String,
    direction: String,
    time: String,
    train_type: String,
}

#[wasm_bindgen]
pub async fn list(id:String) -> Result<String, JsValue> {

    let mut vec = Vec::new();

    let json = get_departures(id.clone()).await.unwrap().as_string().unwrap();

    let json: Value = serde_json::from_str(&json).unwrap();

    let mut i = 1;
    
    while vec.len() <9 {
        
        let train = get_traindata(json.clone(), i);
        
        let string = format!("{} && {} && {}min", train.line, train.direction, train.time);
        
        console_log(&string);

        if train.time.parse::<i32>().unwrap() >= 5 {
            if id.clone() == "20018249" {
                console_log(&train.train_type);
                if ((train.train_type == "S-Bahn") || (train.train_type == "Regionalzug"))&&(train.time.parse::<i32>().unwrap() > 15) {
                    vec.push(string);
                }
            }else{ 
                vec.push(string);
            }
        }
        i = i+1;
    }

    console_log(&vec.join("\n"));
    Ok(vec.join("\n"))



}

#[wasm_bindgen]
pub async fn get_station_name(id:String) -> Result<String, JsValue> {

    let json = get_departures(id).await.unwrap().as_string().unwrap();

    let json: Value = serde_json::from_str(&json).unwrap();

    let name = json["departureList"][0]["stopName"].to_string().replace("\"", "");
    Ok(name)

}

fn get_traindata(json:Value, id:usize) -> Train {
    let day_train = json["departureList"][&id]["dateTime"]["day"].to_string().replace("\"", "");
    let day_now = chrono::Local::now().format("%d").to_string();

    let hour_train = json["departureList"][&id]["dateTime"]["hour"].to_string().replace("\"", ""); 
    let hour_now = chrono::Local::now().format("%H").to_string();

    let minute_train = json["departureList"][&id]["dateTime"]["minute"].to_string().replace("\"", "");
    let minute_now = chrono::Local::now().format("%M").to_string();

    let hour_train = hour_train.parse::<i32>().unwrap();
    let hour_now = hour_now.parse::<i32>().unwrap();
    let minute_train = minute_train.parse::<i32>().unwrap();
    let minute_now = minute_now.parse::<i32>().unwrap();
    let day_train = day_train.parse::<i32>().unwrap();
    let day_now = day_now.parse::<i32>().unwrap();

    let mut _times = String::new();
    let day_off = day_train - day_now;
    let hour_off = hour_train - hour_now;
    let minute_off = minute_train - minute_now;
    let diff;
    if day_off == 1 {
        diff = 1440 - (hour_now * 60 + minute_now) + (hour_train * 60 + minute_train);
    }else {
        diff = hour_off * 60 + minute_off;
    }
    _times = diff.to_string();

    Train {
        line: json["departureList"][&id]["servingLine"]["number"].to_string().replace("\"", ""),
        direction: json["departureList"][&id]["servingLine"]["direction"].to_string().replace("\"", ""),
        time: _times,
        train_type: json["departureList"][&id]["servingLine"]["name"].to_string().replace("\"", ""),
    }
}
