use anyhow::Result;
use chrono::DateTime;
use icalendar::parser::Calendar;
use leptos::leptos_dom::logging::console_log;
use leptos::logging::log;
use leptos::spawn_local;
use leptos::{component, create_signal, set_interval, view, IntoView, SignalGet, SignalSet};
use std::time::Duration;
pub mod progress;

#[derive(Clone, Debug, Default)]
pub struct Event {
    title: String,
    start: chrono::DateTime<chrono::Utc>,
    location: String,
    description: String,
    frequency: String,
    next: chrono::DateTime<chrono::Utc>,
}

pub fn extract_data(cal: Calendar) -> Result<Vec<Event>> {
    let mut vec = vec![Event::default()];

    for event in cal.components {
        if event.name == "VEVENT" {
            console_log(&format!("event: {:?}", event));
            let mut event_tmp = Event::default();
            for property in event.properties {
                if property.name == "SUMMARY" {
                    console_log(&format!("summary: {:?}", property.val));
                    event_tmp.title = property.val.to_string();
                }
                if property.name == "DTSTART" {
                    console_log(&format!("start: {:?}", property.val));
                    //if only date is given, add time
                    let date = format!("{}T{}Z", property.val.to_string(), "000000");
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
                    event_tmp.start = DateTime::parse_from_rfc3339(&date).unwrap().into();
                }
                if property.name == "LOCATION" {
                    console_log(&format!("location: {:?}", property.val));
                    event_tmp.location = property.val.to_string();
                }
                if property.name == "DESCRIPTION" {
                    console_log(&format!("description: {:?}", property.val));
                    event_tmp.description = property.val.to_string();
                }
                if property.name == "RRULE" {
                    console_log(&format!("rrule: {:?}", property.val));
                    event_tmp.frequency = property.val.to_string();
                }
            }
            event_tmp = get_next_occurrence_after_today(event_tmp);
            vec.push(event_tmp);
        }
    }

    //sort after date
    vec.sort_by(|a, b| a.start.cmp(&b.start));

    Ok(vec)
}

fn get_next_occurrence_after_today(event: Event) -> Event {
    let mut event = event;
    if event.frequency.is_empty() {
        event.next = event.start;
        return event;
    }
    let mut frequency = event.frequency.split(';');
    let freq = frequency.next().unwrap();
    let freq = freq.split('=');
    let freq = freq.collect::<Vec<&str>>()[1];
    let freq = match freq {
        "DAILY" => chrono::Duration::days(1),
        "WEEKLY" => chrono::Duration::weeks(1),
        "MONTHLY" => chrono::Duration::days(30),
        "YEARLY" => chrono::Duration::days(365),
        _ => chrono::Duration::days(1),
    };
    let until = event.frequency.split(';');
    let until = until.collect::<Vec<&str>>()[2];
    let until = until.split('=');
    let until = until.collect::<Vec<&str>>()[1];
    let mut next: DateTime<chrono::prelude::Utc> = event.start;

    let date = format!("{}T{}Z", until, "000000");
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
    let until: DateTime<chrono::prelude::Utc> = DateTime::parse_from_rfc3339(&date).unwrap().into();
    while next < chrono::Utc::now() {
        next += freq;
    }
    while next > until {
        next -= freq;
    }
    event.next = next;

    event
}

pub async fn get_events(url: String) -> Result<Vec<Event>> {
    let resp = reqwest::get(url).await.unwrap();

    let ical = icalendar::parser::unfold(&resp.text().await.unwrap());
    let calendar = icalendar::parser::read_calendar(&ical);
    let calendar = calendar.unwrap();

    let mut vec = vec![];
    for event in extract_data(calendar)? {
        console_log(&format!("event: {:?}", event));
        if event.start < chrono::Utc::now() {
            continue;
        }
        vec.push(event);
    }

    for event in vec.iter_mut() {
        console_log(&event.title);
    }

    Ok(vec)
}

#[component]
pub fn App() -> impl IntoView {
    let (events, set_events) = create_signal(vec![Event::default()]);
    let (id, set_id) = create_signal(0);
    spawn_local(async move {
        let file = reqwest::get("http://localhost:8080/config.json")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_str(&file).unwrap();
        let stations = json["calendars"].as_array().unwrap();

        let current_semester = progress::get_current_semester().await.unwrap();
        let timestamp = current_semester
            .start
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp();

        let url = stations[id.get()]["url"]
            .to_string()
            .replace("START", &timestamp.to_string().clone())
            .replace("\"", "");

        let events = match get_events(url).await {
            Ok(events) => events,
            Err(_) => {
                vec![Event::default()]
            }
        };

        set_events.set(events);
    });

    set_interval(
        move || {
            spawn_local(async move {
                let file = reqwest::get("http://localhost:8080/config.json")
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                let json: serde_json::Value = serde_json::from_str(&file).unwrap();
                let stations = json["calendars"].as_array().unwrap();

                let current_semester = progress::get_current_semester().await.unwrap();
                let timestamp = current_semester
                    .start
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .timestamp();

                let url = stations[id.get()]["url"]
                    .to_string()
                    .replace("START", &timestamp.to_string().clone())
                    .replace("\"", "");

                let events = match get_events(url).await {
                    Ok(events) => events,
                    Err(_) => {
                        vec![Event::default()]
                    }
                };

                set_events.set(events);
                if id.get() == stations.len() - 1 {
                    set_id.set(0);
                } else {
                    set_id.set(id.get() + 1);
                }
            });
        },
        Duration::from_secs(10),
    );

    view! {
        <div style="width:100%; height:100%">
            <ul style="list-style-type:none;padding-left:0px">
                {move || events.get().iter().map(move |x| {
                    if x.title.is_empty() {
                        view! {
                            <li class="hidden" style="width:100%">
                            </li>
                            <li>
                            </li>
                        }
                    }else{
                        if x.location.len() > 17 {
                            return view! {
                                <li style="width:100%; font-size:1.8vw; color: #00cc00; padding-bottom:0px">
                                    {x.next.format("%d.%m.%Y  %H:%M").to_string()}
                                </li>
                                <li style="width:100%; font-size:1.8vw;padding-bottom:10px; white-space:initial">
                                    {x.title.clone()}
                                </li>
                                <li style="padding-bottom:30px; font-size:1.3vw">
                                    siehe Kalender
                                </li>
                            };
                        }
                        view! {
                            <li style="width:100%; font-size:1.8vw; color: #00cc00; padding-bottom:0px">
                                {x.next.format("%d.%m.%Y  %H:%M").to_string()}
                            </li>
                            <li style="width:100%; font-size:1.8vw;padding-bottom:10px; white-space:initial">
                                {x.title.clone()}
                            </li>
                            <li style="padding-bottom:30px; font-size:1.3vw">
                                {x.location.clone()}
                            </li>
                        }
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_event() {
        let event = r#"BEGIN:VCALENDAR
VERSION:2.0
CALSCALE:GREGORIAN
PRODID:-//SabreDAV//SabreDAV//EN
X-WR-CALNAME:Fachschaft Informatik (fscs)
X-APPLE-CALENDAR-COLOR:#C37285
REFRESH-INTERVAL;VALUE=DURATION:PT4H
X-PUBLISHED-TTL:PT4H
BEGIN:VTIMEZONE
TZID:Europe/Berlin
BEGIN:DAYLIGHT
TZOFFSETFROM:+0100
TZOFFSETTO:+0200
TZNAME:CEST
DTSTART:19700329T020000
RRULE:FREQ=YEARLY;BYMONTH=3;BYDAY=-1SU
END:DAYLIGHT
BEGIN:STANDARD
TZOFFSETFROM:+0200
TZOFFSETTO:+0100
TZNAME:CET
DTSTART:19701025T030000
RRULE:FREQ=YEARLY;BYMONTH=10;BYDAY=-1SU
END:STANDARD
END:VTIMEZONE
BEGIN:VTIMEZONE
TZID:Africa/Lagos
BEGIN:STANDARD
TZOFFSETFROM:+0100
TZOFFSETTO:+0100
TZNAME:WAT
DTSTART:19700101T000000
END:STANDARD
END:VTIMEZONE
BEGIN:VEVENT
CREATED:20230203T235537Z
DTSTAMP:20230316T092153Z
LAST-MODIFIED:20230316T092153Z
SEQUENCE:7
UID:b2730327-cb87-4573-a408-e30a372f1266
DTSTART;TZID=Europe/Berlin:20230331T160000
DTEND;TZID=Europe/Berlin:20230331T235500
STATUS:CONFIRMED
SUMMARY:Spieleabend
LOCATION:25.12.02.33
END:VEVENT
END:VCALENDAR"#;
        let ical = icalendar::parser::unfold(event);
        let calendar = icalendar::parser::read_calendar(&ical);
        let calendar = calendar.unwrap();
        let vec = extract_data(calendar).unwrap();
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[1].title, "Spieleabend");
        assert_eq!(vec[1].location, "25.12.02.33");
        assert_eq!(
            vec[1].start.naive_utc(),
            chrono::NaiveDate::from_ymd(2023, 3, 31).and_hms(16, 0, 0)
        );
        assert_eq!(vec[1].description, "");
        assert_eq!(vec[1].frequency, "");
    }
    #[test]
    fn test_multiday_event() {
        let event = r#"BEGIN:VCALENDAR
VERSION:2.0
CALSCALE:GREGORIAN
PRODID:-//SabreDAV//SabreDAV//EN
X-WR-CALNAME:Fachschaft Informatik (fscs)
X-APPLE-CALENDAR-COLOR:#C37285
REFRESH-INTERVAL;VALUE=DURATION:PT4H
X-PUBLISHED-TTL:PT4H
BEGIN:VTIMEZONE
TZID:Europe/Berlin
BEGIN:DAYLIGHT
TZOFFSETFROM:+0100
TZOFFSETTO:+0200
TZNAME:CEST
DTSTART:19700329T020000
RRULE:FREQ=YEARLY;BYMONTH=3;BYDAY=-1SU
END:DAYLIGHT
BEGIN:STANDARD
TZOFFSETFROM:+0200
TZOFFSETTO:+0100
TZNAME:CET
DTSTART:19701025T030000
RRULE:FREQ=YEARLY;BYMONTH=10;BYDAY=-1SU
END:STANDARD
END:VTIMEZONE
BEGIN:VTIMEZONE
TZID:Africa/Lagos
BEGIN:STANDARD
TZOFFSETFROM:+0100
TZOFFSETTO:+0100
TZNAME:WAT
DTSTART:19700101T000000
END:STANDARD
END:VTIMEZONE
BEGIN:VEVENT
CREATED:20240219T105326Z
DTSTAMP:20240219T105452Z
LAST-MODIFIED:20240219T105452Z
SEQUENCE:2
UID:ee955755-d77e-4715-898f-158924f1973d
DTSTART;VALUE=DATE:20240508
DTEND;VALUE=DATE:20240513
STATUS:CONFIRMED
SUMMARY:KIF
LOCATION:Kaiserslautern
END:VEVENT
END:VCALENDAR"#;
        let ical = icalendar::parser::unfold(event);
        let calendar = icalendar::parser::read_calendar(&ical);
        let calendar = calendar.unwrap();
        let vec = extract_data(calendar).unwrap();
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[1].title, "KIF");
        assert_eq!(vec[1].location, "Kaiserslautern");
        assert_eq!(
            vec[1].start.naive_utc(),
            chrono::NaiveDate::from_ymd(2024, 5, 8).and_hms(0, 0, 0)
        );
        assert_eq!(vec[1].description, "");
        assert_eq!(vec[1].frequency, "");
    }
    #[test]
    fn test_recurring_events() {
        let event = r#"BEGIN:VCALENDAR
VERSION:2.0
CALSCALE:GREGORIAN
PRODID:-//SabreDAV//SabreDAV//EN
X-WR-CALNAME:Fachschaft Informatik (fscs)
X-APPLE-CALENDAR-COLOR:#C37285
REFRESH-INTERVAL;VALUE=DURATION:PT4H
X-PUBLISHED-TTL:PT4H
BEGIN:VTIMEZONE
TZID:Europe/Berlin
BEGIN:DAYLIGHT
TZOFFSETFROM:+0100
TZOFFSETTO:+0200
TZNAME:CEST
DTSTART:19700329T020000
RRULE:FREQ=YEARLY;BYMONTH=3;BYDAY=-1SU
END:DAYLIGHT
BEGIN:STANDARD
TZOFFSETFROM:+0200
TZOFFSETTO:+0100
TZNAME:CET
DTSTART:19701025T030000
RRULE:FREQ=YEARLY;BYMONTH=10;BYDAY=-1SU
END:STANDARD
END:VTIMEZONE
BEGIN:VTIMEZONE
TZID:Africa/Lagos
BEGIN:STANDARD
TZOFFSETFROM:+0100
TZOFFSETTO:+0100
TZNAME:WAT
DTSTART:19700101T000000
END:STANDARD
END:VTIMEZONE
BEGIN:VEVENT
CREATED:20230414T071228Z
DTSTAMP:20230414T071441Z
LAST-MODIFIED:20230414T071441Z
SEQUENCE:2
UID:3f2a2807-6231-472a-a0b8-1057e0f74083
DTSTART;TZID=Europe/Berlin:20230414T143000
DTEND;TZID=Europe/Berlin:20230414T160000
STATUS:CONFIRMED
SUMMARY:Sitzung
RRULE:FREQ=WEEKLY;BYDAY=FR;UNTIL=20230715T220000Z
END:VEVENT
END:VCALENDAR"#;
        let ical = icalendar::parser::unfold(event);
        let calendar = icalendar::parser::read_calendar(&ical);
        let calendar = calendar.unwrap();
        let vec = extract_data(calendar).unwrap();
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[1].title, "Sitzung");
        assert_eq!(vec[1].location, "");
        assert_eq!(
            vec[1].next.naive_utc(),
            chrono::NaiveDate::from_ymd(2023, 7, 14).and_hms(14, 30, 0)
        );
        assert_eq!(vec[1].description, "");
        assert_eq!(
            vec[1].frequency,
            "FREQ=WEEKLY;BYDAY=FR;UNTIL=20230715T220000Z"
        );
    }
}
