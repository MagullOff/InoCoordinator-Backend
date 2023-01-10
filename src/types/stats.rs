use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(JsonSchema, Debug, Deserialize, Serialize, Clone)]
pub struct EventStats {
    pub name: String,
    pub player_amount: i32,
    pub point_stats: Vec<PointStat>,
}

#[derive(JsonSchema, Debug, Deserialize, Serialize, Clone)]
pub struct PointStat {
    pub name: String,
    pub pass_list: Vec<String>,
}

#[derive(JsonSchema, Debug, Deserialize, Serialize, Clone)]
pub struct PlayerStats {
    pub name: String,
    pub capture_percentage: i32,
    pub point_stats: Vec<PlayerPointStats>,
}

#[derive(JsonSchema, Debug, Deserialize, Serialize, Clone)]
pub struct PlayerPointStats {
    pub name: String,
    pub capture_percentage: i32,
    pub date: Option<NaiveDateTime>,
}
