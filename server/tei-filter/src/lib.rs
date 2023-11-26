use meilisearch_sdk::Client;
use tei_core::env;

pub mod tags;

#[must_use]
pub fn create_client() -> Client {
    Client::new(
        env::get("MEILI_URL").expect("MEILI_URL is mandatory"),
        env::get("name"),
    )
}
