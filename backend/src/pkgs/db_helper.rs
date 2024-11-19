use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

// https://docs.rs/diesel/latest/diesel/r2d2/index.html
pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_host = std::env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let database_port = std::env::var("DATABASE_PORT").expect("DATABASE_PORT must be set");
    let database_user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let database_password =
        std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let database_name = std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database_user, database_password, database_host, database_port, database_name
    );
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create pool")
}
