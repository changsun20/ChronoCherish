use crate::models::{get_upcoming_anniversaries, AppState};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let app_state = use_context::<AppState>();
    let milestones = app_state.milestones.read();

    let upcoming_anniversaries = get_upcoming_anniversaries(&milestones, 30);
    let next_few = upcoming_anniversaries
        .into_iter()
        .take(3)
        .collect::<Vec<_>>();

    rsx! {
        h1 { "ChronoCherish" }

        h2 { "Upcoming Anniversaries" }

        if next_few.is_empty() {
            p { "No upcoming anniversaries in the next 30 days." }
        } else {
            for anniversary in next_few {
                div {
                    h3 { "{anniversary.milestone.title}" }
                    p { "{anniversary.milestone.description}" }
                    p { "{anniversary.years_passed} years - {anniversary.days_until} days remaining" }
                }
            }
        }
    }
}
