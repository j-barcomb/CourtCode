use crate::models::playlist::{Playlist, CreatePlaylistRequest, ExportPlaylistRequest};
use crate::commands::video_commands::DbState;
use uuid::Uuid;

pub fn create_playlist(db: &DbState, req: CreatePlaylistRequest) -> Result<Playlist, String> {
    let mut playlist = Playlist::new(req.name);
    playlist.description = req.description;
    playlist.tag_ids     = req.tag_ids;
    let db = db.lock().map_err(|e| e.to_string())?;
    db.insert_playlist(&playlist).map_err(|e| e.to_string())?;
    Ok(playlist)
}

pub fn list_playlists(db: &DbState) -> Result<Vec<Playlist>, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.list_playlists().map_err(|e| e.to_string())
}

pub fn delete_playlist(db: &DbState, id: String) -> Result<usize, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.delete_playlist(&id).map_err(|e| e.to_string())
}

pub fn share_playlist(_db: &DbState, id: String) -> Result<String, String> {
    let token = Uuid::new_v4().to_string();
    Ok(format!("courtcode://share/{}/{}", id, token))
}

pub fn export_playlist_json(db: &DbState, req: ExportPlaylistRequest) -> Result<String, String> {
    let db_guard = db.lock().map_err(|e| e.to_string())?;
    let playlists = db_guard.list_playlists().map_err(|e| e.to_string())?;
    let playlist  = playlists.into_iter().find(|p| p.id == req.playlist_id)
        .ok_or("Playlist not found".to_string())?;
    let tags = db_guard.all_tags().map_err(|e| e.to_string())?;
    let playlist_tags: Vec<_> = tags.into_iter().filter(|t| playlist.tag_ids.contains(&t.id)).collect();
    let export = serde_json::json!({
        "playlist": playlist,
        "tags": playlist_tags,
        "exported_at": chrono::Utc::now().to_rfc3339(),
    });
    let json_str = serde_json::to_string_pretty(&export).map_err(|e| e.to_string())?;
    std::fs::write(&req.output_path, &json_str).map_err(|e| e.to_string())?;
    Ok(req.output_path)
}

pub fn add_tag_to_playlist(db: &DbState, playlist_id: String, tag_id: String) -> Result<Playlist, String> {
    let db_guard = db.lock().map_err(|e| e.to_string())?;
    let playlists = db_guard.list_playlists().map_err(|e| e.to_string())?;
    let mut playlist = playlists.into_iter().find(|p| p.id == playlist_id)
        .ok_or("Playlist not found".to_string())?;
    if !playlist.tag_ids.contains(&tag_id) {
        playlist.tag_ids.push(tag_id);
        db_guard.insert_playlist(&playlist).map_err(|e| e.to_string())?;
    }
    Ok(playlist)
}

pub fn remove_tag_from_playlist(db: &DbState, playlist_id: String, tag_id: String) -> Result<Playlist, String> {
    let db_guard = db.lock().map_err(|e| e.to_string())?;
    let playlists = db_guard.list_playlists().map_err(|e| e.to_string())?;
    let mut playlist = playlists.into_iter().find(|p| p.id == playlist_id)
        .ok_or("Playlist not found".to_string())?;
    playlist.tag_ids.retain(|id| id != &tag_id);
    db_guard.insert_playlist(&playlist).map_err(|e| e.to_string())?;
    Ok(playlist)
}

pub fn update_playlist_name(db: &DbState, playlist_id: String, name: String) -> Result<Playlist, String> {
    let db_guard = db.lock().map_err(|e| e.to_string())?;
    let playlists = db_guard.list_playlists().map_err(|e| e.to_string())?;
    let mut playlist = playlists.into_iter().find(|p| p.id == playlist_id)
        .ok_or("Playlist not found".to_string())?;
    playlist.name = name;
    playlist.updated_at = chrono::Utc::now();
    db_guard.insert_playlist(&playlist).map_err(|e| e.to_string())?;
    Ok(playlist)
}
