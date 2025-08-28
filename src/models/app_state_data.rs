use crate::models::milestone::Milestone;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppStateData {
    pub milestones: Vec<Milestone>,
    pub next_id: u32,
    pub language: String,
}
