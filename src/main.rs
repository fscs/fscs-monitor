use leptos::*;
use std::time::Duration;

mod calendar;
mod legend;
mod mensa;
mod trains;

fn main() {
    leptos::mount_to_body(move || {
        view! {
            <div style="height:100vh; width:85vw">
                // Main pane
                <div style="height:5vh; width:100%">
                    <Notification_Bar/>
                </div>
                <div style="height:75vh; width:100%;">
                    <trains::App/>
                </div>
                <div style="height:calc(20vh - 20px); width:100%;">
                    <mensa::MensaView/>
                </div>
                <div style="height:20px; width:100%;">
                    <calendar::progress::App/>
                </div>
            </div>
            // Right pane
            <div style="height:100vh; width: calc(15vw - 3px); border-left:2px solid">
                <div style="height:80vh; width:100%">
                    <calendar::App/>
                </div>
                <div style="width:100%;background-color:dark-grey; border-top:2px solid">
                    <legend::App />
                </div>
            </div>
        }
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
        <div class="center" style="height:100%;">
            <h1>{ move || localtime}</h1>
        </div>
    }
}
