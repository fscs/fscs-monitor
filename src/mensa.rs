use leptos::{leptos_dom::logging::console_log, *};
use std::time::Duration;
use wasm_bindgen::prelude::*;
#[component]
pub fn App2() -> impl IntoView {
    view! {
        <Essen id="348".to_string()/>
    }
}

#[component]
fn Essen(id: String) -> impl IntoView {
    let (state, set_state) = create_signal(vec![vec![String::new()]]);

    let id2 = id.clone();

    spawn_local(async move {
        let list = get_menu(id2.clone());
        let list = list.await;
        set_state.set(
            list.split('\n')
                .map(|x| x.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
    });

    set_interval(
        move || {
            let id = id.clone();
            spawn_local(async move {
                let list = get_menu(id);
                let list = list.await;
                set_state.set(
                    list.split('\n')
                        .map(|x| x.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>())
                        .collect::<Vec<_>>(),
                );
            });
        },
        Duration::from_secs(60 * 30),
    );

    view! {
        <table class="center" id="mensa" >
            <tr>
            {move || state.get().iter().map(move |x| {
                console_log(&x[0]);
                if x[0] == "mensa is closed" {
                    return view! {
                        <td class="error">
                            Mensa is closed
                        </td>
                    };
                }
                 if x[0].is_empty() {
                     view! {
                         <td class="hidden">
                         </td>
                     }
                 }else{
                     let style = format!("background-image: url({}); background-size: 110%; background-repeat: no-repeat; background-position: center; height: 100%; width: 100%; padding:0px", x[2].clone());
                         if x[3].clone() == "true" {
                             return view! {
                                 <td style=style>
                                     <div style="width:100%; height:auto; background:#3d3d3d; color:white;">
                                        <div style="width:calc(90% - 20px); background-color:#000000; color:#ffffff; margin:0px;overflow:hidden; text-overflow:ellipsis; height:fit-content;padding:10px">
                                            {x[1].clone()} </div>
                                        <div style="width:10%;padding-top:10px;padding-bottom:10px;color:white;">
                                            "V"
                                        </div>
                                    </div>
                                 </td>
                             }
                         }
                         view! {
                             <td style=style>
                                 <p style="background-color:#000000; color:#ffffff; margin:0px; width:calc(100% - 20px);overflow:hidden; text-overflow:ellipsis;padding:10px;">
                                    {x[1].clone()}
                                 </p>
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
pub async fn get_food_pic(id: String) -> Result<JsValue, JsValue> {
    let mut today = chrono::Local::now().format("%d.%m.%Y").to_string();
    let _time = chrono::Local::now().format("%H:%M").to_string();
    let hour = chrono::Local::now().format("%H").to_string().parse::<i32>().map_err(|e| {
        console_log(&format!("{:?}", e));
        JsValue::from_str("error")
    })?;
    let minute = chrono::Local::now().format("%M").to_string().parse::<i32>().map_err(|e| {
        console_log(&format!("{:?}", e));
        JsValue::from_str("error")
    })?;


    let weekday = chrono::Local::now().format("%u").to_string().parse::<i32>().map_err(|e| {
        console_log(&format!("{:?}", e));
        JsValue::from_str("error")
    })?;

    if weekday >= 5 {
        //set day to monday
        let diff_to_next_monday = 8 - chrono::Local::now()
            .format("%u")
            .to_string()
            .parse::<i64>()
            .map_err(|e| {
                console_log(&format!("{:?}", e));
                JsValue::from_str("error")
            })?;
        today = chrono::Local::now()
            .checked_add_signed(chrono::Duration::days(diff_to_next_monday))
            .unwrap()
            .format("%d.%m.%Y")
            .to_string();
    } else if hour > 14 || (hour == 14 && minute > 30){
        //set day to tomorrow
        today = chrono::Local::now()
            .checked_add_signed(chrono::Duration::days(1))
            .unwrap()
            .format("%d.%m.%Y")
            .to_string();
    }

    let url =
        "https://www.stw-d.de/gastronomie/speiseplaene/essenausgabe-sued-duesseldorf/".to_string();
    let text = match reqwest::get(url).await {
        Ok(x) => x.text().await,
        Err(e) => {
            console_log(&format!("{:?}", e));
            return Ok(JsValue::from_str("error"));
        }
    };
    let text = match text {
        Ok(x) => x,
        Err(e) => {
            console_log(&format!("{:?}", e));
            return Ok(JsValue::from_str("error"));
        }
    };
    let day = format!("data-date='{}'>", today);
    let day_info = text.split(&day);

    let essen = day_info.collect::<Vec<_>>()[1]
        .split("</div>")
        .collect::<Vec<_>>();

    for i in 0..essen.len() {
        if essen[i].contains(&id) {
            let url = essen[i].split("url(").collect::<Vec<_>>()[1]
                .split(')')
                .collect::<Vec<_>>()[0]
                .replace('\"', "");
            return Ok(JsValue::from_str(&url));
        }
    }
    Ok(JsValue::from_str(&text))
}

#[wasm_bindgen]
pub async fn get_menu(id: String) -> String {
    let mut day = chrono::Local::now().format("%Y-%m-%d").to_string();
    let _time = chrono::Local::now().format("%H:%M").to_string();
    let hour = match chrono::Local::now().format("%H").to_string().parse:: <i32>() {
        Ok(x) => x,
        Err(e) => {
            console_log(&format!("{:?}", e));
            return "mensa is closed".to_string();
        }
    };
    let minute = match chrono::Local::now().format("%M").to_string().parse:: <i32>() {
        Ok(x) => x,
        Err(e) => {
            console_log(&format!("{:?}", e));
            return "mensa is closed".to_string();
        }
    };

    let weekday = match chrono::Local::now().format("%u").to_string().parse:: <i32>() {
        Ok(x) => x,
        Err(e) => {
            console_log(&format!("{:?}", e));
            return "mensa is closed".to_string();
        }
    };

    if weekday >= 5 {
        //set day to monday
        let diff_to_next_monday = 8 - chrono::Local::now()
            .format("%u")
            .to_string()
            .parse::<i64>()
            .unwrap();
        day = chrono::Local::now()
            .checked_add_signed(chrono::Duration::days(diff_to_next_monday))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();
    } else if hour > 14 || (hour == 14 && minute > 30) {
        //set day to tomorrow
        day = chrono::Local::now()
            .checked_add_signed(chrono::Duration::days(1))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();
    }
    let text = reqwest::get(format!(
        "https://openmensa.org/api/v2/canteens/{}/days/{}/meals",
        id, day
    ))
    .await;
    let text = match text {
        Ok(x) => x.text().await,
        Err(e) => {
            console_log(&format!("{:?}", e));
            return "mensa is closed".to_string();
        }
    };
    let text = match text {
        Ok(x) => x,
        Err(e) => {
            console_log(&format!("{:?}", e));
            return "mensa is closed".to_string();
        }
    };



    let mut essen = String::new();
    for i in 0..text.matches("name").count() {
        let essen_name = text.split("name\":").collect::<Vec<_>>()[i + 1]
            .split(',')
            .collect::<Vec<_>>()[0]
            .replace('\"', "");

        let essen_category = text.split("category\":").collect::<Vec<_>>()[i + 1]
            .split(',')
            .collect::<Vec<_>>()[0]
            .replace('\"', "");
        let is_vegan = text.split("notes\":").collect::<Vec<_>>()[i + 1].contains("vegan");
        console_log(&essen_category);
        console_log(
            &get_food_pic(essen_category.clone())
                .await
                .unwrap()
                .as_string()
                .unwrap()
                .to_string(),
        );
        let pic_url = match get_food_pic(essen_category.clone()).await.unwrap().as_string() {
            Some(x) => x,
            None => "mensa is closed".to_string(),
        };
        if pic_url.contains(essen_category.as_str()) {
        console_log(&pic_url);
        console_log("true");
        essen.push_str(&format!("{} && {} && {} && {}\n",essen_category, essen_name, pic_url, is_vegan));
        }else{
            return "mensa is closed".to_string();
        }
    }
    console_log(&essen);
    essen
}
