use crate::api::DbConn;
use crate::services::organizer;
use crate::types::organizer::{LoginOrganizer, NewOrganizer};
use crate::types::Organizer;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};

#[post(
    "/organizer/add",
    format = "application/json",
    data = "<body>",
    rank = 1
)]
async fn add_organizer(
    conn: DbConn,
    body: Json<NewOrganizer>,
) -> Result<Json<Organizer>, status::BadRequest<()>> {
    conn.run(move |c| {
        let new_organizer = body.into_inner();
        organizer::add_organizer(new_organizer, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

#[post(
    "/organizer/login",
    format = "application/json",
    data = "<body>",
    rank = 1
)]
async fn organizer_login(
    conn: DbConn,
    body: Json<LoginOrganizer>,
) -> Result<Json<Organizer>, status::BadRequest<()>> {
    conn.run(move |c| {
        let organizer_code = body
            .into_inner()
            .code
            .parse::<i32>()
            .map_err(|_| status::BadRequest(None))?;
        organizer::get_by_code(organizer_code, &c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes() -> Vec<Route> {
    routes![add_organizer, organizer_login]
}
