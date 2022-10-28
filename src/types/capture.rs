use crate::schema::captures;
use diesel::{self, Queryable};
use chrono::NaiveDate;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[table_name = "captures"]
pub struct Capture {
    pub id: Uuid,
    pub player_id: Uuid,
    pub point_id: Uuid,
    pub date: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewCapture {
    pub player_id: Uuid,
    pub point_id: Uuid,
    pub date: NaiveDate,
}
