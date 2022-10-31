use crate::schema::captures;
use crate::types::Capture;
use crate::errors::Errors;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Capture, Errors> {
    captures::table
        .find(id)
        .first::<Capture>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_by_player(player_id: Uuid, conn: &PgConnection) -> Result<Vec<Capture>,Errors> {
    captures::table
            .filter(captures::player_id.eq(player_id))
            .load::<Capture>(conn)
            .map_err(|_| Errors::QueryFailed)
}

pub fn get_by_point(point_id: Uuid, conn: &PgConnection) -> Result<Vec<Capture>, Errors> {
    captures::table
            .filter(captures::point_id.eq(point_id))
            .load::<Capture>(conn)
            .map_err(|_| Errors::QueryFailed)
}

pub fn insert(capture: Capture, conn: &PgConnection) -> Result<Capture, Errors> {
    diesel::insert_into(captures::table)
        .values(capture)
        .get_result::<Capture>(conn)
        .map_err(|_| Errors::InsertFailed)
}
