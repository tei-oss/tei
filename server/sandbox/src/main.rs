use std::time::Duration;

use deadpool_postgres::Config;

use futures::StreamExt;
use tei_mq::StreamReader;
use tokio::time::sleep;
use tracing::{subscriber::set_global_default, Level};

#[tokio::main]
async fn main() {
    let collector = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();

    set_global_default(collector).unwrap();

    let mut config = Config::new();
    config.host = Some("localhost".to_owned());
    config.dbname = Some("localc".to_owned());
    config.user = Some("localc".to_owned());
    config.password = Some("localc".to_owned());
    config.port = Some(15432);
    config.application_name = Some("sandbox".to_owned());

    // let pool = config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    // let dbc = pool
    //     .get()
    //     .await
    //     .expect("Could not get DB connection from pool");

    // let group_id = 1.into();
    // let user_id = 1.into();

    // for _ in 0..10 {
    //     let tag = Tag {
    //         id: TagId::new(group_id, 0),
    //         label: format!("test-{}", Uuid::new_v4()).into_boxed_str(),
    //         description: Some(format!("description-{}", Uuid::new_v4()).into_boxed_str()),
    //         color: None,
    //         icon: None,
    //         group_id,
    //         audit: Audit::created(&user_id),
    //     };

    //     TagsDb::insert(&dbc, &tag).await.unwrap();
    // }

    let nats = async_nats::connect("nats://localhost:14222").await.unwrap();
    let mq = StreamReader::create(nats, "debug-tap").await.unwrap();

    let mut messages = mq.consume().await.unwrap().ready_chunks(6);
    while let Some(m) = messages.next().await {
        let batch: Result<Vec<_>, _> = m.into_iter().collect();
        let batch = batch.unwrap();

        for ev in batch.iter().map(|m| &m.event) {
            println!("received event: {ev:?}");
        }

        for ev in batch {
            ev.ack().await.unwrap();
        }

        sleep(Duration::from_millis(1000)).await;
    }

    // let api_key: Option<String> = None;
    // let meili = Client::new("http://localhost:17700", api_key);

    // let tag_index = TagIndex::new(&meili);

    // tag_index.configure().await.unwrap();
    // tag_index.add([&tag]).await.unwrap();
}
