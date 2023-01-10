use crate::api::DbConn;
use crate::services::event;
use crate::types::event::NewEvent;
use crate::types::token::OrganizerToken;
use crate::types::Event;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_okapi::settings::OpenApiSettings;

use rocket::{routes, Route};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};

#[openapi]
#[post("/", format = "application/json", data = "<body>", rank = 1)]
async fn add_event(
    conn: DbConn,
    token: OrganizerToken,
    body: Json<NewEvent>,
) -> Result<Json<Event>, status::BadRequest<()>> {
    conn.run(move |c| {
        token.check(c)?;
        let new_event = body.into_inner();
        event::add_event(new_event, token.id, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![settings: add_event]
}
