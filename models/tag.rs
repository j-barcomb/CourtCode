use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// A code button definition in a code window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeButton {
    pub id: String,
    pub label: String,
    pub category: TagCategory,
    pub color: String,       // hex color
    pub shortcut_key: Option<String>,
    pub pre_roll_secs: f64,  // seconds before event to include in clip
    pub post_roll_secs: f64, // seconds after event to include in clip
}

impl CodeButton {
    pub fn new(label: String, category: TagCategory, color: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            label,
            category,
            color,
            shortcut_key: None,
            pre_roll_secs: 3.0,
            post_roll_secs: 3.0,
        }
    }
}

/// High-level tag categories for basketball
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TagCategory {
    Offense,
    Defense,
    Transition,
    SetPlay,
    Foul,
    Substitution,
    Timeout,
    FreeThrow,
    Custom(String),
}

impl TagCategory {
    pub fn default_color(&self) -> &str {
        match self {
            TagCategory::Offense => "#2563EB",
            TagCategory::Defense => "#DC2626",
            TagCategory::Transition => "#D97706",
            TagCategory::SetPlay => "#7C3AED",
            TagCategory::Foul => "#DB2777",
            TagCategory::Substitution => "#059669",
            TagCategory::Timeout => "#0891B2",
            TagCategory::FreeThrow => "#65A30D",
            TagCategory::Custom(_) => "#6B7280",
        }
    }
}

impl std::fmt::Display for TagCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagCategory::Offense => write!(f, "Offense"),
            TagCategory::Defense => write!(f, "Defense"),
            TagCategory::Transition => write!(f, "Transition"),
            TagCategory::SetPlay => write!(f, "Set Play"),
            TagCategory::Foul => write!(f, "Foul"),
            TagCategory::Substitution => write!(f, "Substitution"),
            TagCategory::Timeout => write!(f, "Timeout"),
            TagCategory::FreeThrow => write!(f, "Free Throw"),
            TagCategory::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// A single tagged event in a video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub video_id: String,
    pub code_button_id: String,
    pub label: String,
    pub category: TagCategory,
    pub time_in: f64,   // seconds from start
    pub time_out: f64,  // seconds from start
    pub player_ids: Vec<String>,
    pub notes: Option<String>,
    pub draw_data: Option<String>, // JSON telestration overlay
    pub created_at: DateTime<Utc>,
    // Extended basketball-specific fields
    pub quarter: Option<u8>,
    pub shot_result: Option<ShotResult>,
    pub possession: Option<Possession>,
    pub court_zone: Option<CourtZone>,
}

impl Tag {
    pub fn new(
        video_id: String,
        code_button_id: String,
        label: String,
        category: TagCategory,
        time_in: f64,
        time_out: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            video_id,
            code_button_id,
            label,
            category,
            time_in,
            time_out,
            player_ids: vec![],
            notes: None,
            draw_data: None,
            created_at: Utc::now(),
            quarter: None,
            shot_result: None,
            possession: None,
            court_zone: None,
        }
    }

    pub fn duration(&self) -> f64 {
        self.time_out - self.time_in
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShotResult {
    Made,
    Missed,
    Blocked,
    Fouled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Possession {
    Home,
    Away,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CourtZone {
    Paint,
    MidRange,
    ThreePointLeft,
    ThreePointRight,
    ThreePointTop,
    Corner3Left,
    Corner3Right,
    FreeThrowLine,
    Backcourt,
}

/// A named code window (template of code buttons)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeWindow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub buttons: Vec<CodeButton>,
    pub created_at: DateTime<Utc>,
}

impl CodeWindow {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            buttons: vec![],
            created_at: Utc::now(),
        }
    }

    /// Creates a default basketball code window
    pub fn basketball_default() -> Self {
        let mut window = Self::new("Basketball Default".to_string());
        window.description = Some("Standard basketball coding template".to_string());

        let buttons = vec![
            CodeButton::new("Pick & Roll".to_string(), TagCategory::Offense, "#2563EB".to_string()),
            CodeButton::new("Post-Up".to_string(), TagCategory::Offense, "#1D4ED8".to_string()),
            CodeButton::new("Isolation".to_string(), TagCategory::Offense, "#1E40AF".to_string()),
            CodeButton::new("Spot Up".to_string(), TagCategory::Offense, "#1E3A8A".to_string()),
            CodeButton::new("Transition Offense".to_string(), TagCategory::Transition, "#D97706".to_string()),
            CodeButton::new("Man Defense".to_string(), TagCategory::Defense, "#DC2626".to_string()),
            CodeButton::new("Zone Defense".to_string(), TagCategory::Defense, "#B91C1C".to_string()),
            CodeButton::new("Press".to_string(), TagCategory::Defense, "#991B1B".to_string()),
            CodeButton::new("Transition Defense".to_string(), TagCategory::Transition, "#B45309".to_string()),
            CodeButton::new("Set Play".to_string(), TagCategory::SetPlay, "#7C3AED".to_string()),
            CodeButton::new("BLOB".to_string(), TagCategory::SetPlay, "#6D28D9".to_string()),
            CodeButton::new("SLOB".to_string(), TagCategory::SetPlay, "#5B21B6".to_string()),
            CodeButton::new("Offensive Foul".to_string(), TagCategory::Foul, "#DB2777".to_string()),
            CodeButton::new("Defensive Foul".to_string(), TagCategory::Foul, "#BE185D".to_string()),
            CodeButton::new("Timeout".to_string(), TagCategory::Timeout, "#0891B2".to_string()),
            CodeButton::new("Free Throw".to_string(), TagCategory::FreeThrow, "#65A30D".to_string()),
        ];

        window.buttons = buttons;
        window
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub video_id: String,
    pub code_button_id: String,
    pub label: String,
    pub category: TagCategory,
    pub time_in: f64,
    pub time_out: f64,
    pub quarter: Option<u8>,
    pub player_ids: Option<Vec<String>>,
    pub notes: Option<String>,
    pub court_zone: Option<CourtZone>,
    pub shot_result: Option<ShotResult>,
    pub possession: Option<Possession>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagFilter {
    pub video_ids: Option<Vec<String>>,
    pub categories: Option<Vec<TagCategory>>,
    pub player_ids: Option<Vec<String>>,
    pub quarter: Option<u8>,
    pub label_contains: Option<String>,
    pub min_duration: Option<f64>,
    pub max_duration: Option<f64>,
}
