// use crate::components::calendar::CalendarDatePicker;
use dioxus::prelude::*;
use time::{macros::format_description, Date};

#[component]
pub fn MilestoneForm(
    title: Signal<String>,
    description: Signal<String>,
    is_recurring: Signal<bool>,
    date: Signal<Option<Date>>,
    submit_text: String,
    on_submit: EventHandler<()>,
) -> Element {
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
            input {
                r#type: "date",
                value: date().map(|d| d.to_string()).unwrap_or_default(),
                oninput: move |e| {
                    let date_str = e.value();
                    // HTML date input uses ISO 8601 format (YYYY-MM-DD)
                    let format = format_description!("[year]-[month]-[day]");
                    if let Ok(parsed_date) = Date::parse(&date_str, &format) {
                        date.set(Some(parsed_date));
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
