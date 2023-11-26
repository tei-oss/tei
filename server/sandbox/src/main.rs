use tracing::{subscriber::set_global_default, Level};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");

    let collector = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();

    set_global_default(collector).unwrap();

    // let pool = tei_data::create_pool().unwrap();
    // let nats = tei_mq::create_client().await.unwrap();
    // let meili = tei_filter::create_client();
}
