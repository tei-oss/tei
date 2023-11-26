use crate::nats_stream::Action;
use serde::Deserialize;
use tei_core::tag::TagId;

use crate::nats_stream::Payload;

#[derive(Debug, Clone)]
pub enum TagEvent {
    Created(TagId),
    Deleted(TagId),
    Updated(TagId),
    Unknown,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected table {0} for tag")]
    WrongTable(String),

    #[error(transparent)]
    Deserialization(#[from] serde_json::error::Error),
}

#[derive(Debug, Deserialize)]
struct TagIdDto {
    group_id: i32,
    id: i64,
}

impl From<TagIdDto> for TagId {
    fn from(value: TagIdDto) -> Self {
        TagId::new(value.group_id.into(), value.id)
    }
}

impl TryFrom<Payload> for TagEvent {
    type Error = Error;

    fn try_from(value: Payload) -> core::result::Result<Self, Self::Error> {
        if value.table.ne("tags") {
            return Err(Error::WrongTable(value.table));
        }

        let id: TagIdDto = serde_json::from_value(value.data)?;
        let id = id.into();

        let event = match value.action {
            Action::Insert => TagEvent::Created(id),
            Action::Update => TagEvent::Updated(id),
            Action::Delete => TagEvent::Deleted(id),
            Action::Unknown => TagEvent::Unknown,
        };

        Ok(event)
    }
}
