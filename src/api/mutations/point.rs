use crate::api::DbConn;
use crate::services::{point, event};
use crate::types::point::NewPoint;
use crate::types::Point;
use crate::types::token::OrganizerToken;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};

#[post("/point", format="application/json", data="<body>", rank=1)]
async fn add_event(
    conn: DbConn,
    token: OrganizerToken,
    body: Json<NewPoint>
) -> Result<Json<Point>, status::BadRequest<()>> {
    conn.run(move |c| {
        token.check(c)?;
        let new_point = body.into_inner();
        let event = event::get_by_id(new_point.event_id, c)
            .map_err(|_| status::BadRequest(None))?;
        if event.organizer_id != token.id {
            return Err(status::BadRequest(None))
        }
        point::add_point(new_point, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    }).await
}

pub fn get_routes() -> Vec<Route> {
    routes![add_event]
}
