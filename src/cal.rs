use std::time::Duration;

use wasm_bindgen::prelude::*;
use leptos::{leptos_dom::logging::console_log, component, IntoView, create_signal, view, SignalSet, SignalGet, set_interval};
use wasm_bindgen_futures::spawn_local;


struct Event {
    title: String,
    start: Date,
    end: Date,
    location: String,
    description: String,
}

struct Date {
    day: String,
    month: String,
    year: String,
    hour: String,
    minute: String,
}

#[wasm_bindgen]
pub async fn memes() -> String {
    
    let mut vec = vec![Event {
        title: String::new(),
        start: Date {
            day: String::new(),
            month: String::new(),
            year: String::new(),
            hour: String::new(),
            minute: String::new(),
        },
        end: Date {
            day: String::new(),
            month: String::new(),
            year: String::new(),
            hour: String::new(),
            minute: String::new(),
        },
        location: String::new(),
        description: String::new(),
    }];

    console_log("test");

    let timestamp = chrono::Utc::now().timestamp();

    let url = format!("https://nextcloud.inphima.de/remote.php/dav/public-calendars/CAx5MEp7cGrQ6cEe?start={}&export=&componentType=VEVENT", timestamp);
    

    let resp = reqwest::get(url).await.unwrap();
    for i in resp.text().await.unwrap().split("UID:").collect::<Vec<_>>() {
        let i = i.replace("\\", "");
        if vec.len() > 9 {
            break;
        }
        let mut event = Event {
            title: String::new(),
            start: Date {
                day: String::new(),
                month: String::new(),
                year: String::new(),
                hour: String::new(),
                minute: String::new(),
            },
            end: Date {
                day: String::new(),
                month: String::new(),
                year: String::new(),
                hour: String::new(),
                minute: String::new(),
            },
            location: String::new(),
            description: String::new(),
        };


       
        if i.contains("SUMMARY:") {
            event.title = i.split("SUMMARY:").collect::<Vec<_>>()[1].split("\n").collect::<Vec<_>>()[0].to_string();
            console_log(i.split("SUMMARY:").collect::<Vec<_>>()[1].split("\n").collect::<Vec<_>>()[0]);
        }

        if i.contains("DTSTART;TZID=Europe/Berlin:") {
            event.start.year = i.split("DTSTART;TZID=Europe/Berlin:").collect::<Vec<_>>()[1].split("T").collect::<Vec<_>>()[0].to_string()[0..4].to_string();
            event.start.month = i.split("DTSTART;TZID=Europe/Berlin:").collect::<Vec<_>>()[1].split("T").collect::<Vec<_>>()[0].to_string()[4..6].to_string();
            event.start.day = i.split("DTSTART;TZID=Europe/Berlin:").collect::<Vec<_>>()[1].split("T").collect::<Vec<_>>()[0].to_string()[6..8].to_string();
            event.start.hour = i.split("DTSTART;TZID=Europe/Berlin:").collect::<Vec<_>>()[1].split("T").collect::<Vec<_>>()[1].to_string()[0..2].to_string();
            event.start.minute = i.split("DTSTART;TZID=Europe/Berlin:").collect::<Vec<_>>()[1].split("T").collect::<Vec<_>>()[1].to_string()[2..4].to_string();

            console_log(&event.start.year);
        }

        if i.contains("DTEND;TZID=Europe/Berlin:") {
            event.end.year = i.split("DTSTART;TZID=Europe/Berlin:").collect::<Vec<_>>()[1].split("T").collect::<Vec<_>>()[0].to_string()[0..4].to_string();
            event.end.month = i.split("DTSTART;TZID=Europe/Berlin:").collect::<Vec<_>>()[1].split("T").collect::<Vec<_>>()[0].to_string()[4..6].to_string();
            event.end.day = i.split("DTSTART;TZID=Europe/Berlin:").collect::<Vec<_>>()[1].split("T").collect::<Vec<_>>()[0].to_string()[6..8].to_string();
            event.end.hour = i.split("DTSTART;TZID=Europe/Berlin:").collect::<Vec<_>>()[1].split("T").collect::<Vec<_>>()[1].to_string()[0..2].to_string();
            event.end.minute = i.split("DTSTART;TZID=Europe/Berlin:").collect::<Vec<_>>()[1].split("T").collect::<Vec<_>>()[1].to_string()[2..4].to_string();
            console_log(i.split("DTEND;TZID=Europe/Berlin:").collect::<Vec<_>>()[1].split("\n").collect::<Vec<_>>()[0]);
        }
        
        event.location = "TBA".to_string(); 

        if i.contains("LOCATION:") {
            event.location = i.split("LOCATION:").collect::<Vec<_>>()[1].split("\n").collect::<Vec<_>>()[0].to_string().split("|").collect::<Vec<_>>()[0].to_string();
            console_log(i.split("LOCATION:").collect::<Vec<_>>()[1].split("\n").collect::<Vec<_>>()[0]);
        }

        if i.contains("DESCRIPTION:") {
            event.description = i.split("DESCRIPTION:").collect::<Vec<_>>()[1].split("\n").collect::<Vec<_>>()[0].to_string();
            console_log(i.split("DESCRIPTION:").collect::<Vec<_>>()[1].split("\n").collect::<Vec<_>>()[0]);
        }

        if i.contains("END:VEVENT") {
            console_log("test");
        }

        if !event.title.is_empty() {
            vec.push(event);
        }
    }


    //sort after date
    vec.sort_by(|a, b| {

        if a.start.year == b.start.year {
            if a.start.month == b.start.month {
                if a.start.day == b.start.day {
                    if a.start.hour == b.start.hour {
                        if a.start.minute == b.start.minute {
                            return std::cmp::Ordering::Equal;
                        }else{
                            return a.start.minute.cmp(&b.start.minute);
                        }
                    }else{
                        return a.start.hour.cmp(&b.start.hour);
                    }
                }else{
                    return a.start.day.cmp(&b.start.day);
                }
            }else{
                return a.start.month.cmp(&b.start.month);
            }
        }else{
            return a.start.year.cmp(&b.start.year);
        }

    });

    let mut string = String::new();
    
    string = string + &vec[1].title + " && " + &vec[1].start.day + " && " + &vec[1].start.month + " && " + &vec[1].start.year + " && " + &vec[1].start.hour + " && " + &vec[1].start.minute + " && " + &vec[1].end.day + " && " + &vec[1].end.month + " && " + &vec[1].end.year + " && " + &vec[1].end.hour + " && " + &vec[1].end.minute + " && " + &vec[1].location + " && " + &vec[1].description + "\n";

    for i in 2..vec.len() {
        if vec[i].title != vec[i-1].title {
            string = string + &vec[i].title + " && " + &vec[i].start.day + " && " + &vec[i].start.month + " && " + &vec[i].start.year + " && " + &vec[i].start.hour + " && " + &vec[i].start.minute + " && " + &vec[i].end.day + " && " + &vec[i].end.month + " && " + &vec[i].end.year + " && " + &vec[i].end.hour + " && " + &vec[i].end.minute + " && " + &vec[i].location + " && " + &vec[i].description + "\n";
        }
    }

    string
}



#[component]
pub fn App() -> impl IntoView {
    let (events, set_events) = create_signal(vec![vec![String::new()]]);
    spawn_local(async move {
        
        let events = memes().await;  

        let mut tmp = vec![vec![String::new()]];

        for i in events.split("\n").collect::<Vec<_>>() {
            tmp.push(i.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>());
        } 

        set_events.set(tmp);

        for i in events.split("\n").collect::<Vec<_>>() {
            console_log(i);
        }

    });

    set_interval(move || {
        spawn_local(async move {
        
            let events = memes().await;  

            let mut tmp = vec![vec![String::new()]];

            for i in events.split("\n").collect::<Vec<_>>() {
                tmp.push(i.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>());
            } 

            set_events.set(tmp);

            for i in events.split("\n").collect::<Vec<_>>() {
                console_log(i);
            }

        });
    }, Duration::from_secs(60*30));

    view!{
       
        <div style="width:100%; height:100%">
            <ul style="list-style-type:none;padding-left:0px">
        {move || events.get().iter().map(move |x| {
                if x[0].is_empty() {
                    return view! {
                        <li class="hidden" style="width:100%">

                        </li>
                        <li>
                        </li>
                    };
                }else{
                    if x[11].len() > 17 {
                        if x[0].len() > 20 {
                            return view! {
                                <li style="width:100%; font-size:180%; color: #00cc00">
                                {x[1].clone()}.{x[2].clone()}.{x[3].clone()}
                                " " 
                                {x[4].clone()}:{x[5].clone()}<br/><br/>
                                </li>
                                <li style="width:100%; font-size:1.8vw;overflow:hidden; padding-bottom:10px"> 
                                
                                <div style="width:fit-content; overflow:hidden" class="scroll"><span>{x[0].clone()+" "}</span><span>{x[0].clone()+" "}</span><span>{x[0].clone()+" "}</span></div>
                                </li><li style="padding-bottom:30px">
                                {x[11].clone()} 
                                </li>

                            };
                        }
                        return view! {
                            <li style="width:100%; font-size:180%; color: #00cc00">
                            {x[1].clone()}.{x[2].clone()}.{x[3].clone()}
                            " " 
                            {x[4].clone()}:{x[5].clone()}<br/><br/>
                            </li>
                            <li style="width:100%; font-size:1.8vw;padding-bottom:10px"> 
                            
                            {x[0].clone()}
                            </li><li style="padding-bottom:30px">
                                siehe Kalender
                            </li>

                        };
                    }
                    if x[0].len() > 20 {
                        return view! {
                            <li style="width:100%; font-size:180%; color: #00cc00">
                            {x[1].clone()}.{x[2].clone()}.{x[3].clone()}
                            " " 
                            {x[4].clone()}:{x[5].clone()}<br/><br/>
                            </li>
                            <li style="width:100%; font-size:1.8vw;overflow:hidden; padding-bottom:10px"> 
                            
                            <div style="width:fit-content; overflow:hidden" class="scroll"><span>{x[0].clone()+" "}</span><span>{x[0].clone()+" "}</span><span>{x[0].clone()+" "}</span></div>
                            </li><li style="padding-bottom:30px">
                            {x[11].clone()} 
                            </li>

                        };
                    }
                    return view! {
                        <li style="width:100%; font-size:1.8vw; color: #00cc00">
                        {x[1].clone()}.{x[2].clone()}.{x[3].clone()}
                        " " 
                        {x[4].clone()}:{x[5].clone()}<br/><br/>
                        </li>
                        <li style="width:100%; font-size:180%;padding-bottom:10px"> 
                        
                        {x[0].clone()}
                        </li><li style="padding-bottom:30px">
                        {x[11].clone()}
                        </li>

                    };
                }

            }
        ).collect::<Vec<_>>()}
        </ul>
        </div>

    }

}



