use crate::repos::CaptureRepo;
use crate::types::Capture;
use uuid::Uuid;
use crate::Errors;
use diesel::PgConnection;
use chrono::offset::Utc;

pub fn add_capture(point_id: Uuid, player_id: Uuid, conn: &PgConnection) -> Result<Capture, Errors> {
    let capture = Capture {
        id: Uuid::new_v4(),
        player_id,
        point_id,
        date: Utc::now().naive_utc()
    };
    CaptureRepo::insert(capture, conn)
}

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Capture, Errors> {
    CaptureRepo::get_by_id(id, conn)
}

pub fn get_by_player(player_id: Uuid, conn: &PgConnection) -> Result<Vec<Capture>, Errors> {
    CaptureRepo::get_by_player(player_id, conn)
}
