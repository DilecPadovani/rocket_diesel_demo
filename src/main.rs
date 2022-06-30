#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod database;
mod schema;

use database::models::{Counter, NewCounter};
use rocket::{fairing::AdHoc, serde::json::Json, *};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{
    gen::OpenApiGenerator,
    mount_endpoints_and_merged_docs,
    okapi::openapi3::OpenApi,
    openapi, openapi_get_routes_spec,
    rapidoc::*,
    request::{OpenApiFromRequest, RequestHeaderInput},
    swagger_ui::*,
};

use postgres;
use rocket_db_pools::{sqlx, Connection, Database};
use rocket_sync_db_pools::{database, diesel::PgConnection};

// database connection made using the mayor crtaes in rust, my choice in order would be diesel, Sqlx, Postgres

#[database("test_db")]

struct DieselDbConn(PgConnection);

#[derive(Database)]
#[database("test_db")]
struct SqlxDbConn(sqlx::PgPool);

#[get("/sqlx")]
async fn sqlx_all(mut conn: Connection<SqlxDbConn>) -> Json<Vec<Counter>> {
    // let x = &mut *conn;
    let counters = database::actions::with_sqlx::all(&mut *conn).await.unwrap();
    Json(counters)
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    diesel_migrations::embed_migrations!();

    let conn = DieselDbConn::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}

#[launch]
async fn rocket() -> _ {
    let mut building_rocket = rocket::build()
        .attach(DieselDbConn::fairing())
        .attach(SqlxDbConn::init())
        .attach(AdHoc::on_ignite(
            "Initialise server schema",
            run_db_migrations,
        ))
        .attach(AdHoc::on_liftoff("API STARTED?", |_| {
            Box::pin(async move {
                println!("API is online!!");
            })
        }))
        .mount("/", routes![sqlx_all])
        .mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/docs2/",
            make_rapidoc(&RapiDocConfig {
                title: Some("My special documentation | RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        );
    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    let custom_route_spec = (routes![], custom_openapi_spec());
    mount_endpoints_and_merged_docs! {
        building_rocket, "/".to_owned(), openapi_settings,
        "calcio" => custom_route_spec,
    };
    building_rocket
}

fn custom_openapi_spec() -> OpenApi {
    // use indexmap::indexmap;
    use rocket_okapi::okapi::openapi3::*;
    // use rocket_okapi::okapi::schemars::schema::*;
    OpenApi {
        openapi: OpenApi::default_version(),
        info: Info {
            title: "The best counter API ever".to_owned(),
            description: Some("This is the best API every, please use me!".to_owned()),
            terms_of_service: Some(
                "https://github.com/GREsau/okapi/blob/master/LICENSE".to_owned(),
            ),
            contact: Some(Contact {
                name: Some("Dilec Padovani".to_owned()),
                url: Some("https://github.com/DILECPEDO".to_owned()),
                email: Some("test@test.com".to_owned()),
                ..Default::default()
            }),
            license: Some(License {
                name: "MIT".to_owned(),
                url: Some("https://github.com/GREsau/okapi/blob/master/LICENSE".to_owned()),
                ..Default::default()
            }),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            ..Default::default()
        },
        servers: vec![
            Server {
                url: "http://127.0.0.1:8000/".to_owned(),
                description: Some("Localhost".to_owned()),
                ..Default::default()
            },
            Server {
                url: "https://example.com/".to_owned(),
                description: Some("Production Remote".to_owned()),
                ..Default::default()
            },
        ],
        // Add paths that do not exist in Rocket (or add extra into to existing paths)
        // paths: {
        //     indexmap! {
        //         "/home".to_owned() => PathItem{
        //         get: Some(
        //             Operation {
        //             tags: vec!["HomePage".to_owned()],
        //             summary: Some("This is my homepage".to_owned()),
        //             responses: Responses{
        //                 responses: indexmap!{
        //                 "200".to_owned() => RefOr::Object(
        //                     Response{
        //                     description: "Return the page, no error.".to_owned(),
        //                     content: indexmap!{
        //                         "text/html".to_owned() => MediaType{
        //                         schema: Some(SchemaObject{
        //                             instance_type: Some(SingleOrVec::Single(Box::new(
        //                                 InstanceType::String
        //                             ))),
        //                             ..Default::default()
        //                         }),
        //                         ..Default::default()
        //                         }
        //                     },
        //                     ..Default::default()
        //                     }
        //                 )
        //                 },
        //                 ..Default::default()
        //             },
        //             ..Default::default()
        //             }
        //         ),
        //         ..Default::default()
        //         }
        //     }
        // },
        ..Default::default()
    }
}
