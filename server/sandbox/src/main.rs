use deadpool_postgres::{tokio_postgres::NoTls, Config, Runtime};

#[tokio::main]
async fn main() {
    let mut config = Config::new();
    config.dbname = Some("localc".to_owned());
    config.user = Some("localc".to_owned());
    config.password = Some("localc".to_owned());
    config.port = Some(15432);

    let _pool = config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let mut _conn = _pool.get().await.unwrap();

    todo!()
}
