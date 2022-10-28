use crate::schema::players;
use diesel::{self, Queryable};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[table_name = "players"]
pub struct Player {
    pub id: Uuid,
    pub event_id: Uuid,
    pub name: String,
    pub access_code: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewPlayer {
    pub event_id: Uuid,
    pub name: String,
}
