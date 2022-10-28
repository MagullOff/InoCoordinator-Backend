use crate::schema::organizers;
use crate::types::Organizer;
use crate::errors::Errors;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Organizer, Errors> {
    organizers::table
        .find(id)
        .first::<Organizer>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn insert(organizer: Organizer, conn: &PgConnection) -> Result<Organizer, Errors> {
    diesel::insert_into(organizers::table)
        .values(organizer)
        .get_result::<Organizer>(conn)
        .map_err(|_| Errors::InsertFailed)
}
