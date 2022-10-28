use crate::schema::players;
use crate::types::Player;
use crate::errors::Errors;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Player, Errors> {
    players::table
        .find(id)
        .first::<Player>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_by_event(event_id: Uuid, conn: &PgConnection) -> Result<Vec<Player>,Errors> {
    players::table
            .filter(players::event_id.eq(event_id))
            .load::<Player>(conn)
            .map_err(|_| Errors::QueryFailed)
}

pub fn insert(player: Player, conn: &PgConnection) -> Result<Player, Errors> {
    diesel::insert_into(players::table)
        .values(player)
        .get_result::<Player>(conn)
        .map_err(|_| Errors::InsertFailed)
}
