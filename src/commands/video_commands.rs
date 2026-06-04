use std::sync::Mutex;
use crate::db::Database;
use crate::models::video::{VideoFile, ImportVideoRequest};

pub type DbState = Mutex<Database>;

pub fn import_video(db: &DbState, req: ImportVideoRequest) -> Result<VideoFile, String> {
    let mut video = VideoFile::new(req.title, req.file_path.clone());
    video.game_date = req.game_date;
    video.home_team = req.home_team;
    video.away_team = req.away_team;
    video.venue = req.venue;
    if let Ok(metadata) = std::fs::metadata(&req.file_path) {
        video.file_size_bytes = metadata.len();
    }
    let db = db.lock().map_err(|e| e.to_string())?;
    db.insert_video(&video).map_err(|e| e.to_string())?;
    Ok(video)
}

pub fn list_videos(db: &DbState) -> Result<Vec<VideoFile>, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.list_videos().map_err(|e| e.to_string())
}

pub fn get_video(db: &DbState, id: String) -> Result<Option<VideoFile>, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.get_video(&id).map_err(|e| e.to_string())
}

pub fn delete_video(db: &DbState, id: String) -> Result<usize, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.delete_video(&id).map_err(|e| e.to_string())
}

pub fn update_video_duration(db: &DbState, id: String, duration_seconds: f64) -> Result<(), String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.update_video_duration(&id, duration_seconds).map_err(|e| e.to_string())
}
