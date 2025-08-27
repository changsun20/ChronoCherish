use crate::models::anniversary::get_upcoming_anniversaries;
use crate::models::AppState;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn AnniversaryList() -> Element {
    let app_state = use_context::<AppState>();
    let milestones = app_state.milestones.read();

    let upcoming_anniversaries = get_upcoming_anniversaries(&milestones, 365);

    rsx! {
        div { class: "space-y-6",
            // Header
            div { class: "border-b border-gray-200 pb-6",
                h1 { class: "text-3xl font-bold text-gray-900",
                    {t!("anniversaries_title")}
                }
                p { class: "mt-2 text-gray-600",
                    {t!("anniversaries_subtitle")}
                }
            }

            // Anniversary List
            div { class: "bg-white rounded-lg border border-gray-200",
                if upcoming_anniversaries.is_empty() {
                    div { class: "text-center py-12",
                        p { class: "text-gray-500 mb-4",
                            {t!("no_upcoming_anniversaries")}
                        }
                        p { class: "text-sm text-gray-400",
                            {t!("no_upcoming_anniversaries_desc")}
                        }
                    }
                } else {
                    div { class: "divide-y divide-gray-200",
                        for anniversary in upcoming_anniversaries {
                            div { class: "p-6 hover:bg-gray-50 transition-colors duration-150",
                                div { class: "flex items-start justify-between",
                                    div { class: "flex-1 min-w-0",
                                        div { class: "flex items-center space-x-3 mb-2",
                                            h3 { class: "text-lg font-medium text-gray-900",
                                                "{anniversary.milestone.title}"
                                            }
                                            span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-amber-100 text-amber-800",
                                                {t!("years_passed", count: anniversary.years_passed)}
                                            }
                                        }
                                        if !anniversary.milestone.description.is_empty() {
                                            p { class: "text-gray-600 mb-3",
                                                "{anniversary.milestone.description}"
                                            }
                                        }
                                        div { class: "grid grid-cols-2 gap-4 text-sm",
                                            div {
                                                p { class: "text-gray-500",
                                                    {t!("original_date")}
                                                }
                                                p { class: "font-medium text-gray-900",
                                                    "{anniversary.milestone.date}"
                                                }
                                            }
                                            div {
                                                p { class: "text-gray-500",
                                                    {t!("next_anniversary")}
                                                }
                                                p { class: "font-medium text-gray-900",
                                                    "{anniversary.next_anniversary}"
                                                }
                                            }
                                        }
                                    }
                                    div { class: "text-right ml-4",
                                        div { class: "bg-sky-50 rounded-lg p-3 text-center",
                                            p { class: "text-2xl font-bold text-sky-600",
                                                "{anniversary.days_until}"
                                            }
                                            p { class: "text-xs text-sky-600 font-medium",
                                                {t!("days_remaining")}
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
}
