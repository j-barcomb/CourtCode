use anyhow::Result;
use rusqlite::{Connection, params};
use crate::models::{
    video::VideoFile,
    tag::{Tag, CodeWindow},
    playlist::Playlist,
    player::Player,
};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Self { conn };
        db.create_tables()?;
        Ok(db)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.create_tables()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<()> {
        self.conn.execute_batch("
            PRAGMA journal_mode=WAL;
            PRAGMA foreign_keys=ON;

            CREATE TABLE IF NOT EXISTS videos (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                file_path TEXT NOT NULL,
                duration_seconds REAL DEFAULT 0,
                file_size_bytes INTEGER DEFAULT 0,
                width INTEGER DEFAULT 1920,
                height INTEGER DEFAULT 1080,
                fps REAL DEFAULT 30,
                game_date TEXT,
                home_team TEXT,
                away_team TEXT,
                venue TEXT,
                notes TEXT,
                thumbnail_path TEXT,
                imported_at TEXT NOT NULL,
                data_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS code_windows (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                data_json TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tags (
                id TEXT PRIMARY KEY,
                video_id TEXT NOT NULL,
                code_button_id TEXT NOT NULL,
                label TEXT NOT NULL,
                category TEXT NOT NULL,
                time_in REAL NOT NULL,
                time_out REAL NOT NULL,
                quarter INTEGER,
                notes TEXT,
                created_at TEXT NOT NULL,
                data_json TEXT NOT NULL,
                FOREIGN KEY(video_id) REFERENCES videos(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS playlists (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                is_shared INTEGER DEFAULT 0,
                share_token TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                data_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS players (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                team TEXT,
                jersey_number INTEGER,
                created_at TEXT NOT NULL,
                data_json TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_tags_video    ON tags(video_id);
            CREATE INDEX IF NOT EXISTS idx_tags_category ON tags(category);
        ")?;
        Ok(())
    }

    pub fn insert_video(&self, v: &VideoFile) -> Result<()> {
        let json = serde_json::to_string(v)?;
        self.conn.execute(
            "INSERT INTO videos (id, title, file_path, duration_seconds, file_size_bytes,
             width, height, fps, game_date, home_team, away_team, venue, notes,
             thumbnail_path, imported_at, data_json)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16)",
            params![
                v.id, v.title, v.file_path, v.duration_seconds, v.file_size_bytes as i64,
                v.width, v.height, v.fps, v.game_date, v.home_team, v.away_team,
                v.venue, v.notes, v.thumbnail_path, v.imported_at.to_rfc3339(), json
            ],
        )?;
        Ok(())
    }

    pub fn list_videos(&self) -> Result<Vec<VideoFile>> {
        let mut stmt = self.conn.prepare("SELECT data_json FROM videos ORDER BY imported_at DESC")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut videos = vec![];
        for r in rows {
            if let Ok(v) = serde_json::from_str::<VideoFile>(&r?) { videos.push(v); }
        }
        Ok(videos)
    }

    pub fn get_video(&self, id: &str) -> Result<Option<VideoFile>> {
        let mut stmt = self.conn.prepare("SELECT data_json FROM videos WHERE id=?1")?;
        let mut rows = stmt.query_map([id], |row| row.get::<_, String>(0))?;
        if let Some(r) = rows.next() {
            Ok(serde_json::from_str(&r?).ok())
        } else {
            Ok(None)
        }
    }

    pub fn delete_video(&self, id: &str) -> Result<usize> {
        Ok(self.conn.execute("DELETE FROM videos WHERE id=?1", [id])?)
    }

    pub fn update_video_duration(&self, id: &str, duration_seconds: f64) -> Result<()> {
        self.conn.execute(
            "UPDATE videos SET duration_seconds=?1 WHERE id=?2",
            params![duration_seconds, id],
        )?;
        Ok(())
    }

    pub fn insert_code_window(&self, cw: &CodeWindow) -> Result<()> {
        let json = serde_json::to_string(cw)?;
        self.conn.execute(
            "INSERT OR REPLACE INTO code_windows (id, name, description, data_json, created_at) VALUES (?1,?2,?3,?4,?5)",
            params![cw.id, cw.name, cw.description, json, cw.created_at.to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn list_code_windows(&self) -> Result<Vec<CodeWindow>> {
        let mut stmt = self.conn.prepare("SELECT data_json FROM code_windows ORDER BY created_at")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut windows = vec![];
        for r in rows {
            if let Ok(cw) = serde_json::from_str::<CodeWindow>(&r?) { windows.push(cw); }
        }
        Ok(windows)
    }

    pub fn insert_tag(&self, t: &Tag) -> Result<()> {
        let json = serde_json::to_string(t)?;
        self.conn.execute(
            "INSERT INTO tags (id, video_id, code_button_id, label, category, time_in, time_out, quarter, notes, created_at, data_json)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
            params![
                t.id, t.video_id, t.code_button_id, t.label,
                t.category.to_string(), t.time_in, t.time_out,
                t.quarter, t.notes, t.created_at.to_rfc3339(), json
            ],
        )?;
        Ok(())
    }

    pub fn tags_for_video(&self, video_id: &str) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare("SELECT data_json FROM tags WHERE video_id=?1 ORDER BY time_in")?;
        let rows = stmt.query_map([video_id], |row| row.get::<_, String>(0))?;
        let mut tags = vec![];
        for r in rows {
            if let Ok(t) = serde_json::from_str::<Tag>(&r?) { tags.push(t); }
        }
        Ok(tags)
    }

    pub fn all_tags(&self) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare("SELECT data_json FROM tags ORDER BY time_in")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut tags = vec![];
        for r in rows {
            if let Ok(t) = serde_json::from_str::<Tag>(&r?) { tags.push(t); }
        }
        Ok(tags)
    }

    pub fn delete_tag(&self, id: &str) -> Result<usize> {
        Ok(self.conn.execute("DELETE FROM tags WHERE id=?1", [id])?)
    }

    pub fn insert_playlist(&self, p: &Playlist) -> Result<()> {
        let json = serde_json::to_string(p)?;
        self.conn.execute(
            "INSERT OR REPLACE INTO playlists (id, name, description, is_shared, share_token, created_at, updated_at, data_json)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
            params![
                p.id, p.name, p.description, p.is_shared as i32,
                p.share_token, p.created_at.to_rfc3339(), p.updated_at.to_rfc3339(), json
            ],
        )?;
        Ok(())
    }

    pub fn list_playlists(&self) -> Result<Vec<Playlist>> {
        let mut stmt = self.conn.prepare("SELECT data_json FROM playlists ORDER BY updated_at DESC")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut playlists = vec![];
        for r in rows {
            if let Ok(p) = serde_json::from_str::<Playlist>(&r?) { playlists.push(p); }
        }
        Ok(playlists)
    }

    pub fn delete_playlist(&self, id: &str) -> Result<usize> {
        Ok(self.conn.execute("DELETE FROM playlists WHERE id=?1", [id])?)
    }

    pub fn insert_player(&self, p: &Player) -> Result<()> {
        let json = serde_json::to_string(p)?;
        self.conn.execute(
            "INSERT INTO players (id, name, team, jersey_number, created_at, data_json) VALUES (?1,?2,?3,?4,?5,?6)",
            params![p.id, p.name, p.team, p.jersey_number, p.created_at.to_rfc3339(), json],
        )?;
        Ok(())
    }

    pub fn list_players(&self) -> Result<Vec<Player>> {
        let mut stmt = self.conn.prepare("SELECT data_json FROM players ORDER BY name")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut players = vec![];
        for r in rows {
            if let Ok(p) = serde_json::from_str::<Player>(&r?) { players.push(p); }
        }
        Ok(players)
    }
}
