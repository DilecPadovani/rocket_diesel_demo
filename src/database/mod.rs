pub mod models;

pub mod actions {
    use super::models::{Counter, NewCounter};
    use crate::diesel::ExpressionMethods;
    use crate::diesel::RunQueryDsl;
    use diesel::associations::HasTable;
    use diesel::pg::PgConnection;
    use diesel::OptionalExtension;

    pub fn add(conn: &PgConnection, to_add_counter: NewCounter) -> diesel::QueryResult<Counter> {
        use crate::schema::counters::dsl::*;
        diesel::insert_into(counters)
            .values(&to_add_counter)
            .on_conflict(name)
            .do_update()
            .set(counter.eq(counter + to_add_counter.counter))
            .get_result::<Counter>(conn)
    }

    pub fn subtract(
        conn: &PgConnection,
        to_subtract_counter: NewCounter,
    ) -> diesel::QueryResult<Counter> {
        use crate::schema::counters::dsl::*;
        diesel::insert_into(counters)
            .values(&to_subtract_counter)
            .on_conflict(name)
            .do_update()
            .set(counter.eq(counter - to_subtract_counter.counter))
            .get_result::<Counter>(conn)
    }

    pub fn get_counter_by_name(
        conn: &PgConnection,
        _name: String,
    ) -> Result<Option<Counter>, diesel::result::Error> {
        use crate::diesel::QueryDsl;
        use crate::schema::counters::dsl::*;
        counters::table()
            .filter(name.eq(_name))
            .first::<Counter>(conn)
            .optional()
    }

    pub fn get_all_counters(conn: &PgConnection) -> diesel::QueryResult<Vec<Counter>> {
        use crate::schema::counters::dsl::*;
        counters::table().load(conn)
    }
}
