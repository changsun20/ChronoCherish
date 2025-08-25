use crate::models::{AppState, Milestone};
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn EditMilestone(id: u32) -> Element {
    let navigator = navigator();
    let milestones = use_context::<AppState>().milestones;

    let milestone_opt = milestones.read().iter().find(|m| m.id == id).cloned();

    if let Some(milestone) = milestone_opt {
        let mut title_field = use_signal(|| milestone.title.clone());
        let mut description_field = use_signal(|| milestone.description.clone());
        let date_time_field = use_signal(|| milestone.date_time.clone());

        rsx!(
            form {
                onsubmit: move |_| {
                    let new_milestone = Milestone {
                        id: milestone.id,
                        title: title_field(),
                        description: description_field(),
                        date_time: date_time_field()
                    };

                    let mut milestones_mut = milestones;
                    if let Some(milestone) = milestones_mut.write().iter_mut().find(|m| m.id == id) {
                        *milestone = new_milestone;
                    }

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
        )
    } else {
        rsx!(
            div {
                h1 { "Milestone Not Found" }
            }
        )
    }
}
