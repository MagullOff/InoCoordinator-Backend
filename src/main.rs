#[macro_use]
extern crate rocket;
use lib::api::initial::init_routes;

#[launch]
fn rocket() -> _ {
    println!("Server is up");
    init_routes()
}
