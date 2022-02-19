use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub name: Option<String>,
    #[serde(rename="teamId")]
    pub team_id: Option<f32>,
    pub health: Option<f32>,
    pub location: Option<Location>,
    pub ranking: Option<f32>,
    #[serde(rename="accountId")]
    pub account_id: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Common {
    #[serde(rename="matchId")]
    match_id: Option<String>,
    #[serde(rename="mapName")]
    map_name: Option<String>,
    #[serde(rename="isGame")]
    is_game: Option<f32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item{
    #[serde(rename="itemId")]
    item_id: Option<String>,
    #[serde(rename="stackCount")]
    stack_count: Option<i16>,
    category: Option<String>,
    #[serde(rename="subCategory")]
    sub_category: Option<String>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct GameState{
    #[serde(rename="elapsedTime")]
    elapsed_time: Option<i16>,
    #[serde(rename="numAliveTeams")]
    num_alive_teams: Option<i8>,
    #[serde(rename="numJoinPlayers")]
    num_join_players: Option<i16>,
    #[serde(rename="numStartPlayers")]
    num_start_players: Option<i16>,
    #[serde(rename="numAlivePlayers")]
    num_alive_players: Option<i16>,
    #[serde(rename="safetyZonePosition")]
    safety_zone_position: Option<Location>,
    #[serde(rename="safetyZoneRadius")]
    safety_zone_radius: Option<f64>,
    #[serde(rename="poisonGasWarningPosition")]
    poison_gas_warning_position: Option<Location>,
    #[serde(rename="poisonGasWarningRadius")]
    poison_gas_warning_radius: Option<f64>,
    #[serde(rename="redZonePosition")]
    red_zone_position: Option<Location>,
    #[serde(rename="redZoneRadius")]
    red_zone_radius: Option<f64>
}