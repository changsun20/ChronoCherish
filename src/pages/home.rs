use crate::models::{get_upcoming_anniversaries, AppState};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn Home() -> Element {
    let app_state = use_context::<AppState>();
    let milestones = app_state.milestones.read();

    let upcoming_anniversaries = get_upcoming_anniversaries(&milestones, 30);
    let upcoming_count = upcoming_anniversaries.len();
    let next_few = upcoming_anniversaries
        .into_iter()
        .take(3)
        .collect::<Vec<_>>();

    rsx! {
        div { class: "space-y-8",
            // Header
            div { class: "border-b border-gray-200 pb-6",
                h1 { class: "text-3xl font-bold text-gray-900",
                    {t!("dashboard_title")}
                }
                p { class: "mt-2 text-gray-600",
                    {t!("dashboard_subtitle")}
                }
            }

            // Stats Cards
            div { class: "grid grid-cols-3 gap-6",
                // Total Milestones
                div { class: "bg-white rounded-lg border border-gray-200 p-6",
                    div { class: "flex items-center",
                        div { class: "flex-1",
                            p { class: "text-sm font-medium text-gray-600",
                                {t!("stats_total_milestones")}
                            }
                            p { class: "text-2xl font-bold text-gray-900",
                                "{milestones.len()}"
                            }
                        }
                    }
                }

                // Recurring Anniversaries
                div { class: "bg-white rounded-lg border border-gray-200 p-6",
                    div { class: "flex items-center",
                        div { class: "flex-1",
                            p { class: "text-sm font-medium text-gray-600",
                                {t!("stats_recurring")}
                            }
                            p { class: "text-2xl font-bold text-sky-600",
                                "{milestones.iter().filter(|m| m.is_recurring).count()}"
                            }
                        }
                    }
                }

                // Upcoming (30 days)
                div { class: "bg-white rounded-lg border border-gray-200 p-6",
                    div { class: "flex items-center",
                        div { class: "flex-1",
                            p { class: "text-sm font-medium text-gray-600",
                                {t!("stats_upcoming")}
                            }
                            p { class: "text-2xl font-bold text-amber-600",
                                "{upcoming_count}"
                            }
                        }
                    }
                }
            }

            // Upcoming Anniversaries
            div { class: "bg-white rounded-lg border border-gray-200",
                div { class: "px-6 py-4 border-b border-gray-200",
                    h2 { class: "text-lg font-semibold text-gray-900",
                        {t!("anniversaries_title")}
                    }
                }

                div { class: "p-6",
                    if next_few.is_empty() {
                        div { class: "text-center py-12",
                            p { class: "text-gray-500",
                                {t!("no_upcoming_30_days")}
                            }
                        }
                    } else {
                        div { class: "space-y-4",
                            for anniversary in next_few {
                                div { class: "border border-gray-200 rounded-lg p-4 hover:bg-gray-50 transition-colors duration-150",
                                    div { class: "flex items-start justify-between",
                                        div { class: "flex-1",
                                            h3 { class: "font-medium text-gray-900",
                                                "{anniversary.milestone.title}"
                                            }
                                            if !anniversary.milestone.description.is_empty() {
                                                p { class: "text-sm text-gray-600 mt-1",
                                                    "{anniversary.milestone.description}"
                                                }
                                            }
                                        }
                                        div { class: "text-right ml-4",
                                            p { class: "text-sm font-medium text-sky-600",
                                                {t!("years_passed", count: anniversary.years_passed)}
                                            }
                                            p { class: "text-xs text-gray-500",
                                                "{anniversary.days_until} {t!(\"days_remaining\")}"
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
