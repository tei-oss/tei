use deadpool_postgres::{tokio_postgres::NoTls, Config, Runtime};
use meilisearch_sdk::Client;
use tei_core::{
    auxiliary::Audit,
    tag::{Tag, TagId},
};

use tei_filter::tags::TagIndex;
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

    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let _dbc = pool
        .get()
        .await
        .expect("Could not get DB connection from pool");

    let group_id = 1.into();
    let user_id = 1.into();

    let tag = Tag {
        id: TagId::new(group_id, 3),
        label: "rust".into(),
        description: Some("fearless concurrency".into()),
        color: None,
        icon: None,
        group_id,
        audit: Audit::created(&user_id),
    };

    // TagsDb::insert(&dbc, &tag).await.unwrap();

    let api_key: Option<String> = None;
    let meili = Client::new("http://localhost:17700", api_key);

    let tag_index = TagIndex::new(&meili);

    tag_index.configure().await.unwrap();
    tag_index.add([&tag]).await.unwrap();
}
