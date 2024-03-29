use derive_more::Display;

use crate::{auxiliary::Audit, group::GroupId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "{group_id}_{id}")]
pub struct TagId {
    group_id: GroupId,
    id: i64,
}

impl TagId {
    #[must_use]
    pub fn new(group_id: GroupId, id: i64) -> Self {
        Self { group_id, id }
    }

    #[must_use]
    pub fn group_id(&self) -> GroupId {
        self.group_id
    }

    #[must_use]
    pub fn as_i64(&self) -> i64 {
        self.id
    }

    #[must_use]
    pub fn as_uid(&self) -> String {
        // TODO: Base64URL
        format!("{}_{}", self.group_id.as_i32(), self.id)
    }
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: TagId,
    pub label: Box<str>,
    pub color: Option<Box<str>>,
    pub description: Option<Box<str>>,
    pub icon: Option<Box<str>>,
    pub group_id: GroupId,
    pub audit: Audit,
}
