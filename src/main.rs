use dioxus::prelude::*;
use models::AppState;
use routes::Route;

mod components;
mod models;
mod pages;
mod persist;
mod routes;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let (milestones, next_id) = persist::load_app_state();

    let milestones_signal = use_signal(|| milestones);
    let next_id_signal = use_signal(|| next_id);

    use_context_provider(|| AppState {
        milestones: milestones_signal,
        next_id: next_id_signal,
    });

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
