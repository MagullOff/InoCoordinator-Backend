use rocket::Route;

pub mod organizer;
pub mod point;
pub mod event;
pub mod player;
pub mod stats;

pub fn get_routes() -> Vec<Route> {
    vec![
        organizer::get_routes(),
        point::get_routes(),
        event::get_routes(),
        player::get_routes(),
        stats::get_routes()
    ]
    .into_iter()
    .flatten()
    .collect()
}
