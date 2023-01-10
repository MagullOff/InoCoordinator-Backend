use crate::api::DbConn;
use crate::services::point;
use crate::types::Point;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use uuid::Uuid;

#[openapi]
#[get("/<id_string>", rank = 2)]
async fn get_by_id(id_string: String, conn: DbConn) -> Result<Json<Point>, status::BadRequest<()>> {
    let id = Uuid::parse_str(&id_string).map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        point::get_by_id(id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[openapi]
#[get("/event/<id_string>", rank = 1)]
async fn get_by_event(
    id_string: String,
    conn: DbConn,
) -> Result<Json<Vec<Point>>, status::BadRequest<()>> {
    let id = Uuid::parse_str(&id_string).map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        point::get_by_event(id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_by_id, get_by_event]
}
