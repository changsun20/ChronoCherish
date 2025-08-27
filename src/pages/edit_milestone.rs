use crate::components::milestone_form::MilestoneForm;
use crate::models::{AppState, Milestone};
use crate::persist::json_state::save_app_state;
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn EditMilestone(id: u32) -> Element {
    let navigator = navigator();

    let app_state = use_context::<AppState>();
    let milestones = app_state.milestones;

    let milestone_opt = milestones.read().iter().find(|m| m.id == id).cloned();

    if let Some(milestone) = milestone_opt {
        let title_field = use_signal(|| milestone.title.clone());
        let description_field = use_signal(|| milestone.description.clone());
        let is_recurring_field = use_signal(|| milestone.is_recurring);
        let date_field = use_signal(|| Some(milestone.date.clone()));

        let handle_submit = move |_| {
            if let Some(selected_date) = date_field() {
                let new_milestone = Milestone {
                    id: milestone.id,
                    title: title_field(),
                    description: description_field(),
                    date: selected_date,
                    is_recurring: is_recurring_field(),
                };

                let mut milestones_mut = milestones;
                if let Some(milestone) = milestones_mut.write().iter_mut().find(|m| m.id == id) {
                    *milestone = new_milestone;
                }

                app_state.sort_milestones_by_date();
                save_app_state(&app_state);

                navigator.push(Route::MilestoneList {});
            }
        };

        rsx!(
            h2 { "Edit Milestone" }

            MilestoneForm {
                title: title_field,
                description: description_field,
                is_recurring: is_recurring_field,
                date: date_field,
                submit_text: "Update Milestone".to_string(),
                on_submit: handle_submit
            }
        )
    } else {
        rsx!(
            h1 { "Milestone Not Found" }
        )
    }
}
