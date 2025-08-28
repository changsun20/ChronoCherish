use super::app_state_data::AppStateData;
use super::milestone::Milestone;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct AppState {
    pub milestones: Signal<Vec<Milestone>>,
    pub next_id: Signal<u32>,
    pub language: Signal<String>,
}

impl AppState {
    pub fn to_data(&self) -> AppStateData {
        AppStateData {
            milestones: self.milestones.read().clone(),
            next_id: self.next_id.read().clone(),
            language: self.language.read().clone(),
        }
    }

    pub fn from_data(data: AppStateData) -> Self {
        Self {
            milestones: use_signal(|| data.milestones),
            next_id: use_signal(|| data.next_id),
            language: use_signal(|| data.language),
        }
    }

    pub fn sort_milestones_by_date(mut self) {
        self.milestones.write().sort_by(|a, b| b.date.cmp(&a.date));
    }
}
