#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
mod database;
mod schema;

use database::models::{Counter, NewCounter};
use rocket::{fairing::AdHoc, serde::json::Json, *};
use rocket_okapi::{
    gen::OpenApiGenerator,
    openapi, openapi_get_routes,
    request::{OpenApiFromRequest, RequestHeaderInput},
    swagger_ui::*,
};
use rocket_sync_db_pools::{database, diesel::PgConnection};
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

/// # Home page
///
/// Get all records in database
#[openapi(tag = "Home")]
#[get("/")]
async fn all(conn: DbConn) -> Json<Vec<Counter>> {
    let counters = conn
        .run(|c| database::actions::get_all_counters(&c))
        .await
        .unwrap();
    Json(counters)
}

#[openapi(tag = "Counters")]
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

#[openapi(tag = "Counters")]
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

#[openapi(tag = "Counters")]
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

#[launch]
async fn rocket() -> _ {
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
}
