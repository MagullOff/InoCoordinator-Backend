use crate::repos::OrganizerRepo;
use crate::types::{Organizer, token::OrganizerToken};
use crate::types::organizer::NewOrganizer;
use uuid::Uuid;
use crate::Errors;
use rand::Rng;
use diesel::PgConnection;

pub fn add_organizer(new_organizer: NewOrganizer, conn: &PgConnection) -> Result<Organizer, Errors> {
    let mut rng = rand::thread_rng();
    let organizer = Organizer {
        id: Uuid::new_v4(),
        name: new_organizer.name,
        email: new_organizer.email,
        access_code: (rng.gen::<f64>() * 1000000.0) as i32
    };
    OrganizerRepo::insert(organizer, conn)
}
