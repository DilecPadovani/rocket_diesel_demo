#![feature(proc_macro_hygiene, decl_macro)]
#![allow(warnings)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
mod database;

mod schema;
use database::models::{Counter, NewCounter};
use rocket::{fairing::AdHoc, outcome, outcome::Outcome, *};
// use rocket_contrib::{
//     databases::{database, diesel::PgConnection},
//     json::Json,
// };

// use rocket_okapi::okapi::schemars;
// use rocket_okapi::okapi::schemars::JsonSchema;
// use rocket_okapi::settings::UrlObject;

use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
// use rocket::response::status;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_sync_db_pools::Connection;
// to show how request guard works in rocket: usefull to implicitly check requirements
// const TOKEN: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/token"));
use rocket::serde::json::Json;
use rocket_sync_db_pools::{database, diesel::PgConnection};
// #[derive(DerefMut)]
#[database("test_db")]
struct DbConn(PgConnection);

impl<'r> OpenApiFromRequest<'r> for DbConn {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

// use rocket::response::Result;
#[openapi(tag = "xxxxxx")]
#[get("/")]
async fn all(conn: DbConn) -> Json<Counter> {
    let counters = conn
        .run(|c| database::actions::get_all_counters(&c))
        .await
        .unwrap()
        .first()
        .unwrap()
        .clone();
    Json(counters)
}
#[openapi]
#[get("/add/<name>/<number>")]
async fn add(name: String, number: u32, conn: DbConn) -> String {
    let _counter = NewCounter {
        name,
        counter: number as i32,
    };
    let x = conn
        .run(|c| database::actions::add(&c, _counter).unwrap())
        .await;

    format!("Added {:?}", x)
}

#[openapi]
#[get("/subtract/<name>/<number>")]
async fn subtract(name: String, number: u32, conn: DbConn) -> String {
    let _counter = NewCounter {
        name,
        counter: number as i32,
    };
    let x = conn
        .run(|c| database::actions::subtract(&c, _counter))
        .await;

    format!("Subtracted: {:?}", x)
}

#[openapi]
#[get("/status/<name>")]
async fn status(name: String, conn: DbConn) -> String {
    let x = conn
        .run(|c| database::actions::get_counter_by_name(&c, name))
        .await;
    format!("Hello, {:?} ", x)
}

async fn run_db_migrations<P: Phase>(rocket: Rocket<P>) -> Result<Rocket<P>, Rocket<P>> {
    let conn = DbConn::get_one(&rocket)
        .await
        .expect("Failed to establish database connection");
    diesel_migrations::embed_migrations!();

    if let Err(e) = conn.run(|c| embedded_migrations::run(c)).await {
        eprintln!("Failed to run database migrations: {:?}", e);
        return Err(rocket);
    }
    Ok(rocket)
}

#[rocket::main]
async fn main() {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::try_on_ignite(
            "Initialise server schema",
            run_db_migrations,
        ))
        .mount("/", openapi_get_routes![all, add, subtract, status])
        .mount(
            "/docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await;
}
