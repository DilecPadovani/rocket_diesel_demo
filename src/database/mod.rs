pub mod models;

pub mod actions {
    pub mod with_sqlx {
        use sqlx::{pool::PoolConnection, Postgres};

        use crate::database::models::Counter;

        // use postgres::{, NoTls};

        pub async fn all(
            conn: &mut PoolConnection<Postgres>,
        ) -> Result<Vec<Counter>, sqlx::error::Error> {
            sqlx::query_as::<_, Counter>("SELECT * FROM counters")
                .fetch_all(conn)
                .await
        }
    }
}
