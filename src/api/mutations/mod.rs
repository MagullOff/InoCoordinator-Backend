use rocket::Route;
pub mod organizer;


pub fn get_routes() -> Vec<Route> {
    vec![
        organizer::get_routes()
    ]
    .into_iter()
    .flatten()
    .collect()
}
