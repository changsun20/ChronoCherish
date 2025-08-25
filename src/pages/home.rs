use dioxus::prelude::*;

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            p { "Countdown Timer" }
        }
    }
}
