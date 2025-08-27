use crate::components::calendar::CalendarDatePicker;
use dioxus::prelude::*;
use time::Date;

#[component]
pub fn MilestoneForm(
    title: Signal<String>,
    description: Signal<String>,
    is_recurring: Signal<bool>,
    date: Signal<Option<Date>>,
    submit_text: String,
    on_submit: EventHandler<()>,
) -> Element {
    let mut show_calendar = use_signal(|| false);

    rsx! {
        form {
            onsubmit: move |_| {
                on_submit.call(());
            },

            label { "Title:" }
            input {
                value: "{title}",
                oninput: move |e| title.set(e.value()),
                placeholder: "Enter milestone title"
            }

            label { "Description:" }
            input {
                value: "{description}",
                oninput: move |e| description.set(e.value()),
                placeholder: "Enter milestone description"
            }

            label { "Is Recurring Anniversary:" }
            input {
                r#type: "checkbox",
                checked: is_recurring(),
                onchange: move |e| is_recurring.set(e.checked()),
            }

            label { "Date:" }
            p {
                "Selected date: {date().map(|d| d.to_string()).unwrap_or_else(|| \"Not selected\".to_string())}"
            }
            button {
                r#type: "button",
                onclick: move |_| show_calendar.set(!show_calendar()),
                if show_calendar() { "Hide Calendar" } else { "Select Date" }
            }

            if show_calendar() {
                CalendarDatePicker {
                    selected_date: date,
                    on_date_change: move |new_date| {
                        date.set(new_date);
                        show_calendar.set(false);
                    }
                }
            }

            input {
                r#type: "submit",
                value: "{submit_text}"
            }
        }
    }
}
