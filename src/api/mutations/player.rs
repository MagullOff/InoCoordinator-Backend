use crate::api::DbConn;
use crate::services::{event, player};
use crate::types::player::{LoginPlayer, NewPlayer};
use crate::types::token::OrganizerToken;
use crate::types::Player;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};

#[post("/player", format = "application/json", data = "<body>", rank = 1)]
async fn add_player(
    conn: DbConn,
    token: OrganizerToken,
    body: Json<NewPlayer>,
) -> Result<Json<Player>, status::BadRequest<()>> {
    conn.run(move |c| {
        token.check(c)?;
        let new_player = body.into_inner();
        let event =
            event::get_by_id(new_player.event_id, c).map_err(|_| status::BadRequest(None))?;
        if event.organizer_id != token.id {
            return Err(status::BadRequest(None));
        }
        player::add_player(new_player, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[post(
    "/player/login",
    format = "application/json",
    data = "<body>",
    rank = 1
)]
async fn player_login(
    conn: DbConn,
    body: Json<LoginPlayer>,
) -> Result<Json<Player>, status::BadRequest<()>> {
    conn.run(move |c| {
        let player_code = body
            .into_inner()
            .code
            .parse::<i32>()
            .map_err(|_| status::BadRequest(None))?;
        player::get_by_code(player_code, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![add_player, player_login]
}
