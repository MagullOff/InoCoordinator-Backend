use crate::api::DbConn;
use crate::services::{event, player};
use crate::types::player::{LoginPlayer, NewPlayer};
use crate::types::token::OrganizerToken;
use crate::types::Player;
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
    rank = 2
)]
async fn add_player(
    conn: DbConn,
    token: OrganizerToken,
    event_id_string: String,
    body: Json<NewPlayer>,
) -> Result<Json<Player>, status::BadRequest<()>> {
    let event_id = Uuid::parse_str(&event_id_string).map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        token.check(c)?;
        let new_player = body.into_inner();
        let event = event::get_by_id(event_id, c).map_err(|_| status::BadRequest(None))?;
        if event.organizer_id != token.id {
            return Err(status::BadRequest(None));
        }
        player::add_player(new_player, event_id, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[openapi]
#[post("/login", format = "application/json", data = "<body>", rank = 1)]
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

pub fn get_routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![settings: add_player, player_login]
}
