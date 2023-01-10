use crate::schema::captures;
use chrono::NaiveDateTime;
use diesel::{self, Queryable};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use uuid::Uuid;

#[derive(JsonSchema, Queryable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[table_name = "captures"]
pub struct Capture {
    pub id: Uuid,
    pub player_id: Uuid,
    pub point_id: Uuid,
    pub date: NaiveDateTime,
}
