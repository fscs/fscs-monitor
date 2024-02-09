use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <h1>Legend</h1>
        <p style="font-size:22px">line | destination | departure time</p>
        <p style="font-size:22px; color:yellow">(Delay) arrival time</p>
        <p style="font-size:22px; color:fuchsia">Semester Progress</p>

    }
}
