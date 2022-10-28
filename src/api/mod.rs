use rocket::response::status;
use rocket::serde::json::Json;

pub mod utils;
pub mod initial;
pub mod mutations;
pub mod queries;

pub use initial::DbConn;
pub type Response<T> = Result<Json<T>, status::BadRequest<&'static str>>;

