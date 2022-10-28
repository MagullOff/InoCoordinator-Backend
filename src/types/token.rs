use crate::types::Organizer;
use crate::Errors;
use diesel::PgConnection;
use uuid::Uuid;
pub struct OrganizerToken{
    pub id: Uuid,
    pub access_code: i32
}

pub struct PlayerToken{
    pub id: Uuid,
    pub access_code: i32
}

impl OrganizerToken {
    pub fn check(&self, conn: &PgConnection) -> Result<Organizer, Errors> {

    }
}
