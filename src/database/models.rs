use crate::schema::counters;

#[derive(Queryable, Debug)]
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
