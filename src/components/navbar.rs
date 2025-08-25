use crate::routes::Route;
use dioxus::prelude::*;

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
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
