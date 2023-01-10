use crate::api::DbConn;
use crate::services::{event, player};
use crate::types::token::{OrganizerToken, PlayerToken};
use crate::types::Player;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use uuid::Uuid;

#[openapi]
#[get("/event/<id_string>", rank = 2)]
async fn get_by_event(
    id_string: String,
    token: OrganizerToken,
    conn: DbConn,
) -> Result<Json<Vec<Player>>, status::BadRequest<()>> {
    let id = Uuid::parse_str(&id_string).map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        let organizer = token.check(c)?;
        let event = event::get_by_id(id, c).map_err(|_| status::BadRequest(None))?;
        if event.organizer_id != organizer.id {
            return Err(status::BadRequest(None));
        }
        player::get_by_event(id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[openapi]
#[get("/me", rank = 1)]
async fn player_me(
    conn: DbConn,
    token: PlayerToken,
) -> Result<Json<Player>, status::BadRequest<()>> {
    conn.run(move |c| {
        token
            .check(c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![settings: player_me, get_by_event]
}
