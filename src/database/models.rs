use crate::schema::counters;
use rocket_okapi::JsonSchema;
use serde::Serialize;
// note: JsonSchema for OpenAPI doc, Queryable for diesel, sqlx::FromRow for sqlx_postgres
#[derive(Queryable, Clone, Debug, Serialize, JsonSchema, sqlx::FromRow)]
pub struct Counter {
    pub id: i32,
    pub name: String,
    pub counter: i32,
}

#[derive(Insertable)]
#[table_name = "counters"]
pub struct NewCounter {
    pub name: String,
    pub counter: i32,
}
