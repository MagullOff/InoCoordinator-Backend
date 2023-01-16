use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use uuid::Uuid;

#[derive(JsonSchema, Debug, Deserialize, Serialize, Clone)]
pub struct EventStats {
    pub id: Uuid,
    pub name: String,
    pub completion_amount: i32,
    pub average_completion_amount: i32,
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
