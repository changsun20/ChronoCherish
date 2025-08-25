use chrono::{DateTime, Utc};
use dioxus::{html::mi, prelude::*};
use serde::{de, Deserialize, Serialize};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/new-milestone")]
    NewMilestone {},

    #[route("/milestone-list")]
    MilestoneList {},

    #[route("/milestone/:id")]
    EditMilestone { id: u32 },
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Milestone {
    id: u32,
    title: String,
    description: String,
    date_time: DateTime<Utc>,
}

#[derive(Clone, Copy)]
struct AppState {
    milestones: Signal<Vec<Milestone>>,
    next_id: Signal<u32>,
}

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

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            Link {
                to: Route::Home {},
                "Home"
            }
        }

        div {
            Link {
                to: Route::NewMilestone {},
                "New Milestone"
            }
        }

        div {
            Link {
                to: Route::MilestoneList {},
                "Milestone List"
            }
        }

        Outlet::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        CountDown {}
    }
}

#[component]
pub fn CountDown() -> Element {
    rsx! {
        div {
            p { "Countdown Timer" }
        }
    }
}

#[component]
fn NewMilestone() -> Element {
    let navigator = navigator();

    let mut next_id = use_context::<AppState>().next_id;
    let mut milestones = use_context::<AppState>().milestones;

    let mut title_field = use_signal(|| "".to_string());
    let mut description_field = use_signal(|| "".to_string());
    let mut date_time_field = use_signal(|| Utc::now());

    rsx! {
        form {
            onsubmit: move |_| {
                date_time_field.set(Utc::now());
                let new_milestone = Milestone {
                    id: next_id(),
                    title: title_field(),
                    description: description_field(),
                    date_time: date_time_field()
                };
                next_id += 1;
                milestones.write().push(new_milestone);

                title_field.set("".into());
                description_field.set("".into());
                date_time_field.set(Utc::now());

                navigator.push(Route::MilestoneList {});
            },
            input {
                value: "{title_field}",
                oninput: move |e| title_field.set(e.value()),
            }
            input {
                value: "{description_field}",
                oninput: move |e| description_field.set(e.value()),
            }
            input { r#type: "submit" }
        }
    }
}

#[component]
fn MilestoneList() -> Element {
    let milestones = use_context::<AppState>().milestones;
    let milestones_lock = milestones.read();

    rsx! {
        div {
            h1 { "Milestone List" }
        }
        div {
            ul {
                for milestone in milestones_lock.iter() {
                    li { "{milestone.id} - {milestone.title} - {milestone.description} - {milestone.date_time}" }
                }
            }
        }
    }
}

#[component]
fn EditMilestone(id: u32) -> Element {
    rsx! {
        div {
            h1 { "Edit Milestone" }
        }
    }
}
