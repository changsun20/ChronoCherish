use dioxus::prelude::*;
use models::AppState;
use routes::Route;

use crate::models::AppStateData;

mod components;
mod models;
mod pages;
mod persist;
mod routes;
mod utils;

const TAILWIND_CSS: Asset = asset!("/assets/css/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let data = persist::load_app_state();

    let app_state_data = AppStateData {
        milestones: data.0,
        next_id: data.1,
    };

    let app_state = AppState::from_data(app_state_data);

    use_context_provider(|| app_state);

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
