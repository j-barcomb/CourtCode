// CourtCode - Basketball Video Analysis Platform
// Tauri Desktop Application Entry Point

use std::sync::Mutex;
use courtcode_lib::{
    db::Database,
    models::{
        video::ImportVideoRequest,
        tag::{CodeWindow, CreateTagRequest, TagFilter},
        playlist::{CreatePlaylistRequest, ExportPlaylistRequest},
    },
    commands::{
        video_commands::{self, DbState},
        tag_commands,
        playlist_commands,
        stats_commands,
    },
};

fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║        CourtCode — Basketball Video Analysis     ║");
    println!("║        Tauri Desktop App (Rust Backend)          ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    let db = Database::in_memory().expect("Failed to create DB");
    let state = Mutex::new(db);
    println!("✓ Database initialized");

    // Default code window
    let default_window = tag_commands::default_code_window(&state).expect("Failed to get default code window");
    println!("✓ Default code window: '{}' with {} buttons", default_window.name, default_window.buttons.len());
    for btn in &default_window.buttons {
        println!("    [{:>15}]  {}", format!("{}", btn.category), btn.label);
    }
    tag_commands::save_code_window(&state, default_window.clone()).expect("Failed to save code window");
    println!();

    // Import video
    let req = ImportVideoRequest {
        file_path: "/videos/game1_vs_lakers.mp4".to_string(),
        title: "Game 1 vs Lakers".to_string(),
        game_date: Some("2024-11-15".to_string()),
        home_team: Some("Celtics".to_string()),
        away_team: Some("Lakers".to_string()),
        venue: Some("TD Garden".to_string()),
    };
    let video = video_commands::import_video(&state, req).expect("Failed to import video");
    println!("✓ Video imported: '{}' (id: {})", video.title, &video.id[..8]);

    // Create tags
    use courtcode_lib::models::tag::TagCategory;
    let sample_tags: Vec<(&str, TagCategory, f64, f64, u8)> = vec![
        ("Pick & Roll",        TagCategory::Offense,    45.0,  52.0, 1),
        ("Man Defense",        TagCategory::Defense,    53.0,  58.0, 1),
        ("Isolation",          TagCategory::Offense,   120.0, 130.0, 1),
        ("Transition Offense", TagCategory::Transition,200.0, 210.0, 2),
        ("Zone Defense",       TagCategory::Defense,   300.0, 312.0, 2),
        ("Set Play",           TagCategory::SetPlay,   450.0, 465.0, 3),
        ("Offensive Foul",     TagCategory::Foul,      520.0, 525.0, 3),
        ("Pick & Roll",        TagCategory::Offense,   600.0, 608.0, 4),
        ("Free Throw",         TagCategory::FreeThrow, 610.0, 620.0, 4),
        ("Press",              TagCategory::Defense,   700.0, 715.0, 4),
    ];

    let buttons = &default_window.buttons;
    let mut created_tag_ids = vec![];
    for (label, category, t_in, t_out, quarter) in sample_tags {
        let btn = buttons.iter().find(|b| b.label == label).unwrap_or(&buttons[0]);
        let req = CreateTagRequest {
            video_id: video.id.clone(),
            code_button_id: btn.id.clone(),
            label: label.to_string(),
            category,
            time_in: t_in,
            time_out: t_out,
            quarter: Some(quarter),
            player_ids: None,
            notes: None,
            court_zone: None,
            shot_result: None,
            possession: None,
        };
        let tag = tag_commands::create_tag(&state, req).expect("Failed to create tag");
        created_tag_ids.push(tag.id.clone());
    }
    println!("✓ {} tags created (coded events)", created_tag_ids.len());

    // Show timeline
    let tags = tag_commands::tags_for_video(&state, video.id.clone()).expect("Failed to list tags");
    println!();
    println!("  Timeline for '{}':", video.title);
    for tag in &tags {
        println!("    {:>6.1}s → {:>6.1}s  [{:<15}]  {}  (Q{})",
            tag.time_in, tag.time_out, format!("{}", tag.category), tag.label, tag.quarter.unwrap_or(0));
    }

    // Filter tags
    let filter = TagFilter {
        video_ids: Some(vec![video.id.clone()]),
        categories: Some(vec![TagCategory::Offense]),
        player_ids: None, quarter: None, label_contains: None,
        min_duration: None, max_duration: None,
    };
    let offense_tags = tag_commands::filter_tags(&state, filter).expect("Filter failed");
    println!();
    println!("✓ Filter: {} offense tags found", offense_tags.len());

    // Create playlist
    let playlist = playlist_commands::create_playlist(&state, CreatePlaylistRequest {
        name: "Q4 Offense Breakdown".to_string(),
        description: Some("Key offensive possessions in Q4".to_string()),
        tag_ids: created_tag_ids[..5].to_vec(),
    }).expect("Failed to create playlist");
    println!("✓ Playlist created: '{}' ({} clips)", playlist.name, playlist.clip_count());

    let share_link = playlist_commands::share_playlist(&state, playlist.id.clone()).expect("Share failed");
    println!("✓ Share link: {}", share_link);

    // Statistics
    let stats = stats_commands::video_stats(&state, video.id.clone()).expect("Stats failed");
    println!();
    println!("📊 Statistics for '{}':", video.title);
    println!("   Total tags:        {}", stats.total_tags);
    println!("   Avg clip duration: {:.1}s", stats.average_clip_duration);
    println!();
    println!("   Tags by category:");
    let mut cat_counts: Vec<_> = stats.tags_by_category.iter().collect();
    cat_counts.sort_by(|a, b| b.1.cmp(a.1));
    for (cat, count) in &cat_counts {
        println!("     {:>18}  {:>2}  {}", cat, count, "█".repeat(*count * 4));
    }
    println!();
    println!("   Tags by quarter:");
    let mut q_counts: Vec<_> = stats.tags_by_quarter.iter().collect();
    q_counts.sort_by_key(|(k, _)| *k);
    for (q, count) in &q_counts {
        println!("     Q{}  {:>2}  {}", q, count, "█".repeat(*count * 3));
    }

    println!();
    println!("   Data matrix:");
    let matrix = &stats.matrix;
    let col_w = 12usize;
    print!("   {:>16}", "");
    for col in &matrix.col_labels { print!(" {:>col_w$}", &col[..col.len().min(col_w)]); }
    println!();
    for (i, row_label) in matrix.row_labels.iter().enumerate() {
        print!("   {:>16}", &row_label[..row_label.len().min(16)]);
        for j in 0..matrix.col_labels.len() {
            let val = matrix.cells[i][j];
            if val > 0 { print!(" {:>col_w$}", val); } else { print!(" {:>col_w$}", "·"); }
        }
        println!();
    }

    println!();
    println!("═══════════════════════════════════════════════════");
    println!("  CourtCode Rust backend verified ✓");
    println!("  In the full Tauri app, this core powers:");
    println!("    • Native desktop UI via embedded WebView");
    println!("    • Real video playback & frame-accurate coding");
    println!("    • Keyboard shortcut live tagging");
    println!("    • Drag-and-drop video import");
    println!("    • Telestration drawing over video frames");
    println!("    • Clip export via FFmpeg integration");
    println!("═══════════════════════════════════════════════════");
}
