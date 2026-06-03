use crate::models::stats::{GameStats, compute_stats};
use crate::commands::video_commands::DbState;

pub fn video_stats(db: &DbState, video_id: String) -> Result<GameStats, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    let tags = db.tags_for_video(&video_id).map_err(|e| e.to_string())?;
    Ok(compute_stats(&video_id, &tags))
}

pub fn global_stats(db: &DbState) -> Result<GameStats, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    let tags = db.all_tags().map_err(|e| e.to_string())?;
    Ok(compute_stats("all", &tags))
}
