use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub date_time: DateTime<Utc>,
}
