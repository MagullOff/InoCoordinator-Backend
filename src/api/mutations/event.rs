use crate::api::DbConn;
use crate::services::event;
use crate::types::event::NewEvent;
use crate::types::Event;
use crate::types::token::OrganizerToken;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};

#[post("/event", format="application/json", data="<body>", rank=1)]
async fn add_event(
    conn: DbConn,
    token: OrganizerToken,
    body: Json<NewEvent>
) -> Result<Json<Event>, status::BadRequest<()>> {
    conn.run(move |c| {
        token.check(c)?;
        let new_event = body.into_inner();
        event::add_event(new_event, token.id, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}

pub fn get_routes() -> Vec<Route> {
    routes![add_event]
}
