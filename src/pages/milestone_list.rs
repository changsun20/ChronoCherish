use crate::models::AppState;
use crate::persist::json_state::save_app_state;
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn MilestoneList() -> Element {
    let app_state = use_context::<AppState>();
    let milestones = app_state.milestones;

    rsx! {
        div {
            h1 { "Milestone List" }
        }
        div {
            ul {
                for milestone in milestones.read().iter() {
                    li {
                        div { "{milestone.id} - {milestone.title} - {milestone.description} - {milestone.date}" }
                        Link {
                            to: Route::EditMilestone { id: milestone.id },
                            "Edit"
                        }
                        button {
                            onclick: {
                                let mut milestones_mut = milestones;
                                let milestone_id = milestone.id;
                                move |_| {
                                    milestones_mut.write().retain(|m| m.id != milestone_id);
                                    save_app_state(&app_state);
                                }
                            },
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}
