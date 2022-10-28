use crate::schema::points;
use diesel::{self, Queryable};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[table_name = "points"]
pub struct Point {
    pub id: Uuid,
    pub event_id: Uuid,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewPoint {
    pub name: String,
}
