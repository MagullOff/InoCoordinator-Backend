use crate::api::DbConn;
use crate::services::organizer;
use crate::types::organizer::{LoginOrganizer, NewOrganizer};
use crate::types::Organizer;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};

#[openapi]
#[post("/add", format = "application/json", data = "<body>", rank = 1)]
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

#[openapi]
#[post("/login", format = "application/json", data = "<body>", rank = 1)]
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

pub fn get_routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![settings: add_organizer, organizer_login]
}
