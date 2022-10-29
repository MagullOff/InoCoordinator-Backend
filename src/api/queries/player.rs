use crate::api::DbConn;
use crate::types::Organizer;
use crate::types::token::OrganizerToken;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{routes, Route};

pub fn get_routes() -> Vec<Route> {
    routes![]
}
