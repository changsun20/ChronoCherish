use crate::components::milestone_form::MilestoneForm;
use crate::models::{AppState, Milestone};
use crate::persist::save_app_state;
use crate::routes::Route;
use crate::utils::now_local_date;
use dioxus::prelude::*;

#[component]
pub fn NewMilestone() -> Element {
    let navigator = navigator();

    let app_state = use_context::<AppState>();
    let mut milestones = app_state.milestones;
    let mut next_id = app_state.next_id;

    let title_field = use_signal(|| "".to_string());
    let description_field = use_signal(|| "".to_string());
    let is_recurring_field = use_signal(|| false);
    let date_field = use_signal(|| Some(now_local_date()));

    let handle_submit = move |_| {
        if let Some(selected_date) = date_field() {
            let new_milestone = Milestone {
                id: next_id(),
                title: title_field(),
                description: description_field(),
                date: selected_date,
                is_recurring: is_recurring_field(),
            };

            next_id.set(next_id() + 1);
            milestones.write().push(new_milestone);

            app_state.sort_milestones_by_date();
            save_app_state(&app_state);

            navigator.push(Route::MilestoneList {});
        }
    };

    rsx! {
        div { class: "space-y-6",
            // Header
            div { class: "border-b border-gray-200 pb-6",
                h1 { class: "text-3xl font-bold text-gray-900",
                    "Create New Milestone"
                }
                p { class: "mt-2 text-gray-600",
                    "Add a new milestone to track important dates and anniversaries"
                }
            }

            MilestoneForm {
                title: title_field,
                description: description_field,
                is_recurring: is_recurring_field,
                date: date_field,
                submit_text: "Create Milestone".to_string(),
                on_submit: handle_submit
            }
        }
    }
}
