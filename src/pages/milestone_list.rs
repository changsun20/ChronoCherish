use crate::models::AppState;
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn MilestoneList() -> Element {
    let milestones = use_context::<AppState>().milestones;
    let milestones_ref = milestones.read();

    rsx! {
        div {
            h1 { "Milestone List" }
        }
        div {
            ul {
                for milestone in milestones_ref.iter() {
                    li { "{milestone.id} - {milestone.title} - {milestone.description} - {milestone.date_time}" }
                    Link {
                        to: Route::EditMilestone { id: milestone.id },
                        "Edit"
                    }
                }
            }
        }
    }
}
