# CourtCode

**A Hudl Sportscode replacement for basketball video analysis, built in Rust + Tauri.**

---

## Overview

ATH-TAG is a native desktop application for basketball video analysis. It provides the core workflows of Sportscode — video import, frame-accurate tagging, playlist building, statistical analysis, and scouting reports — in a modern, open-source Rust architecture.

### Tech Stack

| Layer | Technology |
|---|---|
| Desktop shell | [Tauri 1.x](https://tauri.app) |
| Backend / logic | Rust (stable) |
| Database | SQLite via `rusqlite` (bundled) |
| Frontend UI | HTML + Vanilla JS (served by Tauri WebView) |
| Serialization | `serde` + `serde_json` |
| IDs | `uuid` v4 |
| Timestamps | `chrono` |

---

## Project Structure

```
courtcode/
├── src/                        # Rust library (shared logic)
│   ├── lib.rs
│   ├── models/
│   │   ├── video.rs            # VideoFile, ImportVideoRequest
│   │   ├── tag.rs              # Tag, CodeButton, CodeWindow, TagFilter
│   │   ├── playlist.rs         # Playlist, ExportPlaylistRequest
│   │   ├── player.rs           # Player, Team, Position
│   │   └── stats.rs            # GameStats, DataMatrix, compute_stats()
│   ├── db/
│   │   └── mod.rs              # SQLite database layer (CRUD for all models)
│   └── commands/
│       ├── video_commands.rs   # import_video, list_videos, delete_video
│       ├── tag_commands.rs     # create_tag, filter_tags, code windows
│       ├── playlist_commands.rs # create/share/export playlists
│       ├── player_commands.rs  # add/list players
│       └── stats_commands.rs   # video_stats, global_stats
│
├── src-tauri/
│   ├── tauri.conf.json         # Tauri configuration (window, permissions)
│   └── src/
│       └── main.rs             # Tauri command bindings + app entry point
│
├── ui/
│   └── index.html              # Full frontend UI (dark theme, single file)
│
├── Cargo.toml
└── README.md
```

---

## Features

### 1. Video Import & Management
- Import video files by path (MP4, MOV, etc.)
- Store game metadata: date, home/away teams, venue
- File size and duration tracking
- Tag count per video shown in library view

### 2. Tagging & Coding Engine
- **Code Windows** — configurable panels of code buttons grouped by category
- Default basketball code window with 16 buttons across 7 categories:
  - **Offense**: Pick & Roll, Post-Up, Isolation, Spot Up
  - **Defense**: Man Defense, Zone Defense, Press
  - **Transition**: Transition Offense, Transition Defense
  - **Set Play**: Set Play, BLOB, SLOB
  - **Foul**: Offensive Foul, Defensive Foul
  - **Timeout / Free Throw**
- Per-button pre-roll / post-roll (default ±3 seconds)
- Tag extended fields: quarter, court zone, shot result, possession, player IDs
- Real-time timeline visualization of tagged events

### 3. Clip & Playlist Generation
- Filter tags by: category, video, player, quarter, label text, duration
- Build playlists from any filtered tag set
- Playlist sharing via `courtcode://` deep link
- Export to JSON (data) — FFmpeg clip export available in full build

### 4. Statistical Analysis & Data Matrix
- Tags by category (bar chart)
- Tags by quarter (bar chart)
- Average clip duration
- Sportscode-style data matrix (category cross-tabulation)
- Shot result stats with field goal percentage

### 5. Scouting Reports
- Offense / Defense tendency breakdown
- Session-level summary stats
- Export to PDF / CSV / clip package (native build)

### 6. Team & Player Management
- Player profiles with position, jersey number, team
- Player involvement tracking across tags

---

## Running the Demo (CLI)

```bash
# Clone / navigate to project
cd courtcode

# Run the CLI demo (no Tauri needed)
cargo run
```

The demo:
- Creates an in-memory SQLite database
- Loads the default basketball code window
- Imports a sample game video
- Creates 10 tagged events across 4 quarters
- Filters offense tags
- Builds a playlist
- Generates a share link
- Computes and displays full statistics + data matrix

---

## Building the Full Tauri App

### Prerequisites
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js (for Tauri CLI)
npm install -g @tauri-apps/cli

# Platform dependencies
# macOS: Xcode Command Line Tools
# Linux: webkit2gtk, libgtk-3-dev, libappindicator3-dev
# Windows: WebView2 (bundled with Windows 11)
```

### Dev mode
```bash
cd courtcode
npm install
npx tauri dev
```

### Production build
```bash
npx tauri build
# Output: src-tauri/target/release/bundle/
```

---

## Architecture: Rust ↔ Frontend IPC

All Rust functions exposed to the frontend are registered as **Tauri commands** in `src-tauri/src/main.rs` via `#[tauri::command]` and `tauri::generate_handler![]`.

The frontend calls them via:
```javascript
import { invoke } from '@tauri-apps/api/tauri';

// Example: import a video
const video = await invoke('import_video', {
  req: {
    file_path: '/videos/game.mp4',
    title: 'Game 1 vs Lakers',
    home_team: 'Celtics',
    away_team: 'Lakers',
    game_date: '2024-11-15',
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
    time_in: currentTime - 3.0,
    time_out: currentTime + 3.0,
    quarter: 2,
  }
});

// Get statistics
const stats = await invoke('video_stats', { video_id: video.id });

// Filter tags
const offenseTags = await invoke('filter_tags', {
  filter: {
    categories: ['Offense'],
    video_ids: [video.id],
    quarter: 4,
  }
});
```

---

## Extending CourtCode

### Add a new code button category
In `src/models/tag.rs`, add a variant to `TagCategory` and a color in `default_color()`.

### Add video playback
In the Tauri app, use the `<video>` element with `convertFileSrc` from `@tauri-apps/api`:
```javascript
import { convertFileSrc } from '@tauri-apps/api/tauri';
videoElement.src = convertFileSrc(video.file_path);
```

### Add FFmpeg clip export
```rust
// In playlist_commands.rs
use std::process::Command;
Command::new("ffmpeg")
    .args(["-ss", &tag.time_in.to_string(), "-i", &video.file_path,
           "-t", &(tag.time_out - tag.time_in).to_string(),
           "-c", "copy", &output_path])
    .output()?;
```

### Add tracking data integration
Extend `Tag` with spatial coordinates and link to Second Spectrum or Hawk-Eye feeds via the API.

---

## Comparison with Hudl Sportscode

| Feature | Hudl Sportscode | CourtCode |
|---|---|---|
| Platform | macOS (native) | macOS / Windows / Linux (Tauri) |
| Source | Proprietary | Open source (Rust) |
| Video playback | ✅ Full | 🔧 Requires `<video>` + `convertFileSrc` |
| Code windows | ✅ | ✅ |
| Tagging | ✅ Frame-accurate | ✅ (time-accurate, frame via native) |
| Data matrix | ✅ | ✅ |
| Playlists | ✅ | ✅ |
| Sharing | ✅ Hudl platform | 🔧 Local deep-link token |
| Clip export | ✅ | 🔧 FFmpeg integration |
| Telestration | ✅ | 🔧 Canvas overlay (to implement) |
| Tracking data | ✅ Second Spectrum | 🔧 Extensible via API |
| Price | $$$$ | Free / open source |

---

## License

MIT
