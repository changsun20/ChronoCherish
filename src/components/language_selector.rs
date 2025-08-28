use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t};
use unic_langid::langid;

use crate::{models::AppState, persist::save_app_state};

#[component]
pub fn LanguageSelector() -> Element {
    let mut i18n = i18n();
    let mut show_dropdown = use_signal(|| false);

    let app_state = use_context::<AppState>();
    let mut language_signal = app_state.language;

    let change_to_english = move |_| {
        i18n.set_language(langid!("en-US"));
        show_dropdown.set(false);
        language_signal.set("en-US".to_string());
        save_app_state(&app_state);
    };
    let change_to_chinese = move |_| {
        i18n.set_language(langid!("zh-CN"));
        show_dropdown.set(false);
        language_signal.set("zh-CN".to_string());
        save_app_state(&app_state);
    };

    rsx! {
        div {
            class: "relative",

            button {
                class: "flex items-center space-x-2 px-3 py-2 rounded-lg text-gray-700 hover:bg-sky-50 hover:text-sky-700 transition-colors duration-150 w-full",
                onclick: move |_| show_dropdown.set(!show_dropdown()),

                svg {
                    class: "w-4 h-4",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path { d: "M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 0 1 6.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" }
                }
                span { class: "text-sm font-medium flex-1 text-left",
                    {t!("language")}
                }
                svg {
                    class: if show_dropdown() { "w-4 h-4 transition-transform duration-150 rotate-180" } else { "w-4 h-4 transition-transform duration-150" },
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path { d: "M6 9l6 6 6-6" }
                }
            }

            // Dropdown menu
            if show_dropdown() {
                div {
                    class: "absolute left-0 bottom-full mb-2 w-full bg-white rounded-lg shadow-lg border border-gray-200 z-50 transition-all duration-150",
                    div { class: "py-2",
                        button {
                            class: "w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-sky-50 hover:text-sky-700 transition-colors duration-150",
                            onclick: change_to_english,
                            {t!("language_english")}
                        }
                        button {
                            class: "w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-sky-50 hover:text-sky-700 transition-colors duration-150",
                            onclick: change_to_chinese,
                            {t!("language_chinese")}
                        }
                    }
                }
            }
        }
    }
}
