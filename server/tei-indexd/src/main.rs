mod indexer;
mod tags;

use color_eyre::eyre::Result;
use indexer::Indexer;
use tei_mq::StreamReader;
use tracing::{subscriber::set_global_default, Level};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().expect(".env file not found");

    let collector = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();

    set_global_default(collector)?;

    // TODO: Move everything avobe this line to separate crate

    let db_pool = tei_data::create_pool()?;
    let meili = tei_filter::create_client();
    let nats = tei_mq::create_client().await?;

    let stream = StreamReader::create(nats, "tei-indexer").await?;

    let indexer = Indexer::new(db_pool, stream, meili);
    indexer.run().await?;

    Ok(())
}
