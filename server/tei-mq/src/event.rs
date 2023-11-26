use crate::{
    nats_stream::Payload,
    tags::{self, TagEvent},
};

#[derive(Debug, Clone)]
pub enum Event {
    Tag(TagEvent),
    Unknown,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Tag(#[from] tags::Error),

    #[error("no event mappings are configured for table {0}")]
    UnknownTable(String),
}

pub type Result<T> = core::result::Result<T, Error>;

impl Event {
    pub(crate) fn from_payload(payload: Payload) -> Result<Event> {
        let ev = match payload.table.as_str() {
            "tags" => Event::Tag(TagEvent::try_from(payload)?),
            _ => return Err(Error::UnknownTable(payload.table)),
        };

        Ok(ev)
    }
}
