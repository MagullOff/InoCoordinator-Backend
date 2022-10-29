use uuid::Uuid;
use rocket::response::status;
use crate::repos::{OrganizerRepo, PlayerRepo};
use crate::types::{Organizer, Player};
use diesel::PgConnection;

pub struct OrganizerToken{
    pub id: Uuid,
    pub access_code: i32
}

pub struct PlayerToken{
    pub id: Uuid,
    pub access_code: i32
}

impl OrganizerToken {
    pub fn check(&self, conn: &PgConnection) -> Result<Organizer, status::BadRequest<()>> {
        match OrganizerRepo::get_by_id(self.id, conn) {
            Ok(organizer) => {
                if organizer.access_code == self.access_code {
                    Ok(organizer)
                } else {
                    Err(status::BadRequest(None))
                }
            },
            Err(_) => Err(status::BadRequest(None))
        }
    }
}

impl PlayerToken {
    pub fn check(&self, conn: &PgConnection) -> Result<Player, status::BadRequest<()>> {
        match PlayerRepo::get_by_id(self.id, conn) {
            Ok(player) => {
                if player.access_code == self.access_code {
                    Ok(player)
                } else {
                    Err(status::BadRequest(None))
                }
            },
            Err(_) => Err(status::BadRequest(None))
        }
    }
}
