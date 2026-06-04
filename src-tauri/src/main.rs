#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::{Manager, State};

// Re-use all the logic from the library
use courtcode_lib::{
    db::Database,
    models::{
        video::{VideoFile, ImportVideoRequest},
        tag::{Tag, CodeWindow, CreateTagRequest, TagFilter},
        playlist::{Playlist, CreatePlaylistRequest, ExportPlaylistRequest},
        player::Player,
        stats::GameStats,
    },
    commands::{
        video_commands::{self, DbState},
        tag_commands,
        playlist_commands,
        player_commands,
        stats_commands,
    },
};

#[tauri::command]
fn import_video(state: State<DbState>, req: ImportVideoRequest) -> Result<VideoFile, String> {
    video_commands::import_video(&state, req)
}
#[tauri::command]
fn list_videos(state: State<DbState>) -> Result<Vec<VideoFile>, String> {
    video_commands::list_videos(&state)
}
#[tauri::command]
fn delete_video(state: State<DbState>, id: String) -> Result<usize, String> {
    video_commands::delete_video(&state, id)
}
#[tauri::command]
fn update_video_duration(state: State<DbState>, id: String, duration_seconds: f64) -> Result<(), String> {
    video_commands::update_video_duration(&state, id, duration_seconds)
}
#[tauri::command]
fn create_tag(state: State<DbState>, req: CreateTagRequest) -> Result<Tag, String> {
    tag_commands::create_tag(&state, req)
}
#[tauri::command]
fn tags_for_video(state: State<DbState>, video_id: String) -> Result<Vec<Tag>, String> {
    tag_commands::tags_for_video(&state, video_id)
}
#[tauri::command]
fn filter_tags(state: State<DbState>, filter: TagFilter) -> Result<Vec<Tag>, String> {
    tag_commands::filter_tags(&state, filter)
}
#[tauri::command]
fn delete_tag(state: State<DbState>, id: String) -> Result<usize, String> {
    tag_commands::delete_tag(&state, id)
}
#[tauri::command]
fn default_code_window(state: State<DbState>) -> Result<CodeWindow, String> {
    tag_commands::default_code_window(&state)
}
#[tauri::command]
fn save_code_window(state: State<DbState>, window: CodeWindow) -> Result<CodeWindow, String> {
    tag_commands::save_code_window(&state, window)
}
#[tauri::command]
fn list_code_windows(state: State<DbState>) -> Result<Vec<CodeWindow>, String> {
    tag_commands::list_code_windows(&state)
}
#[tauri::command]
fn create_playlist(state: State<DbState>, req: CreatePlaylistRequest) -> Result<Playlist, String> {
    playlist_commands::create_playlist(&state, req)
}
#[tauri::command]
fn list_playlists(state: State<DbState>) -> Result<Vec<Playlist>, String> {
    playlist_commands::list_playlists(&state)
}
#[tauri::command]
fn delete_playlist(state: State<DbState>, id: String) -> Result<usize, String> {
    playlist_commands::delete_playlist(&state, id)
}
#[tauri::command]
fn share_playlist(state: State<DbState>, id: String) -> Result<String, String> {
    playlist_commands::share_playlist(&state, id)
}
#[tauri::command]
fn export_playlist_json(state: State<DbState>, req: ExportPlaylistRequest) -> Result<String, String> {
    playlist_commands::export_playlist_json(&state, req)
}
#[tauri::command]
fn add_tag_to_playlist(state: State<DbState>, playlist_id: String, tag_id: String) -> Result<Playlist, String> {
    playlist_commands::add_tag_to_playlist(&state, playlist_id, tag_id)
}
#[tauri::command]
fn remove_tag_from_playlist(state: State<DbState>, playlist_id: String, tag_id: String) -> Result<Playlist, String> {
    playlist_commands::remove_tag_from_playlist(&state, playlist_id, tag_id)
}
#[tauri::command]
fn update_playlist_name(state: State<DbState>, playlist_id: String, name: String) -> Result<Playlist, String> {
    playlist_commands::update_playlist_name(&state, playlist_id, name)
}
#[tauri::command]
fn add_player(state: State<DbState>, player: Player) -> Result<Player, String> {
    player_commands::add_player(&state, player)
}
#[tauri::command]
fn list_players(state: State<DbState>) -> Result<Vec<Player>, String> {
    player_commands::list_players(&state)
}
#[tauri::command]
fn video_stats(state: State<DbState>, video_id: String) -> Result<GameStats, String> {
    stats_commands::video_stats(&state, video_id)
}
#[tauri::command]
fn global_stats(state: State<DbState>) -> Result<GameStats, String> {
    stats_commands::global_stats(&state)
}

fn main() {
    let db_path = dirs::data_dir()
        .map(|p| p.join("courtcode").join("courtcode.db"))
        .unwrap_or_else(|| std::path::PathBuf::from("courtcode.db"));

    std::fs::create_dir_all(db_path.parent().unwrap()).ok();

    let db = Database::new(db_path.to_str().unwrap())
        .unwrap_or_else(|_| Database::in_memory().expect("DB failed"));

    tauri::Builder::default()
        .setup(|app| {
            // Open the asset:// protocol scope so it can serve video files from anywhere
            let scope = app.asset_protocol_scope();
            // Allow the user's home directory
            if let Some(home) = dirs::home_dir() {
                scope.allow_directory(&home, true).ok();
            }
            // On Windows, allow the root of every drive letter that exists
            #[cfg(windows)]
            for letter in b'A'..=b'Z' {
                let root = std::path::PathBuf::from(format!("{}:\\", letter as char));
                if root.exists() {
                    scope.allow_directory(&root, true).ok();
                }
            }
            // On macOS / Linux allow filesystem root
            #[cfg(not(windows))]
            scope.allow_directory("/", true).ok();
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(db))
        .invoke_handler(tauri::generate_handler![
            import_video, list_videos, delete_video, update_video_duration,
            create_tag, tags_for_video, filter_tags, delete_tag,
            default_code_window, save_code_window, list_code_windows,
            create_playlist, list_playlists, delete_playlist,
            share_playlist, export_playlist_json,
            add_tag_to_playlist, remove_tag_from_playlist, update_playlist_name,
            add_player, list_players,
            video_stats, global_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running CourtCode");
}