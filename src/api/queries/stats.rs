use crate::api::DbConn;
use crate::services::{event, player, stats};
use crate::types::stats::{EventStats, PlayerStats};
use crate::types::token::{OrganizerToken, PlayerToken};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use uuid::Uuid;

#[openapi]
#[get("/player/me", rank = 1)]
async fn get_my_stats(
    token: PlayerToken,
    conn: DbConn,
) -> Result<Json<PlayerStats>, status::BadRequest<()>> {
    conn.run(move |c| {
        token.check(c)?;
        stats::get_player_stats(token.id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[openapi]
#[get("/player/<id_string>", rank = 2)]
async fn get_player_stats(
    id_string: String,
    token: OrganizerToken,
    conn: DbConn,
) -> Result<Json<PlayerStats>, status::BadRequest<()>> {
    let id = Uuid::parse_str(&id_string).map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        token.check(c)?;
        let player = player::get_by_id(id, c).map_err(|_| status::BadRequest(None))?;
        let event = event::get_by_id(player.event_id, c).map_err(|_| status::BadRequest(None))?;
        if event.organizer_id != token.id {
            return Err(status::BadRequest(None));
        }

        stats::get_player_stats(id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[openapi]
#[get("/event/<id_string>", rank = 1)]
async fn get_event_stats(
    id_string: String,
    token: OrganizerToken,
    conn: DbConn,
) -> Result<Json<EventStats>, status::BadRequest<()>> {
    let id = Uuid::parse_str(&id_string).map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        token.check(c)?;
        let event = event::get_by_id(id, c).map_err(|_| status::BadRequest(None))?;

        if event.organizer_id != token.id {
            return Err(status::BadRequest(None));
        }

        stats::get_event_stats(id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_my_stats, get_player_stats, get_event_stats]
}
