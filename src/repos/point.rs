use crate::schema::points;
use crate::types::Point;
use crate::errors::Errors;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Point, Errors> {
    points::table
        .find(id)
        .first::<Point>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_by_code(code: String, conn: &PgConnection) -> Result<Point, Errors> {
    points::table
        .filter(points::code.eq_all(code))
        .first::<Point>(conn)
        .map_err(|_| Errors::QueryFailed)
}

pub fn get_by_event(event_id: Uuid, conn: &PgConnection) -> Result<Vec<Point>,Errors> {
    points::table
            .filter(points::event_id.eq(event_id))
            .load::<Point>(conn)
            .map_err(|_| Errors::QueryFailed)
}

pub fn insert(point: Point, conn: &PgConnection) -> Result<Point, Errors> {
    diesel::insert_into(points::table)
        .values(point)
        .get_result::<Point>(conn)
        .map_err(|_| Errors::InsertFailed)
}
