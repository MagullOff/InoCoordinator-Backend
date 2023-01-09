use crate::repos::OrganizerRepo;
use crate::types::organizer::NewOrganizer;
use crate::types::Organizer;
use crate::Errors;
use diesel::PgConnection;
use rand::Rng;
use uuid::Uuid;

pub fn add_organizer(
    new_organizer: NewOrganizer,
    conn: &PgConnection,
) -> Result<Organizer, Errors> {
    let mut rng = rand::thread_rng();
    let organizer = Organizer {
        id: Uuid::new_v4(),
        name: new_organizer.name,
        email: new_organizer.email,
        access_code: (rng.gen::<f64>() * 1000000.0) as i32,
    };
    OrganizerRepo::insert(organizer, conn)
}

pub fn get_by_code(code: i32, conn: &PgConnection) -> Result<Organizer, Errors> {
    OrganizerRepo::get_by_code(code, conn)
}

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Organizer, Errors> {
    OrganizerRepo::get_by_id(id, conn)
}
