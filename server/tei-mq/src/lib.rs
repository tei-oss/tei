mod event;
mod nats_stream;
pub mod tags;

pub use event::*;
pub use nats_stream::{Error, Message, MessageError, StreamReader};
use tei_core::env;

pub async fn create_client() -> core::result::Result<async_nats::Client, async_nats::ConnectError> {
    let url = env::get("NATS_URL").expect("NATS_URL is mandatory");
    async_nats::connect(url).await
}
