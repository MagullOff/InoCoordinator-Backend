use crate::api::mutations;
use crate::api::queries;
use dotenv::dotenv;
use rocket::config::Config;
use rocket::figment::Figment;
use rocket::figment::{
    util::map,
    value::{Map, Value},
};
use rocket::http::Method;
use rocket::Rocket;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::request::OpenApiFromRequest;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_sync_db_pools::{database, diesel};

#[derive(OpenApiFromRequest)]
#[database("InoOrganizer")]
pub struct DbConn(diesel::PgConnection);

pub fn init_routes() -> Rocket<rocket::Build> {
    dotenv().ok();

    let port_number: u16 = std::env::var("ROCKET_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    let db: Map<_, Value> = map! {
        "url" => std::env::var("DATABASE_URL").unwrap().into(),
        "pool_size" => 10.into()
    };

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Put]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .unwrap();

    let config = Figment::from(Config::default())
        .merge(("port", port_number))
        .merge(("address", "0.0.0.0"))
        .merge(("databases", map!["InoOrganizer" => db]));

    println!("mounting");

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();

    let mut building_rocket = rocket::custom(config)
        .attach(DbConn::fairing())
        .attach(cors)
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        );

    mount_endpoints_and_merged_docs! {
        building_rocket, "/".to_owned(), openapi_settings,
        "/capture" => mutations::capture::get_routes(&openapi_settings),
        "/event" => mutations::event::get_routes(&openapi_settings),
        "/organizer" => mutations::organizer::get_routes(&openapi_settings),
        "/player" => mutations::player::get_routes(&openapi_settings),
        "/point" => mutations::point::get_routes(&openapi_settings),
        "/stats" => queries::stats::get_routes(&openapi_settings),
        "/event" => queries::event::get_routes(&openapi_settings),
        "/organizer" => queries::organizer::get_routes(&openapi_settings),
        "/player" => queries::player::get_routes(&openapi_settings),
        "/point" => queries::point::get_routes(&openapi_settings),
    };

    building_rocket
}
