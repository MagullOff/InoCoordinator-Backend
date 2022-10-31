use crate::api::DbConn;
use crate::services::capture;
use crate::types::Capture;
use crate::types::token::PlayerToken;
use crate::repos::PointRepo;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};

#[post("/capture/add/<point_code>", rank=1)]
async fn add_capture(
    point_code: String,
    conn: DbConn,
    token: PlayerToken,
) -> Result<Json<Capture>, status::BadRequest<()>> {
    conn.run(move |c| {
        token.check(c)?;
        let point = PointRepo::get_by_code(point_code, c)
            .map_err(|_| status::BadRequest(None))?;
        capture::add_capture(point.id, token.id, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}

pub fn get_routes() -> Vec<Route> {
    routes![add_capture]
}
