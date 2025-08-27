use crate::components::Navbar;
use crate::pages::{AnniversaryList, EditMilestone, Home, MilestoneList, NewMilestone};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/new_milestone")]
    NewMilestone {},

    #[route("/milestone_list")]
    MilestoneList {},

    #[route("/milestone/:id")]
    EditMilestone { id: u32 },

    #[route("/anniversary_list")]
    AnniversaryList {},
}
