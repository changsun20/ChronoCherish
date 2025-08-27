use crate::models::anniversary::get_upcoming_anniversaries;
use crate::models::AppState;
use dioxus::prelude::*;

#[component]
pub fn AnniversaryList() -> Element {
    let app_state = use_context::<AppState>();
    let milestones = app_state.milestones.read();

    let upcoming_anniversaries = get_upcoming_anniversaries(&milestones, 365);

    rsx! {
        h1 { "Upcoming Anniversaries" }

        if upcoming_anniversaries.is_empty() {
            p { "No upcoming anniversaries found." }
        } else {
            for anniversary in upcoming_anniversaries {
                div {
                    h3 { "{anniversary.milestone.title}" }
                    p { "{anniversary.milestone.description}" }
                    p { "Original Date: {anniversary.milestone.date}" }
                    p { "{anniversary.years_passed} years - {anniversary.days_until} days remaining" }
                    p { "Next Anniversary: {anniversary.next_anniversary}" }
                }
            }
        }
    }
}
