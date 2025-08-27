#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use models::AppState;
use routes::Route;
use unic_langid::langid;

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
    use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("../assets/i18n/en-US.ftl")))
            .with_locale((langid!("zh-CN"), include_str!("../assets/i18n/zh-CN.ftl")))
    });

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
