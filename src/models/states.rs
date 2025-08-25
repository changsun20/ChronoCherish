use super::milestone::Milestone;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct AppState {
    pub milestones: Signal<Vec<Milestone>>,
    pub next_id: Signal<u32>,
}
