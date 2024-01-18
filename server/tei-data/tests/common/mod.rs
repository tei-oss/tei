use std::thread;

use deadpool_postgres::Pool;
use lazy_static::lazy_static;
use tei_threading::{channel_unbounded, execute_blocking, UnboundedChannel};
use testcontainers::{
    clients::{self},
    RunnableImage,
};
use testcontainers_modules::postgres::Postgres;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

lazy_static! {
    static ref PG_CMD: UnboundedChannel<ContainerCommand> = channel_unbounded();
    static ref PG_STOP: UnboundedChannel<()> = channel_unbounded();
    static ref PG_POOLS: UnboundedChannel<Pool> = channel_unbounded();
}

enum ContainerCommand {
    GetDb,
    Stop,
}

pub fn start_up() {
    thread::spawn(|| execute_blocking(async move { handle_pg().await }));
}
pub fn clean_up() {
    PG_CMD.tx.send(ContainerCommand::Stop).unwrap();

    execute_blocking(async move {
        PG_STOP.rx.lock().await.recv().await;
    });
}

pub async fn get_db() -> Pool {
    PG_CMD.tx.send(ContainerCommand::GetDb).unwrap();

    PG_POOLS
        .rx
        .lock()
        .await
        .recv()
        .await
        .expect("PG_POOLS closed while tests were running")
}

async fn handle_pg() {
    let docker = clients::Cli::default();
    let image = RunnableImage::from(Postgres::default()).with_tag("16.1");

    println!("Starting postgresql container");
    let postgres_instance = docker.run(image);
    let port = postgres_instance.get_host_port_ipv4(5432);

    println!("Opening management connection to the database");
    let management_pool = create_test_pool(port, "postgres").unwrap();

    println!("Waiting for tests");
    let mut rx = PG_CMD.rx.lock().await;
    let mut test_counter = 0u32;
    while let Some(ev) = rx.recv().await {
        match ev {
            ContainerCommand::GetDb => {
                test_counter += 1;
                let db_name = format!("test_{test_counter}");

                create_db(&management_pool, &db_name).await;
                let test_pool = create_test_pool(port, &db_name).unwrap();
                migrate_up(&test_pool).await;

                println!("created database {db_name} for a test");
                PG_POOLS.tx.send(test_pool).unwrap();
            }
            ContainerCommand::Stop => {
                postgres_instance.stop();
                PG_STOP.tx.send(()).unwrap();
                rx.close();
            }
        }
    }
}

async fn create_db(db_pool: &Pool, name: &str) {
    let db = db_pool.get().await.unwrap();

    // we can't use parametrized SQL for database/table names
    let query = format!("CREATE DATABASE {name}");

    db.execute(&query, &[]).await.unwrap();
}

async fn migrate_up(db_pool: &Pool) {
    let mut db = db_pool.get().await.unwrap();
    let client = &mut **db;

    embedded::migrations::runner()
        .run_async(client)
        .await
        .map_err(|e| {
            println!("error: {e}");
            e
        })
        .unwrap();
}

fn create_test_pool(port: u16, database: &str) -> Result<Pool, deadpool_postgres::CreatePoolError> {
    let mut config = deadpool_postgres::Config::new();
    config.host = Some("localhost".to_string());
    config.dbname = Some(database.to_string());
    config.user = Some("postgres".to_string());
    config.password = Some("postgres".to_string());
    config.port = Some(port);
    config.application_name = Some("tei_tests".to_string());

    let pool = config.create_pool(
        Some(deadpool_postgres::Runtime::Tokio1),
        tokio_postgres::NoTls,
    )?;

    Ok(pool)
}
