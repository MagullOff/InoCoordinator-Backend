use crate::schema::events;
use crate::types::Event;
use crate::errors::Errors;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Event, Errors> {
    events::table
        .find(id)
        .first::<Event>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_by_organizer(organizer_id: Uuid, conn: &PgConnection) -> Result<Vec<Event>,Errors> {
    events::table
            .filter(events::organizer_id.eq(organizer_id))
            .load::<Event>(conn)
            .map_err(|_| Errors::QueryFailed)
}

pub fn insert(event: Event, conn: &PgConnection) -> Result<Event, Errors> {
    diesel::insert_into(events::table)
        .values(event)
        .get_result::<Event>(conn)
        .map_err(|_| Errors::InsertFailed)
}
