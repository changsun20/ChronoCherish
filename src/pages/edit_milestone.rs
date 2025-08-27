use crate::components::calendar::CalendarDatePicker;
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
        let mut title_field = use_signal(|| milestone.title.clone());
        let mut description_field = use_signal(|| milestone.description.clone());
        let mut is_recurring_field = use_signal(|| milestone.is_recurring);

        let mut date_field = use_signal(|| Some(milestone.date.clone()));
        let mut show_calendar = use_signal(|| false);

        rsx!(
            h2 { "Edit Milestone" }

            form {
                onsubmit: move |_| {
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

                        save_app_state(&app_state);

                        navigator.push(Route::MilestoneList {});
                    }
                },

                label { "Title:" }
                input {
                    value: "{title_field}",
                    oninput: move |e| title_field.set(e.value()),
                    placeholder: "Enter milestone title"
                }

                label { "Description:" }
                input {
                    value: "{description_field}",
                    oninput: move |e| description_field.set(e.value()),
                    placeholder: "Enter milestone description"
                }

                label { "Is Recurring Anniversary:" }
                input {
                    r#type: "checkbox",
                    checked: is_recurring_field(),
                    onchange: move |e| is_recurring_field.set(e.checked()),
                }

                label { "Date:" }
                p {
                    "Selected date: {date_field().map(|d| d.to_string()).unwrap_or_else(|| \"Not selected\".to_string())}"
                }
                button {
                    r#type: "button",
                    onclick: move |_| show_calendar.set(!show_calendar()),
                    if show_calendar() { "Hide Calendar" } else { "Select Date" }
                }

                if show_calendar() {
                    CalendarDatePicker {
                        selected_date: date_field,
                        on_date_change: move |date| {
                            date_field.set(date);
                            show_calendar.set(false);
                        }
                    }
                }

                input {
                    r#type: "submit",
                    value: "Update Milestone"
                }
            }
        )
    } else {
        rsx!(
            h1 { "Milestone Not Found" }
        )
    }
}
