use crate::models::{AppState, Milestone};
use crate::persist::save_app_state;
use crate::routes::Route;
use chrono::Utc;
use dioxus::prelude::*;

#[component]
pub fn NewMilestone() -> Element {
    let navigator = navigator();

    let app_state = use_context::<AppState>();
    let mut milestones = app_state.milestones;
    let mut next_id = app_state.next_id;

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

                next_id.set(next_id() + 1);
                milestones.write().push(new_milestone);

                save_app_state(&app_state);

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
