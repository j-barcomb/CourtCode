use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::tag::Tag;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStats {
    pub video_id: String,
    pub total_tags: usize,
    pub tags_by_category: HashMap<String, usize>,
    pub tags_by_quarter: HashMap<u8, usize>,
    pub average_clip_duration: f64,
    pub possession_breakdown: PossessionBreakdown,
    pub shot_results: ShotResultStats,
    pub top_players_by_involvement: Vec<(String, usize)>,
    pub court_zone_distribution: HashMap<String, usize>,
    pub matrix: DataMatrix,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PossessionBreakdown {
    pub home_possessions: usize,
    pub away_possessions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShotResultStats {
    pub made: usize,
    pub missed: usize,
    pub blocked: usize,
    pub fouled: usize,
}

impl ShotResultStats {
    pub fn total(&self) -> usize {
        self.made + self.missed + self.blocked + self.fouled
    }

    pub fn field_goal_percentage(&self) -> f64 {
        let attempts = self.made + self.missed + self.blocked;
        if attempts == 0 { 0.0 } else { self.made as f64 / attempts as f64 * 100.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMatrix {
    pub row_labels: Vec<String>,
    pub col_labels: Vec<String>,
    pub cells: Vec<Vec<usize>>,
}

impl DataMatrix {
    pub fn from_tags(tags: &[Tag]) -> Self {
        let categories: Vec<String> = vec![
            "Offense".to_string(), "Defense".to_string(), "Transition".to_string(),
            "Set Play".to_string(), "Foul".to_string(), "Free Throw".to_string(),
        ];
        let n = categories.len();
        let mut cells = vec![vec![0usize; n]; n];
        for tag in tags {
            let cat_label = tag.category.to_string();
            if let Some(i) = categories.iter().position(|c| c == &cat_label) {
                cells[i][i] += 1;
            }
        }
        DataMatrix { row_labels: categories.clone(), col_labels: categories, cells }
    }
}

pub fn compute_stats(video_id: &str, tags: &[Tag]) -> GameStats {
    let total_tags = tags.len();
    let mut tags_by_category: HashMap<String, usize> = HashMap::new();
    let mut tags_by_quarter: HashMap<u8, usize> = HashMap::new();
    let mut player_involvement: HashMap<String, usize> = HashMap::new();
    let mut court_zone_distribution: HashMap<String, usize> = HashMap::new();
    let mut possession_breakdown = PossessionBreakdown::default();
    let mut shot_results = ShotResultStats::default();
    let mut total_duration = 0.0f64;

    for tag in tags {
        *tags_by_category.entry(tag.category.to_string()).or_insert(0) += 1;
        if let Some(q) = tag.quarter {
            *tags_by_quarter.entry(q).or_insert(0) += 1;
        }
        total_duration += tag.duration();
        for pid in &tag.player_ids {
            *player_involvement.entry(pid.clone()).or_insert(0) += 1;
        }
        if let Some(zone) = &tag.court_zone {
            *court_zone_distribution.entry(format!("{:?}", zone)).or_insert(0) += 1;
        }
        if let Some(poss) = &tag.possession {
            match poss {
                crate::models::tag::Possession::Home => possession_breakdown.home_possessions += 1,
                crate::models::tag::Possession::Away => possession_breakdown.away_possessions += 1,
            }
        }
        if let Some(result) = &tag.shot_result {
            match result {
                crate::models::tag::ShotResult::Made    => shot_results.made += 1,
                crate::models::tag::ShotResult::Missed  => shot_results.missed += 1,
                crate::models::tag::ShotResult::Blocked => shot_results.blocked += 1,
                crate::models::tag::ShotResult::Fouled  => shot_results.fouled += 1,
            }
        }
    }

    let average_clip_duration = if total_tags > 0 { total_duration / total_tags as f64 } else { 0.0 };
    let mut top_players: Vec<(String, usize)> = player_involvement.into_iter().collect();
    top_players.sort_by(|a, b| b.1.cmp(&a.1));
    top_players.truncate(10);

    GameStats {
        video_id: video_id.to_string(),
        total_tags,
        tags_by_category,
        tags_by_quarter,
        average_clip_duration,
        possession_breakdown,
        shot_results,
        top_players_by_involvement: top_players,
        court_zone_distribution,
        matrix: DataMatrix::from_tags(tags),
    }
}
