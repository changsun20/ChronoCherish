use crate::routes::Route;
use dioxus::prelude::*;

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-50 flex",
            // Sidebar Navigation
            nav { class: "w-64 bg-white border-r border-gray-200 fixed h-full",
                div { class: "px-6 py-8",
                    // App Title
                    h1 { class: "text-2xl font-bold text-sky-700 mb-8",
                        "ChronoCherish"
                    }

                    // Navigation Menu
                    div { class: "space-y-2",
                        Link {
                            class: "flex items-center px-3 py-2 rounded-lg text-gray-700 hover:bg-sky-50 hover:text-sky-700 transition-colors duration-150",
                            to: Route::Home {},
                            "Dashboard"
                        }
                        Link {
                            class: "flex items-center px-3 py-2 rounded-lg text-gray-700 hover:bg-sky-50 hover:text-sky-700 transition-colors duration-150",
                            to: Route::NewMilestone {},
                            "New Milestone"
                        }
                        Link {
                            class: "flex items-center px-3 py-2 rounded-lg text-gray-700 hover:bg-sky-50 hover:text-sky-700 transition-colors duration-150",
                            to: Route::MilestoneList {},
                            "All Milestones"
                        }
                        Link {
                            class: "flex items-center px-3 py-2 rounded-lg text-gray-700 hover:bg-sky-50 hover:text-sky-700 transition-colors duration-150",
                            to: Route::AnniversaryList {},
                            "Anniversaries"
                        }
                    }
                }
            }

            // Main Content Area
            div { class: "flex-1 ml-64",
                div { class: "max-w-4xl mx-auto px-8 py-8",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
