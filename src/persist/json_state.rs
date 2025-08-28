use crate::models::app_state::AppState;
use crate::models::app_state_data::AppStateData;
use crate::models::milestone::Milestone;
use std::path::Path;

const DATA_FILE: &str = "app_data.json";

pub fn load_app_state() -> (Vec<Milestone>, u32, String) {
    if Path::new(DATA_FILE).exists() {
        match std::fs::read_to_string(DATA_FILE) {
            Ok(content) => match serde_json::from_str::<AppStateData>(&content) {
                Ok(data) => (data.milestones, data.next_id, data.language),
                Err(e) => {
                    eprintln!("Failed to parse JSON: {}", e);
                    (Vec::new(), 1, "en-US".to_string())
                }
            },
            Err(e) => {
                eprintln!("Failed to read file: {}", e);
                (Vec::new(), 1, "en-US".to_string())
            }
        }
    } else {
        (Vec::new(), 1, "en-US".to_string())
    }
}

pub fn save_app_state(app_state: &AppState) {
    let data = app_state.to_data();

    match serde_json::to_string_pretty(&data) {
        Ok(json) => match std::fs::write(DATA_FILE, &json) {
            Ok(()) => {}
            Err(e) => eprintln!("Failed to save data: {}", e),
        },
        Err(e) => {
            eprintln!("Failed to serialize data: {}", e);
        }
    }
}
