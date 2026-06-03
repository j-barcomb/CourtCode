use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub tag_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_shared: bool,
    pub share_token: Option<String>,
    pub cover_tag_id: Option<String>,
}

impl Playlist {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            tag_ids: vec![],
            created_at: now,
            updated_at: now,
            is_shared: false,
            share_token: None,
            cover_tag_id: None,
        }
    }

    pub fn clip_count(&self) -> usize {
        self.tag_ids.len()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipAnnotation {
    pub id: String,
    pub playlist_id: String,
    pub tag_id: String,
    pub draw_data: Option<String>,
    pub voice_note_path: Option<String>,
    pub text_note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylistRequest {
    pub name: String,
    pub description: Option<String>,
    pub tag_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPlaylistRequest {
    pub playlist_id: String,
    pub format: ExportFormat,
    pub output_path: String,
    pub include_annotations: bool,
    pub watermark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Mp4,
    Pdf,
    Json,
    Csv,
}
