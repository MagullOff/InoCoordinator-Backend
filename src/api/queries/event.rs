use crate::api::DbConn;
use crate::types::Event;
use crate::types::token::OrganizerToken;
use crate::services::event;
use rocket::response::status;
use rocket::serde::json::Json;
use uuid::Uuid;
use rocket::{routes, Route};

#[get("/events/<event_id_string>",  rank=2)]
async fn get_by_id(
    event_id_string: String,
    conn: DbConn
) -> Result<Json<Event>, status::BadRequest<()>> {
    let event_id = Uuid::parse_str(&event_id_string)
        .map_err(|_| status::BadRequest(None))?;
    conn.run(move |c| {
        event::get_by_id(event_id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}

#[get("/events/me",  rank=1)]
async fn get_my_events(
    token: OrganizerToken,
    conn: DbConn
) -> Result<Json<Vec<Event>>, status::BadRequest<()>> {
    conn.run(move |c| {
        token.check(c)?;
        event::get_by_organizer(token.id, c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}

pub fn get_routes() -> Vec<Route> {
    routes![get_by_id, get_my_events]
}
