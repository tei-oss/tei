use meilisearch_sdk::{Client, Index, TaskInfo};
use serde::{Deserialize, Serialize};
use std::convert::Into;
use tei_core::tag::Tag;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
struct IndexEntry {
    id: i64,
    group_id: i32,
    label: String,
    description: Option<String>,
    created_by: i32,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("meilisearch error")]
    Meilisearch(#[from] meilisearch_sdk::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<&Tag> for IndexEntry {
    fn from(value: &Tag) -> Self {
        Self {
            id: value.id.id,
            group_id: value.group_id.as_i32(),
            label: value.label.to_string(),
            description: value.description.clone().map(|e| e.to_string()),
            created_by: value.audit.created.user_id.as_i32(),
        }
    }
}

pub struct TagIndex {
    index: Index,
}

impl TagIndex {
    #[must_use]
    pub fn new(client: &Client) -> Self {
        let index = client.index("tags");

        Self { index }
    }

    pub async fn configure(&self) -> Result<()> {
        self.index
            .set_filterable_attributes(["group_id", "created_by"])
            .await?;

        self.index
            .set_searchable_attributes(["label", "description"])
            .await?;

        Ok(())
    }

    pub async fn add<'a, T: AsRef<[&'a Tag]>>(&self, tags: T) -> Result<TaskInfo> {
        let dtos: Vec<IndexEntry> = tags.as_ref().iter().map(|t| (*t).into()).collect();

        let task = self.index.add_documents(&dtos, Some("id")).await?;

        Ok(task)
    }
}
