use crate::api::DbConn;
use crate::types::token::OrganizerToken;
use crate::types::Organizer;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};

#[openapi]
#[get("/me", rank = 1)]
async fn organizer_me(
    conn: DbConn,
    token: OrganizerToken,
) -> Result<Json<Organizer>, status::BadRequest<()>> {
    conn.run(move |c| {
        token
            .check(c)
            .map(|r| Json(r))
            .map_err(|_| status::BadRequest(None))
    })
    .await
}

pub fn get_routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![settings: organizer_me]
}
