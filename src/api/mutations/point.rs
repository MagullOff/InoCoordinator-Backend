use crate::api::DbConn;
use crate::services::{event, point};
use crate::types::point::NewPoint;
use crate::types::token::OrganizerToken;
use crate::types::Point;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use uuid::Uuid;

#[openapi]
#[post(
    "/<event_id_string>",
    format = "application/json",
    data = "<body>",
    rank = 1
)]
async fn add_point(
    conn: DbConn,
    token: OrganizerToken,
    event_id_string: String,
    body: Json<NewPoint>,
) -> Result<Json<Point>, status::BadRequest<()>> {
    let event_id = Uuid::parse_str(&event_id_string).map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        token.check(c)?;
        let new_point = body.into_inner();
        let event = event::get_by_id(event_id, c).map_err(|_| status::BadRequest(None))?;
        if event.organizer_id != token.id {
            return Err(status::BadRequest(None));
        }
        point::add_point(new_point, event_id, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![settings: add_point]
}
