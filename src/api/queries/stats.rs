use rocket::{routes, Route};
use crate::api::DbConn;
use crate::types::stats::{PlayerStats, EventStats};
use crate::services::{stats, player, event};
use crate::types::token::{PlayerToken, OrganizerToken};
use rocket::response::status;
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/stats/player/me",  rank=1)]
async fn get_my_stats(
    token: PlayerToken,
    conn: DbConn
) -> Result<Json<PlayerStats>, status::BadRequest<()>> {
    conn.run(move |c| {
        token.check(c)?;
        stats::get_player_stats(token.id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}

#[get("/stats/player/<id_string>",  rank=2)]
async fn get_player_stats(
    id_string: String,
    token: OrganizerToken,
    conn: DbConn
) -> Result<Json<PlayerStats>, status::BadRequest<()>> {
    let id = Uuid::parse_str(&id_string)
        .map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        token.check(c)?;
        let player = player::get_by_id(id, c)
            .map_err(|_| status::BadRequest(None))?;
        let event = event::get_by_id(player.event_id, c)
            .map_err(|_| status::BadRequest(None))?;
        if event.organizer_id != token.id {
            return Err(status::BadRequest(None));
        }

        stats::get_player_stats(id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}

#[get("/stats/event/<id_string>",  rank=1)]
async fn get_event_stats(
    id_string: String,
    token: OrganizerToken,
    conn: DbConn
) -> Result<Json<EventStats>, status::BadRequest<()>> {
    let id = Uuid::parse_str(&id_string)
        .map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        token.check(c)?;
        let event = event::get_by_id(id, c)
            .map_err(|_| status::BadRequest(None))?;

        if event.organizer_id != token.id {
            return Err(status::BadRequest(None));
        }
        
        stats::get_event_stats(id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}

pub fn get_routes() -> Vec<Route> {
    routes![get_my_stats, get_player_stats, get_event_stats]
}
