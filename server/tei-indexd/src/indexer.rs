use std::sync::Arc;

use color_eyre::eyre;

use color_eyre::Result;
use deadpool_postgres::Pool;
use futures::StreamExt;

use tei_mq::Message;
use tei_mq::StreamReader;
use tracing::info;

use crate::tags::TagEvents;
pub struct Indexer {
    deps: Arc<Deps>,
}

pub(crate) struct Deps {
    pub pool: Pool,
    pub stream_reader: StreamReader,
    pub meili: meilisearch_sdk::Client,
}

impl Indexer {
    pub fn new(db: Pool, stream: StreamReader, index: meilisearch_sdk::Client) -> Self {
        let deps = Deps {
            pool: db,
            stream_reader: stream,
            meili: index,
        };

        Self {
            deps: Arc::new(deps),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let mut stream = self.deps.stream_reader.consume().await?.ready_chunks(100);

        while let Some(messages) = stream.next().await {
            let messages: Result<Vec<_>, _> = messages.into_iter().collect();
            let messages = messages?;

            info!("handling {} events", messages.len());
            let mut tag_events = TagEvents::new(self.deps.clone());

            for msg in &messages {
                match &msg.event {
                    tei_mq::Event::Tag(tag) => tag_events.add(tag),
                    tei_mq::Event::Unknown => {}
                }
            }

            tag_events.handle().await?;

            ack_all(&messages).await?;
        }

        todo!();
    }
}

async fn ack_all(messages: &Vec<Message>) -> Result<()> {
    for msg in messages {
        msg.ack().await.map_err(|e| eyre::eyre!("{e:?}"))?;
    }

    Ok(())
}
