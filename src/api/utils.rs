use crate::types::token::{OrganizerToken, PlayerToken};
use crate::Errors;
use okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData};
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
        let security_scheme = SecurityScheme {
            description: Some(
                "Requires an Organizer token to access. Token format is organizer-id@passcode"
                    .to_owned(),
            ),
            data: SecuritySchemeData::Http {
                scheme: "Organizer".to_owned(),
                bearer_format: Some("Organizer".to_owned()),
            },
            extensions: Object::default(),
        };
        let mut security_req = SecurityRequirement::new();
        security_req.insert("Authorization".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "Authorization".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}

impl<'r> OpenApiFromRequest<'r> for PlayerToken {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some(
                "Requires a Player token to access. Token format is player-id@passcode".to_owned(),
            ),
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(),
                bearer_format: Some("@".to_owned()),
            },
            extensions: Object::default(),
        };
        let mut security_req = SecurityRequirement::new();
        security_req.insert("Authorization".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "Authorization".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}
