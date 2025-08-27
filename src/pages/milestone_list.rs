use crate::models::AppState;
use crate::persist::json_state::save_app_state;
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn MilestoneList() -> Element {
    let app_state = use_context::<AppState>();
    let milestones = app_state.milestones;

    rsx! {
        div { class: "space-y-6",
            // Header
            div { class: "border-b border-gray-200 pb-6",
                h1 { class: "text-3xl font-bold text-gray-900",
                    "All Milestones"
                }
                p { class: "mt-2 text-gray-600",
                    "Manage all your milestones and anniversaries"
                }
            }

            // Milestone List
            div { class: "bg-white rounded-lg border border-gray-200",
                if milestones.read().is_empty() {
                    div { class: "text-center py-12",
                        p { class: "text-gray-500 mb-4",
                            "No milestones yet"
                        }
                        Link {
                            class: "inline-flex items-center px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-medium rounded-md transition-colors duration-150",
                            to: Route::NewMilestone {},
                            "Create Your First Milestone"
                        }
                    }
                } else {
                    div { class: "divide-y divide-gray-200",
                        for milestone in milestones.read().iter() {
                            div { class: "p-6 hover:bg-gray-50 transition-colors duration-150",
                                div { class: "flex items-start justify-between",
                                    div { class: "flex-1 min-w-0",
                                        div { class: "flex items-center space-x-3 mb-2",
                                            h3 { class: "text-lg font-medium text-gray-900 truncate",
                                                "{milestone.title}"
                                            }
                                            if milestone.is_recurring {
                                                span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-sky-100 text-sky-800",
                                                    "Recurring"
                                                }
                                            }
                                        }
                                        if !milestone.description.is_empty() {
                                            p { class: "text-gray-600 mb-2",
                                                "{milestone.description}"
                                            }
                                        }
                                        p { class: "text-sm text-gray-500",
                                            "Date: {milestone.date}"
                                        }
                                    }
                                    div { class: "flex items-center space-x-3 ml-4",
                                        Link {
                                            class: "inline-flex items-center px-3 py-1.5 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-sky-500 transition-colors duration-150",
                                            to: Route::EditMilestone { id: milestone.id },
                                            "Edit"
                                        }
                                        button {
                                            class: "inline-flex items-center px-3 py-1.5 border border-red-300 text-sm font-medium rounded-md text-red-700 bg-white hover:bg-red-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 transition-colors duration-150",
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
            }
        }
    }
}
