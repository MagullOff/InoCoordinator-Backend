use crate::api::DbConn;
use crate::types::Point;
use crate::services::point;
use rocket::response::status;
use rocket::serde::json::Json;
use uuid::Uuid;
use rocket::{routes, Route};

#[get("/point/<id_string>",  rank=2)]
async fn get_by_id(
    id_string: String,
    conn: DbConn
) -> Result<Json<Point>, status::BadRequest<()>> {
    let id = Uuid::parse_str(&id_string)
        .map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        point::get_by_id(id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}

#[get("/point/event/<id_string>",  rank=1)]
async fn get_by_event(
    id_string: String,
    conn: DbConn
) -> Result<Json<Vec<Point>>, status::BadRequest<()>> {
    let id = Uuid::parse_str(&id_string)
        .map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        point::get_by_event(id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}

pub fn get_routes() -> Vec<Route> {
    routes![get_by_id, get_by_event]
}
