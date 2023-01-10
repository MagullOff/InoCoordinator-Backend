use crate::schema::points;
use diesel::{self, Queryable};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use uuid::Uuid;

#[derive(JsonSchema, Queryable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[table_name = "points"]
pub struct Point {
    pub id: Uuid,
    pub event_id: Uuid,
    pub code: String,
    pub name: String,
}

#[derive(JsonSchema, Debug, Deserialize, Serialize, Clone)]
pub struct NewPoint {
    pub name: String,
    pub event_id: Uuid,
}
