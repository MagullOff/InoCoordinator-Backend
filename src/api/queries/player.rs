use crate::api::DbConn;
use crate::types::Player;
use crate::types::token::{OrganizerToken, PlayerToken};
use crate::services::{player, event};
use rocket::response::status;
use rocket::serde::json::Json;
use uuid::Uuid;
use rocket::{routes, Route};

#[get("/player/event/<id_string>",  rank=2)]
async fn get_by_event(
    id_string: String,
    token: OrganizerToken,
    conn: DbConn
) -> Result<Json<Vec<Player>>, status::BadRequest<()>> {
    let id = Uuid::parse_str(&id_string)
        .map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        let organizer = token.check(c)?;
        let event = event::get_by_id(id, c)
            .map_err(|_| status::BadRequest(None))?;
        if event.organizer_id == organizer.id {
            return Err(status::BadRequest(None));
        }
        player::get_by_event(id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}


#[get("/player/me",  rank=1)]
async fn player_me(
    conn: DbConn,
    token: PlayerToken
) -> Result<Json<Player>, status::BadRequest<()>> {
    conn.run(move |c| {
        token.check(c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}

pub fn get_routes() -> Vec<Route> {
    routes![player_me, get_by_event]
}
