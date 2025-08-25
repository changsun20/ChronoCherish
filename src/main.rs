use dioxus::prelude::*;
use models::{AppState, Milestone};
use routes::Route;

mod components;
mod models;
mod pages;
mod routes;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let milestones = use_signal(|| Vec::<Milestone>::new());
    let next_id = use_signal(|| 1u32);

    use_context_provider(|| AppState {
        milestones,
        next_id,
    });

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
