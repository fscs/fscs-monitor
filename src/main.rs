use std::time::Duration;
use leptos::{*, leptos_dom::logging::console_log};
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Request, Response, RequestCache};
fn main() {
    // create a new app
    leptos::mount_to_body(move || view! { 
        <div style="height:5vh; width:100vw">
            <Notification_Bar/>
        </div>
        <div style="height:75vh; width:100vw">
          <App/>  
        </div>
        <div style="height:20vh; width:100vw">
          <App2/>
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
        <div class="center" style="height:100%">
            <h1>{ move || localtime}</h1>
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

#[component]
fn App() -> impl IntoView {
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
fn App2() -> impl IntoView {
    view! {
            <Essen id="348".to_string()/> 
    }
}

#[component]
fn Essen(id:String) -> impl IntoView { 
    let (state,set_state) = create_signal(vec![vec![String::new()]]);

    let id2 = id.clone();

    spawn_local(async move {
        let list = get_menu(id2.clone());
        let list = list.await.unwrap().as_string().unwrap();
        set_state.set(list.split("\n").map(|x| x.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>());
    });

    set_interval(
        move || {
            let id = id.clone();
            spawn_local(async move {
                let list = get_menu(id.clone());
                let list = list.await.unwrap().as_string().unwrap();
                set_state.set(list.split("\n").map(|x| x.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>());
            });
        }, Duration::from_secs(60*60*30),
        );

   
    view! {
        <table class="center" style="table-layout:fixed; height:100%; padding: 15pt; background-image:url('https://static.planetminecraft.com/files/image/minecraft/texture-pack/2020/428/13530476-cover_l.jpg'); background-size:contain">
            <tr> 
            {move || state.get().iter().map(move |x| {
                                                         if x[0].is_empty() {
                                                             return view! {
                                                                 <td class="hidden">
                                                                     </td>
                                                             };
                                                         }else{
                                                             let style = format!("background-image: url({}); background-size: 110%; background-repeat: no-repeat; background-position: center; height: 100%; width: 100%; padding:0px", x[2].clone());
                                                             if x[3].clone() == "true" {
                                                                 return view! {
                                                                 <td style=style>
                                                                     <div style="width:100%; height:auto; background:#3d3d3d; color:white;">
                                                                        <div style="width:90%; background-color:#000000; color:#ffffff; margin:0px;overflow:hidden; text-overflow:ellipsis; height:fit-content;padding-top:10px;padding-bottom:10px;">
                                                                        {x[1].clone()} </div>
                                                                        <div style="width:10%;padding-top:10px;padding-bottom:10px;color:white;">
                                                                        "V"
                                                                        </div>
                                                                    </div>
                                                                     </td>
                                                             }
                                                             }
                                                             return view! {
                                                                 <td style=style> 
                                                                     <p style="background-color:#000000; color:#ffffff; margin:0px; width:100%;overflow:hidden; text-overflow:ellipsis;padding-top:10px;padding-bottom:10px;">

                                                                        {x[1].clone()}</p>
                                                                     </td>
                                                             }
                                                         }
                                                     }).collect::<Vec<_>>()
            } 
        </tr>
        </table>
    }
}



#[wasm_bindgen]
pub async fn get_food_pic(id:String) -> Result<JsValue, JsValue> {
    let mut today = chrono::Local::now().format("%d.%m.%Y").to_string();
    let time = chrono::Local::now().format("%H:%M").to_string();
    if time > "14:30".to_string() {
        //set day to tomorrow
        today = chrono::Local::now().checked_add_signed(chrono::Duration::days(1)).unwrap().format("%d.%m.%Y").to_string(); 
    }
    let mut opts = RequestInit::new();
    opts.mode(RequestMode::Cors);
    opts.method("GET");
    opts.cache(RequestCache::NoStore);
    opts.mode(RequestMode::Cors);
    let url = format!("https://www.stw-d.de/gastronomie/speiseplaene/essenausgabe-sued-duesseldorf/"); 
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Reuponse` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let text = JsFuture::from(resp.text()?).await?.as_string().unwrap(); 
    let day = format!("data-date='{}'>", today);
    let day_info = text.split(&day);


    let essen = day_info.collect::<Vec<_>>()[1].split("</div>").collect::<Vec<_>>();
    
    console_log(&essen[0]);

    for i in 0..essen.len() {
        if essen[i].contains(&id) {
            console_log("found");
            let url = essen[i].split("url(").collect::<Vec<_>>()[1].split(")").collect::<Vec<_>>()[0].replace("\"", "");
            console_log(&url);
            return Ok(JsValue::from_str(&url));
        }
    }
    
    Ok(JsValue::from_str(&text))
}




#[wasm_bindgen]
pub async fn get_menu(id:String) ->Result<JsValue, JsValue> {
    let mut day = chrono::Local::now().format("%Y-%m-%d").to_string();
    let time = chrono::Local::now().format("%H:%M").to_string();
    if time > "14:30".to_string() {
        //set day to tomorrow
        day = chrono::Local::now().checked_add_signed(chrono::Duration::days(1)).unwrap().format("%Y-%m-%d").to_string(); 
    }
    let mut opts = RequestInit::new();
    opts.mode(RequestMode::Cors);
    opts.method("GET");
    opts.cache(RequestCache::NoStore);
    opts.mode(RequestMode::Cors);
    let url = format!("https://openmensa.org/api/v2/canteens/{}/days/{}/meals", id, &day); 
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Reuponse` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let text = JsFuture::from(resp.text()?).await?.as_string().unwrap(); 
    let mut essen = String::new();
    for i in 0..text.matches("name").count() {
    
        let essen_name = text.split("name\":").collect::<Vec<_>>()[i+1].split(",").collect::<Vec<_>>()[0].replace("\"", "");

        let essen_category = text.split("category\":").collect::<Vec<_>>()[i+1].split(",").collect::<Vec<_>>()[0].replace("\"", "");
        let is_vegan = text.split("notes\":").collect::<Vec<_>>()[i+1].contains("vegan"); 

        let pic_url = get_food_pic(essen_category.clone()).await.unwrap().as_string().unwrap();

        
        essen = format!("{} {} && {} && {} && {}\n", essen, essen_category, essen_name, pic_url, is_vegan);

    }

    Ok(JsValue::from_str(&essen))
}

#[wasm_bindgen]
pub async fn get_departures(id:String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.mode(RequestMode::Cors);
    opts.method("GET");
    opts.cache(RequestCache::NoStore);
    opts.mode(RequestMode::Cors);
    console_log(&id);
    let mut url = format!("https://app.vrr.de/vrrstd/XML_DM_REQUEST?outputFormat=JSON&commonMacro=dm&type_dm=any&name_dm={}&language=de&realtime=1&lsShowTrainsExplicit=1&mode=direct&typeInfo_dm=stopID", id); 
    if id=="20018249" {
       url = format!("https://app.vrr.de/vrrstd/XML_DM_REQUEST?outputFormat=JSON&commonMacro=dm&type_dm=any&name_dm={}&language=de&realtime=1&lsShowTrainsExplicit=1&mode=direct&typeInfo_dm=stopID&limit=100", id); 
    }
    let request = Request::new_with_str_and_init(&url, &opts)?;
    console_log(&request.method());
    console_log(&url);
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Reuponse` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let text = JsFuture::from(resp.text()?).await?.as_string().unwrap();


    Ok(JsValue::from_str(&text))

}

#[wasm_bindgen]
pub async fn list(id:String) -> Result<String, JsValue> {

    let mut vec = Vec::new();

    let json = get_departures(id.clone()).await.unwrap().as_string().unwrap();

    let json: Value = serde_json::from_str(&json).unwrap();

    
    let mut i = 0;

    console_log(&json["departureList"].to_string().matches("servingLine").count().to_string());

    while (vec.len() <9) && (i < json["departureList"].to_string().matches("servingLine").count()) {
        let line = json["departureList"][i]["servingLine"]["number"].to_string().replace("\"", "");
        let direction = json["departureList"][i]["servingLine"]["direction"].to_string().replace("\"", "");

        let train_type = json["departureList"][i]["servingLine"]["name"].to_string().replace("\"", "");

        let day_train = json["departureList"][i]["dateTime"]["day"].to_string().replace("\"", "");
        let day_now = chrono::Local::now().format("%d").to_string();

        let hour_train = json["departureList"][i]["dateTime"]["hour"].to_string().replace("\"", ""); 
        let hour_now = chrono::Local::now().format("%H").to_string();

        let minute_train = json["departureList"][i]["dateTime"]["minute"].to_string().replace("\"", "");
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

        let mut arr: [&str; 3] = [""; 3]; 

        arr[0] = &line;
        arr[1] = &direction;
        arr[2] = &_times;

        let string = format!("{} && {} && {}min", arr[0], arr[1], arr[2]);
        if _times.parse::<i32>().unwrap() >= 5 {
            if id.clone() == "20018249" {
                console_log(&train_type);
                if ((train_type == "S-Bahn") || (train_type == "Regionalzug"))&&(_times.parse::<i32>().unwrap() > 15) {
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
