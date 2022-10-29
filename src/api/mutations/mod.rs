use rocket::Route;

pub mod organizer;
pub mod point;
pub mod player;
pub mod event;
pub mod capture;

pub fn get_routes() -> Vec<Route> {
    vec![
        organizer::get_routes(),
        player::get_routes(),
        event::get_routes(),
        capture::get_routes()
    ]
    .into_iter()
    .flatten()
    .collect()
}
