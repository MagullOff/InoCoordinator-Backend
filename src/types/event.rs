use crate::schema::events;
use diesel::{self, Queryable};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use uuid::Uuid;

#[derive(JsonSchema, Queryable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[table_name = "events"]
pub struct Event {
    pub id: Uuid,
    pub organizer_id: Uuid,
    pub name: String,
}

#[derive(JsonSchema, Debug, Deserialize, Serialize, Clone)]
pub struct NewEvent {
    pub name: String,
}
