use crate::schema::counters;
use rocket_okapi::JsonSchema;
#[derive(Queryable, Debug, JsonSchema)]
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
