use async_nats::{
    jetstream::{
        self,
        consumer::{pull, Consumer, StreamError},
        context::GetStreamError,
        stream::ConsumerError,
    },
    Client,
};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::event::{self, Event};

#[derive(Debug, Clone)]
pub struct Message {
    pub event: Event,
    message: jetstream::Message,
}

impl Message {
    fn new(event: Event, message: jetstream::Message) -> Self {
        Self { event, message }
    }

    pub async fn ack(&self) -> Result<(), async_nats::Error> {
        self.message.ack().await?;

        Ok(())
    }

    pub async fn nack(&self) -> Result<(), async_nats::Error> {
        self.message.ack_with(jetstream::AckKind::Nak(None)).await?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    pub id: Uuid,
    pub schema: String,
    pub table: String,
    pub action: Action,
    pub data: Value,
    #[serde(rename = "dataOld")]
    pub data_old: Value,
    #[serde(rename = "commitTime")]
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
}

#[derive(Deserialize, Serialize, Clone, Debug, Copy)]
pub enum Action {
    #[serde(rename = "INSERT")]
    Insert,
    #[serde(rename = "UPDATE")]
    Update,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    GetStream(#[from] GetStreamError),
    #[error(transparent)]
    Consumer(#[from] ConsumerError),
    #[error(transparent)]
    Stream(#[from] StreamError),

    #[error("failed to extact message")]
    MessageExtract(#[from] async_nats::error::Error<pull::MessagesErrorKind>),
    #[error(transparent)]
    Nats(#[from] async_nats::Error),
}

#[derive(Debug, Error)]
pub enum MessageError {
    #[error(transparent)]
    JsonDeserialize(#[from] serde_json::Error),

    #[error(transparent)]
    BadPayload(#[from] event::Error),

    #[error(transparent)]
    Nats(#[from] async_nats::error::Error<pull::MessagesErrorKind>),
}

pub struct StreamReader {
    consumer: Consumer<pull::Config>,
}

impl StreamReader {
    pub async fn create(client: Client, name: &str) -> Result<Self, Error> {
        let js = jetstream::new(client);

        let consumer = js
            .get_stream("walli")
            .await?
            .get_or_create_consumer(
                name,
                pull::Config {
                    durable_name: Some(name.to_string()),
                    ..Default::default()
                },
            )
            .await?;

        Ok(Self { consumer })
    }

    pub async fn consume(
        &self,
    ) -> Result<impl Stream<Item = Result<Message, MessageError>>, Error> {
        let event_stream = self.consumer.messages().await?.map(convert_msg);
        Ok(event_stream)
    }
}

fn convert_msg(
    msg: core::result::Result<
        jetstream::Message,
        async_nats::error::Error<pull::MessagesErrorKind>,
    >,
) -> Result<Message, MessageError> {
    let msg = msg?;

    let payload: Payload = serde_json::de::from_slice(&msg.payload)?;
    let event = Event::from_payload(payload)?;
    let message = Message::new(event, msg);

    Ok(message)
}
