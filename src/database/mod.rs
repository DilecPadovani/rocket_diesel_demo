pub mod models;

pub mod actions {
    pub mod with_postgres_crate {
        use postgres;

        use crate::database::models::{Counter, NewCounter};

        // use postgres::{, NoTls};

        pub fn all(conn: &mut postgres::Client) -> Result<Vec<Counter>, postgres::Error> {
            let counters = conn
                .query("select * from counters", &[])?
                .iter()
                .map(|row| Counter {
                    // input of get is column name or positional argument
                    // id: row.get(0),
                    id: row.get("id"),
                    name: row.get("name"),
                    counter: row.get("counter"),
                })
                .collect();
            Ok(counters)
        }

        pub fn add(
            conn: &mut postgres::Client,
            to_add_counter: NewCounter,
        ) -> Result<Counter, postgres::Error> {
            let row = conn
                .query_one("INSERT INTO counters (name, counter) VALUES ($1, $2) ON CONFLICT (name) DO UPDATE SET counter = counters.counter + $2 RETURNING *", &[&to_add_counter.name, &to_add_counter.counter ])?;
            Ok(Counter {
                id: row.get("id"),
                name: row.get("name"),
                counter: row.get("counter"),
            })
        }

        pub fn subtract(
            conn: &mut postgres::Client,
            to_add_counter: NewCounter,
        ) -> Result<Counter, postgres::Error> {
            let row = conn
                .query_one("INSERT INTO counters (name,counter) VALUES ($1, $2) ON CONFLICT (name) DO UPDATE SET counter = counters.counter + $2 RETURNING *", &[ &to_add_counter.name, &to_add_counter.counter ])?;
            Ok(Counter {
                id: row.get("id"),
                name: row.get("name"),
                counter: row.get("counter"),
            })
        }
    }
}
