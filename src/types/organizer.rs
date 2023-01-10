use crate::schema::organizers;
use diesel::{self, Queryable};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use uuid::Uuid;

#[derive(JsonSchema, Queryable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[table_name = "organizers"]
pub struct Organizer {
    pub id: Uuid,
    pub name: String,
    pub access_code: i32,
    pub email: String,
}

#[derive(JsonSchema, Debug, Deserialize, Serialize, Clone)]
pub struct NewOrganizer {
    pub name: String,
    pub email: String,
}

#[derive(JsonSchema, Debug, Deserialize, Serialize, Clone)]
pub struct LoginOrganizer {
    pub code: String,
}
