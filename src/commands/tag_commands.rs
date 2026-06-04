use crate::models::tag::{Tag, CodeWindow, CreateTagRequest, TagFilter};
use crate::commands::video_commands::DbState;

pub fn create_tag(db: &DbState, req: CreateTagRequest) -> Result<Tag, String> {
    let mut tag = Tag::new(req.video_id, req.code_button_id, req.label, req.category, req.time_in, req.time_out);
    tag.quarter    = req.quarter;
    tag.player_ids = req.player_ids.unwrap_or_default();
    tag.notes      = req.notes;
    tag.court_zone = req.court_zone;
    tag.shot_result = req.shot_result;
    tag.possession = req.possession;
    let db = db.lock().map_err(|e| e.to_string())?;
    db.insert_tag(&tag).map_err(|e| e.to_string())?;
    Ok(tag)
}

pub fn tags_for_video(db: &DbState, video_id: String) -> Result<Vec<Tag>, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.tags_for_video(&video_id).map_err(|e| e.to_string())
}

pub fn filter_tags(db: &DbState, filter: TagFilter) -> Result<Vec<Tag>, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    let all_tags = db.all_tags().map_err(|e| e.to_string())?;
    let filtered = all_tags.into_iter().filter(|tag| {
        if let Some(ref vids) = filter.video_ids {
            if !vids.contains(&tag.video_id) { return false; }
        }
        if let Some(ref cats) = filter.categories {
            if !cats.contains(&tag.category) { return false; }
        }
        if let Some(ref pids) = filter.player_ids {
            if !pids.iter().any(|p| tag.player_ids.contains(p)) { return false; }
        }
        if let Some(q) = filter.quarter {
            if tag.quarter != Some(q) { return false; }
        }
        if let Some(ref lbl) = filter.label_contains {
            if !tag.label.to_lowercase().contains(&lbl.to_lowercase()) { return false; }
        }
        let dur = tag.duration();
        if let Some(min) = filter.min_duration { if dur < min { return false; } }
        if let Some(max) = filter.max_duration { if dur > max { return false; } }
        true
    }).collect();
    Ok(filtered)
}

pub fn delete_tag(db: &DbState, id: String) -> Result<usize, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.delete_tag(&id).map_err(|e| e.to_string())
}

pub fn update_tag_players(db: &DbState, tag_id: String, player_ids: Vec<String>) -> Result<Tag, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    let all_tags = db.all_tags().map_err(|e| e.to_string())?;
    let mut tag = all_tags.into_iter().find(|t| t.id == tag_id)
        .ok_or("Tag not found".to_string())?;
    tag.player_ids = player_ids;
    db.insert_tag(&tag).map_err(|e| e.to_string())?;
    Ok(tag)
}

pub fn save_code_window(db: &DbState, window: CodeWindow) -> Result<CodeWindow, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.insert_code_window(&window).map_err(|e| e.to_string())?;
    Ok(window)
}

pub fn list_code_windows(db: &DbState) -> Result<Vec<CodeWindow>, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.list_code_windows().map_err(|e| e.to_string())
}

pub fn default_code_window(_db: &DbState) -> Result<CodeWindow, String> {
    Ok(CodeWindow::basketball_default())
}
