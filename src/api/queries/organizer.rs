use crate::api::DbConn;
use crate::types::Organizer;
use crate::types::token::OrganizerToken;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};

#[get("/organizer/me",  rank=1)]
async fn organizer_me(
    conn: DbConn,
    token: OrganizerToken
) -> Result<Json<Organizer>, status::BadRequest<()>> {
    conn.run(move |c| {
        token.check(c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}

pub fn get_routes() -> Vec<Route> {
    routes![organizer_me]
}
