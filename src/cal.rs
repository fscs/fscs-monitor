use chrono::DateTime;
use leptos::{
    component, create_signal, leptos_dom::logging::console_log, set_interval, view, IntoView,
    SignalGet, SignalSet,
};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

struct Event {
    title: String,
    start: chrono::DateTime<chrono::Utc>,
    location: String,
    description: String,
}

#[wasm_bindgen]
pub async fn memes() -> String {
    let mut vec = vec![Event {
        title: String::new(),
        start: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
            .unwrap()
            .into(),
        location: String::new(),
        description: String::new(),
    }];

    let timestamp = chrono::Utc::now().timestamp();

    let url = format!("https://nextcloud.inphima.de/remote.php/dav/public-calendars/CAx5MEp7cGrQ6cEe?start={}&export=&componentType=VEVENT", timestamp);

    let resp = reqwest::get(url).await.unwrap();
    for i in resp.text().await.unwrap().split("UID:").collect::<Vec<_>>() {
        console_log(i);
        let i = i.replace('\\', "");
        if vec.len() > 7 {
            break;
        }
        let mut event = Event {
            title: String::new(),
            location: String::new(),
            start: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
                .unwrap()
                .into(),
            description: String::new(),
        };

        if i.contains("SUMMARY:") {
            event.title = i.split("SUMMARY:").collect::<Vec<_>>()[1]
                .split('\n')
                .collect::<Vec<_>>()[0]
                .to_string();
            console_log(
                i.split("SUMMARY:").collect::<Vec<_>>()[1]
                    .split('\n')
                    .collect::<Vec<_>>()[0],
            );
        }

        if i.contains("DTSTART;TZID=Europe/Berlin:") {
            console_log("test");
            let date = i.split("DTSTART;TZID=Europe/Berlin:").collect::<Vec<_>>()[1]
                .split('\n')
                .collect::<Vec<_>>()[0]
                .to_string();
            console_log(&date);
            //parse Date to DateTime 20230101T000000
            let date = format!("{}T{}Z", &date[0..8], &date[9..15]);
            let date = format!(
                "{}{}{}{}{}{}{}",
                &date[0..4],
                "-",
                &date[4..6],
                "-",
                &date[6..8],
                "T",
                &date[9..15]
            );
            let date = format!(
                "{}{}{}{}{}{}{}",
                &date[0..11],
                &date[11..13],
                ":",
                &date[13..15],
                ":",
                "00",
                "Z"
            );

            console_log(&date);

            event.start = DateTime::parse_from_rfc3339(&date).unwrap().into();
            console_log(
                &date
                    .parse::<DateTime<chrono::Utc>>()
                    .unwrap()
                    .format("%d.%m.%Y %H:%M")
                    .to_string(),
            );
        }

        event.location = "TBA".to_string();

        if i.contains("LOCATION:") {
            event.location = i.split("LOCATION:").collect::<Vec<_>>()[1]
                .split('\n')
                .collect::<Vec<_>>()[0]
                .to_string()
                .split('|')
                .collect::<Vec<_>>()[0]
                .to_string();
            console_log(
                i.split("LOCATION:").collect::<Vec<_>>()[1]
                    .split('\n')
                    .collect::<Vec<_>>()[0],
            );
        }

        if i.contains("DESCRIPTION:") {
            event.description = i.split("DESCRIPTION:").collect::<Vec<_>>()[1]
                .split('\n')
                .collect::<Vec<_>>()[0]
                .to_string();
            console_log(
                i.split("DESCRIPTION:").collect::<Vec<_>>()[1]
                    .split('\n')
                    .collect::<Vec<_>>()[0],
            );
        }

        console_log(&event.title);

        if !event.title.is_empty() {
            vec.push(event);
        }
    }

    //sort after date

    vec.sort_by(|a, b| a.start.cmp(&b.start));

    //format Date to string

    let mut string = String::new();

    string = string
        + &vec[1].title
        + " && "
        + &vec[1].start.format("%d.%m.%Y %H:%M").to_string()
        + " && "
        + &vec[1].location
        + " && "
        + &vec[1].description
        + "\n";
    console_log("test");
    console_log(&string.clone());

    for i in 2..vec.len() {
        if vec[i].title != vec[i - 1].title {
            string = string
                + &vec[i].title
                + " && "
                + &vec[i].start.format("%d.%m.%Y %H:%M").to_string()
                + " && "
                + &vec[i].location
                + " && "
                + &vec[i].description
                + "\n";
        }
    }

    console_log("test");
    console_log(&string.clone());
    string
}

#[component]
pub fn App() -> impl IntoView {
    let (events, set_events) = create_signal(vec![vec![String::new()]]);
    spawn_local(async move {
        let events = memes().await;

        let mut tmp = vec![vec![String::new()]];

        for i in events.split('\n').collect::<Vec<_>>() {
            tmp.push(i.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>());
        }

        set_events.set(tmp);

        for i in events.split('\n').collect::<Vec<_>>() {
            console_log(i);
        }
        console_log(&events);
    });

    set_interval(
        move || {
            spawn_local(async move {
                let events = memes().await;

                let mut tmp = vec![vec![String::new()]];

                for i in events.split('\n').collect::<Vec<_>>() {
                    tmp.push(i.split(" && ").map(|x| x.to_string()).collect::<Vec<_>>());
                }

                set_events.set(tmp);

                for i in events.split('\n').collect::<Vec<_>>() {
                    console_log(i);
                }
            });
        },
        Duration::from_secs(60 * 30),
    );

    view! {

        <div style="width:100%; height:100%">
            <ul style="list-style-type:none;padding-left:0px">
            {move || events.get().iter().map(move |x| {
              if x[0].is_empty() {
                  view! {
                      <li class="hidden" style="width:100%">

                          </li>
                          <li>
                          </li>
                  }
              }else{
                  if x[2].len() > 17 {
                      if x[0].len() > 20 {
                          return view! {
                              <li style="width:100%; font-size:1.8vw; color: #00cc00; padding-bottom:0px">
                              {x[1].clone()}
                              </li>
                                  <li style="width:100%; font-size:1.8vw;overflow:hidden; padding-bottom:10px">

                                  <div style="width:fit-content; overflow:hidden" class="scroll"><span>{x[0].clone()+" "}</span><span>{x[0].clone()+" "}</span><span>{x[0].clone()+" "}</span></div>
                                  </li><li style="padding-bottom:30px; font-size:1.3vw">
                                  {x[2].clone()}
                              </li>

                          };
                      }
                      return view! {
                          <li style="width:100%; font-size:1.8vw; color: #00cc00; padding-bottom:0px">
                          {x[1].clone()}
                          </li>
                              <li style="width:100%; font-size:1.8vw;padding-bottom:10px">

                              {x[0].clone()}
                          </li><li style="padding-bottom:30px; font-size:1.3vw">
                              siehe Kalender
                              </li>

                      };
                  }
                  if x[0].len() > 20 {
                      return view! {
                          <li style="width:100%; font-size:1.8vw; color: #00cc00; padding-bottom:0px">
                          {x[1].clone()}
                          </li>
                              <li style="width:100%; font-size:1.8vw;overflow:hidden; padding-bottom:10px">

                              <div style="width:fit-content; overflow:hidden" class="scroll"><span>{x[0].clone()+" "}</span><span>{x[0].clone()+" "}</span><span>{x[0].clone()+" "}</span></div>
                              </li><li style="padding-bottom:30px; font-size:1.3vw">
                              {x[2].clone()}
                          </li>

                      };
                  }
                  view! {
                      <li style="width:100%; font-size:1.8vw; color: #00cc00; padding-bottom:0px">
                      {x[1].clone()}
                      </li>
                          <li style="width:100%; font-size:1.8vw;padding-bottom:10px">

                          {x[0].clone()}
                      </li><li style="padding-bottom:30px; font-size:1.3vw">
                      {x[2].clone()}
                      </li>

                  }
              }

          }
            ).collect::<Vec<_>>()}
        </ul>
            </div>

    }
}
