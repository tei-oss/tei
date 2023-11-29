mod indexer;
mod tags;

use color_eyre::eyre::Result;
use indexer::Indexer;
use tei_mq::StreamReader;

#[tokio::main]
async fn main() -> Result<()> {
    tei_hosting::install()?;

    let db_pool = tei_data::create_pool()?;
    let meili = tei_filter::create_client();
    let nats = tei_mq::create_client().await?;

    let stream = StreamReader::create(nats, "tei-indexer").await?;

    let indexer = Indexer::new(db_pool, stream, meili);
    indexer.run().await?;

    Ok(())
}
