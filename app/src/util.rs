use super::DbPool;
// use sqlx::postgres::PgPoolOptions;
// use sqlx::mysql::MySql
/// Create a PostgreSQL database connection pool.
///
/// The pool will be configured to have at least 1 connection and up to
/// `max_connections` for the given `db_conn_url`.
///
/// Will return the configured connection pool or panic.
pub async fn create_db_conn_pool(db_conn_url: &String, max_connections: u32) -> DbPool {
    let conn = sqlx::MySqlPool::connect(&db_conn_url)
        .await.unwrap();
    // PgPoolOptions::new()
    //     .max_connections(max_connections)
    //     .connect(&db_conn_url)
    //     .await
    //     .expect("Failed to establish database connection pool")

    conn
}
