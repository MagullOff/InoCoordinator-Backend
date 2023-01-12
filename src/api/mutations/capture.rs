use crate::api::DbConn;
use crate::repos::PointRepo;
use crate::services::capture;
use crate::types::token::PlayerToken;
use crate::types::Capture;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};

#[openapi]
#[post("/<point_code>", rank = 1)]
async fn add_capture(
    point_code: String,
    conn: DbConn,
    token: PlayerToken,
) -> Result<Json<Capture>, status::BadRequest<()>> {
    conn.run(move |c| {
        token.check(c)?;
        let point = PointRepo::get_by_code(point_code, c).map_err(|_| status::BadRequest(None))?;
        capture::add_capture(point.id, token.id, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![settings: add_capture]
}
