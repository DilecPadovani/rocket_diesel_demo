#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
mod database;
mod schema;

use database::models::{Counter, NewCounter};
use rocket::{fairing::AdHoc, *};
use rocket_contrib::{
    databases::{database, diesel::PgConnection},
    json::Json,
};

// use rocket_okapi::okapi::schemars;
// use rocket_okapi::okapi::schemars::JsonSchema;
// use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, routes_with_openapi, swagger_ui::*};

use rocket::response::status;

// to show how request guard works in rocket: usefull to implicitly check requirements
// const TOKEN: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/token"));
#[database("test_db")]
struct DbConn(PgConnection);

#[openapi]
#[get("/")]
fn all(conn: DbConn) -> String {
    if let Ok(counters) = database::actions::get_all_counters(&*conn) {
        format!("{:#?}", counters)
    } else {
        return "Could not get counters in the database".to_string();
    }
}
#[openapi]
#[get("/add/<name>/<number>")]
fn add(name: String, number: u32, conn: DbConn) -> String {
    let counter = NewCounter {
        name,
        counter: number as i32,
    };
    let x = database::actions::add(&*conn, counter).unwrap();

    format!("Added {:?}", x)
}

#[openapi]
#[get("/subtract/<name>/<number>")]
fn subtract(name: String, number: u32, conn: DbConn) -> String {
    let counter = NewCounter {
        name,
        counter: number as i32,
    };
    let x = database::actions::subtract(&*conn, counter);

    format!("Subtracted: {:?}", x)
}

#[openapi]
#[get("/status/<name>")]
fn status(name: String, conn: DbConn) -> String {
    let x = database::actions::get_counter_by_name(&*conn, name);
    format!("Hello, {:?} ", x)
}

fn run_db_migrations<'r>(rocket: &'r Rocket) {
    let conn = DbConn::get_one(&rocket).expect("database connection");
    diesel_migrations::embed_migrations!();

    if let Err(e) = embedded_migrations::run(&*conn) {
        eprintln!("Failed to run database migrations: {:?}", e);
    }
}
fn main() {
    // diesel_migrations::embed_migrations!();
    // embedded_migrations::run();
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_launch(
            "Initialise server schema",
            run_db_migrations,
        ))
        .mount("/", routes_with_openapi![all, add, subtract, status])
        .mount(
            "/docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch();
}
