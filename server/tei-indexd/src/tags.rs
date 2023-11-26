use std::{collections::HashSet, sync::Arc};

use itertools::Itertools;
use tei_core::tag::{Tag, TagId};
use tei_data::tags::TagsDb;
use tei_filter::tags::TagIndex;
use tei_mq::tags::TagEvent;
use tracing::warn;

use crate::indexer::Deps;
use color_eyre::Result;

pub struct TagEvents {
    changes: HashSet<TagId>,
    deletes: HashSet<TagId>,
    deps: Arc<Deps>,
}

impl TagEvents {
    pub fn new(deps: Arc<Deps>) -> Self {
        Self {
            deps,
            changes: HashSet::default(),
            deletes: HashSet::default(),
        }
    }

    fn changed(&mut self, id: TagId) {
        self.deletes.remove(&id);
        self.changes.insert(id);
    }

    fn deleted(&mut self, id: TagId) {
        self.changes.remove(&id);
        self.deletes.insert(id);
    }

    pub fn add(&mut self, event: &TagEvent) {
        match event {
            TagEvent::Deleted(id) => self.deleted(*id),
            TagEvent::Updated(id) | TagEvent::Created(id) => self.changed(*id),
            TagEvent::Unknown => warn!("skipping unknown tag event"),
        }
    }

    pub async fn handle(&self) -> Result<()> {
        self.index_up().await?;
        self.index_down().await?;

        Ok(())
    }

    async fn index_up(&self) -> Result<()> {
        if self.changes.is_empty() {
            return Ok(());
        }

        let tags = self.load_tags().await?;
        TagIndex::new(&self.deps.meili).add(tags).await?;

        Ok(())
    }

    // TODO: Rewrite using streams and buffer_unordered
    async fn load_tags(&self) -> Result<Vec<Tag>> {
        let db = self.deps.pool.get().await?;
        let mut tags = vec![];

        for id in &self.changes {
            if let Some(tag) = TagsDb::get_by_id(&db, id).await? {
                tags.push(tag);
            }
        }

        Ok(tags)
    }

    async fn index_down(&self) -> Result<()> {
        if self.deletes.is_empty() {
            return Ok(());
        }

        let ids = self.deletes.iter().copied().collect_vec();
        TagIndex::new(&self.deps.meili).remove(ids).await?;

        Ok(())
    }
}
