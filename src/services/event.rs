use crate::repos::EventRepo;
use crate::types::Event;
use crate::types::event::NewEvent;
use uuid::Uuid;
use crate::Errors;
use diesel::PgConnection;

pub fn add_event(new_event: NewEvent, organizer_id: Uuid, conn: &PgConnection) -> Result<Event, Errors> {
    let event = Event {
        id: Uuid::new_v4(),
        name: new_event.name,
        organizer_id
    };
    EventRepo::insert(event, conn)
}

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Event, Errors> {
    EventRepo::get_by_id(id, conn)
}

pub fn get_by_organizer(organizer_id: Uuid, conn: &PgConnection) -> Result<Vec<Event>, Errors> {
    EventRepo::get_by_organizer(organizer_id, conn)
}
