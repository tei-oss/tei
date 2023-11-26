use deadpool_postgres::{CreatePoolError, Pool};
use tei_core::env;

pub(crate) mod auxiliary;
pub(crate) mod extensions;
pub mod tags;

pub fn create_pool() -> Result<Pool, CreatePoolError> {
    let mut config = deadpool_postgres::Config::new();
    config.host = env::get("PG_HOST");
    config.dbname = env::get("PG_DB");
    config.user = env::get("PG_USER");
    config.password = env::get("PG_PASSWORD");
    config.port = env::get("PG_PORT").map(|s| s.parse().expect("PG_PORT is "));
    config.application_name = env::get("PG_APPNAME");

    let pool = config.create_pool(
        Some(deadpool_postgres::Runtime::Tokio1),
        tokio_postgres::NoTls,
    )?;

    Ok(pool)
}
