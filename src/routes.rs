use crate::components::Navbar;
use crate::pages::{EditMilestone, Home, MilestoneList, NewMilestone};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/new-milestone")]
    NewMilestone {},

    #[route("/milestone-list")]
    MilestoneList {},

    #[route("/milestone/:id")]
    EditMilestone { id: u32 },
}
