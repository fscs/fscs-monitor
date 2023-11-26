use std::time::Duration;
use leptos::*;
mod trains;
mod mensa;

fn main() {
    leptos::mount_to_body(move || view! { 
        <div style="height:5vh; width:100vw">
            <Notification_Bar/>
        </div>
        <div style="height:75vh; width:100vw;">
          <trains::App/>  
        </div>
        <div style="height:20vh; width:100vw;">
          <mensa::App2/>
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
        <div class="center" style="height:100%;">
            <h1>{ move || localtime}</h1>
        </div>
    }
}



