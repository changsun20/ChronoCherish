use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    Calendar, CalendarGrid, CalendarHeader, CalendarNavigation, CalendarNextMonthButton,
    CalendarPreviousMonthButton, CalendarSelectMonth, CalendarSelectYear,
};

use crate::utils::now_local_date;
use time::{macros::date, Date};

#[component]
pub fn CalendarDatePicker(
    selected_date: Signal<Option<Date>>,
    on_date_change: EventHandler<Option<Date>>,
) -> Element {
    let mut view_date = use_signal(|| now_local_date());

    rsx! {
        div {
            Calendar {
                selected_date: selected_date(),
                on_date_change: move |date| {
                    on_date_change.call(date);
                },
                view_date: view_date(),
                on_view_change: move |new_view: Date| {
                    view_date.set(new_view);
                },
                min_date: date!(1995-07-21),
                max_date: date!(2035-09-11),
                CalendarHeader {
                    CalendarNavigation {
                        CalendarPreviousMonthButton {
                            // svg {
                            //     view_box: "0 0 24 24",
                            //     xmlns: "http://www.w3.org/2000/svg",
                            //     polyline { points: "15 6 9 12 15 18" }
                            // }
                        }
                        CalendarSelectMonth {}
                        CalendarSelectYear {}
                        CalendarNextMonthButton {
                            // svg {
                            //     view_box: "0 0 24 24",
                            //     xmlns: "http://www.w3.org/2000/svg",
                            //     polyline { points: "9 18 15 12 9 6" }
                            // }
                        }
                    }
                }
                CalendarGrid {}
            }
        }
    }
}
