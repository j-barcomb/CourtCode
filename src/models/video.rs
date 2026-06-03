use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFile {
    pub id: String,
    pub title: String,
    pub file_path: String,
    pub duration_seconds: f64,
    pub file_size_bytes: u64,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub game_date: Option<String>,
    pub home_team: Option<String>,
    pub away_team: Option<String>,
    pub venue: Option<String>,
    pub notes: Option<String>,
    pub thumbnail_path: Option<String>,
    pub imported_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

impl VideoFile {
    pub fn new(title: String, file_path: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            file_path,
            duration_seconds: 0.0,
            file_size_bytes: 0,
            width: 1920,
            height: 1080,
            fps: 30.0,
            game_date: None,
            home_team: None,
            away_team: None,
            venue: None,
            notes: None,
            thumbnail_path: None,
            imported_at: Utc::now(),
            tags: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportVideoRequest {
    pub file_path: String,
    pub title: String,
    pub game_date: Option<String>,
    pub home_team: Option<String>,
    pub away_team: Option<String>,
    pub venue: Option<String>,
}
