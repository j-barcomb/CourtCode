use crate::models::player::Player;
use crate::commands::video_commands::DbState;

pub fn add_player(db: &DbState, player: Player) -> Result<Player, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.insert_player(&player).map_err(|e| e.to_string())?;
    Ok(player)
}

pub fn list_players(db: &DbState) -> Result<Vec<Player>, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.list_players().map_err(|e| e.to_string())
}
