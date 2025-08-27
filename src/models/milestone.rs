use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub date: Date,
    pub is_recurring: bool,
}
