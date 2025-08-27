// use crate::components::calendar::CalendarDatePicker;
use dioxus::prelude::*;
use dioxus_i18n::t;
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
        div { class: "bg-white rounded-lg border border-gray-200 p-6",
            form {
                class: "space-y-6",
                onsubmit: move |_| {
                    on_submit.call(());
                },

                // Title Field
                div { class: "space-y-2",
                    label {
                        class: "block text-sm font-medium text-gray-700",
                        {t!("field_title")}
                    }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-sky-500 focus:border-sky-500",
                        value: "{title}",
                        oninput: move |e| title.set(e.value()),
                        placeholder: "{t!(\"field_title_placeholder\")}"
                    }
                }

                // Description Field
                div { class: "space-y-2",
                    label {
                        class: "block text-sm font-medium text-gray-700",
                        {t!("field_description")}
                    }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-sky-500 focus:border-sky-500",
                        value: "{description}",
                        oninput: move |e| description.set(e.value()),
                        placeholder: "{t!(\"field_description_placeholder\")}"
                    }
                }

                // Date Field
                div { class: "space-y-2",
                    label {
                        class: "block text-sm font-medium text-gray-700",
                        {t!("field_date")}
                    }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-sky-500 focus:border-sky-500",
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
                }

                // Recurring Checkbox
                div { class: "flex items-center space-x-3",
                    input {
                        class: "h-4 w-4 text-sky-600 focus:ring-sky-500 border-gray-300 rounded",
                        r#type: "checkbox",
                        checked: is_recurring(),
                        onchange: move |e| is_recurring.set(e.checked()),
                    }
                    label {
                        class: "text-sm font-medium text-gray-700",
                        {t!("field_recurring")}
                    }
                }

                // Submit Button
                div { class: "pt-4",
                    input {
                        class: "w-full bg-sky-600 hover:bg-sky-700 text-white font-medium py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-sky-500 focus:ring-offset-2 transition-colors duration-150 cursor-pointer",
                        r#type: "submit",
                        value: "{submit_text}"
                    }
                }
            }
        }
    }
}
