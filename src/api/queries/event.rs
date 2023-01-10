use crate::api::DbConn;
use crate::services::event;
use crate::types::token::OrganizerToken;
use crate::types::Event;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use uuid::Uuid;

#[openapi]
#[get("/<event_id_string>", rank = 2)]
async fn get_by_id(
    event_id_string: String,
    conn: DbConn,
) -> Result<Json<Event>, status::BadRequest<()>> {
    let event_id = Uuid::parse_str(&event_id_string).map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        event::get_by_id(event_id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[openapi]
#[get("/events/me", rank = 1)]
async fn get_my_events(
    token: OrganizerToken,
    conn: DbConn,
) -> Result<Json<Vec<Event>>, status::BadRequest<()>> {
    conn.run(move |c| {
        token.check(c)?;
        event::get_by_organizer(token.id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_by_id, get_my_events]
}
