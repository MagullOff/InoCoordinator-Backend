use crate::types::token::{OrganizerToken, PlayerToken};
use crate::Errors;
use rocket::request::{FromRequest, Outcome, Request};

use rocket::http::Status;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OrganizerToken {
    type Error = Errors;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let h = req.headers().get_one("Authorization");
        match h {
            None => Outcome::Failure((Status::BadRequest, Errors::Unauthorized)),
            Some(t) => {
                let parts = t.split("@").collect::<Vec<&str>>();
                match parts.len() {
                    2 => Outcome::Success(OrganizerToken {
                        id: uuid::Uuid::parse_str(parts[0]).unwrap(),
                        access_code: parts[1].parse().unwrap_or(0),
                    }),
                    _ => Outcome::Failure((Status::BadRequest, Errors::Unauthorized)),
                }
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PlayerToken {
    type Error = Errors;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let h = req.headers().get_one("Authorization");
        match h {
            None => Outcome::Failure((Status::BadRequest, Errors::Unauthorized)),
            Some(t) => {
                let parts = t.split("@").collect::<Vec<&str>>();
                match parts.len() {
                    2 => Outcome::Success(PlayerToken {
                        id: uuid::Uuid::parse_str(parts[0]).unwrap(),
                        access_code: parts[1].parse().unwrap_or(0),
                    }),
                    _ => Outcome::Failure((Status::BadRequest, Errors::Unauthorized)),
                }
            }
        }
    }
}

use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

impl<'r> OpenApiFromRequest<'r> for OrganizerToken {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        //TODO: Authorization header
        Ok(RequestHeaderInput::None)
    }
}

impl<'r> OpenApiFromRequest<'r> for PlayerToken {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        //TODO: Authorization header
        Ok(RequestHeaderInput::None)
    }
}
