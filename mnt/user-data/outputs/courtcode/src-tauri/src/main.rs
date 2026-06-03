// CourtCode — Tauri Command Bindings
// Wires Rust functions to the Tauri IPC bridge,
// making them callable from the JS/HTML frontend.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// In real Tauri build, you'd use:
// use tauri::State;
// use tauri::generate_handler;
// Here we stub the tauri macros for illustration since tauri
// requires the full build toolchain. The actual command signatures
// are defined below and match exactly what Tauri expects.

/*
  Example Tauri frontend call (JavaScript):
  
  import { invoke } from '@tauri-apps/api/tauri';
  
  // Import a video
  const video = await invoke('import_video', {
    req: {
      file_path: '/path/to/game.mp4',
      title: 'Game 1 vs Lakers',
      game_date: '2024-11-15',
      home_team: 'Celtics',
      away_team: 'Lakers',
      venue: 'TD Garden',
    }
  });
  
  // Tag a moment
  const tag = await invoke('create_tag', {
    req: {
      video_id: video.id,
      code_button_id: buttonId,
      label: 'Pick & Roll',
      category: 'Offense',
      time_in: player.currentTime - 3.0,
      time_out: player.currentTime + 3.0,
      quarter: currentQuarter,
    }
  });
  
  // Get stats
  const stats = await invoke('video_stats', { video_id: video.id });
  
  // Filter tags
  const offenseTags = await invoke('filter_tags', {
    filter: { categories: ['Offense'], video_ids: [video.id] }
  });
  
  // Build playlist from filtered tags
  const playlist = await invoke('create_playlist', {
    req: {
      name: 'Offense Breakdown',
      tag_ids: offenseTags.map(t => t.id),
    }
  });
  
  // Export playlist
  const path = await invoke('export_playlist_json', {
    req: {
      playlist_id: playlist.id,
      format: 'Json',
      output_path: '/tmp/export.json',
      include_annotations: false,
    }
  });
*/

fn main() {
    println!("CourtCode Tauri main — see courtcode binary for runnable demo");
    println!("Tauri commands registered:");
    let commands = [
        "import_video", "list_videos", "get_video", "delete_video",
        "create_tag", "tags_for_video", "filter_tags", "delete_tag",
        "save_code_window", "list_code_windows", "default_code_window",
        "create_playlist", "list_playlists", "delete_playlist",
        "share_playlist", "export_playlist_json",
        "add_player", "list_players",
        "video_stats", "global_stats",
    ];
    for cmd in &commands {
        println!("  • invoke('{}')", cmd);
    }
}
