use crate::repos::PointRepo;
use crate::types::Point;
use crate::types::point::NewPoint;
use uuid::Uuid;
use crate::Errors;
use diesel::PgConnection;
use random_string::generate;

pub fn add_point(new_point: NewPoint, conn: &PgConnection) -> Result<Point, Errors> {
    let point = Point {
        id: Uuid::new_v4(),
        name: new_point.name,
        event_id: new_point.event_id,
        code: generate(7, "abcdefghijklmnopqrstuwxyz1234567890")
    };
    PointRepo::insert(point, conn)
}

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Point, Errors> {
    PointRepo::get_by_id(id, conn)
}

pub fn get_by_event(event_id: Uuid, conn: &PgConnection) -> Result<Vec<Point>, Errors> {
    PointRepo::get_by_event(event_id, conn)
}
