use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub jersey_number: Option<u8>,
    pub position: Option<Position>,
    pub team: Option<String>,
    pub notes: Option<String>,
    pub photo_path: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            jersey_number: None,
            position: None,
            team: None,
            notes: None,
            photo_path: None,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Position {
    PointGuard,
    ShootingGuard,
    SmallForward,
    PowerForward,
    Center,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::PointGuard    => write!(f, "PG"),
            Position::ShootingGuard => write!(f, "SG"),
            Position::SmallForward  => write!(f, "SF"),
            Position::PowerForward  => write!(f, "PF"),
            Position::Center        => write!(f, "C"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub players: Vec<Player>,
    pub logo_path: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Team {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            players: vec![],
            logo_path: None,
            created_at: Utc::now(),
        }
    }
}
