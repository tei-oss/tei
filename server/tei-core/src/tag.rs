use derive_more::Display;

use crate::{auxiliary::Audit, group::GroupId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "{group_id}_{id}")]
pub struct TagId {
    pub group_id: GroupId,
    pub id: i64,
}

impl TagId {
    #[must_use]
    pub fn new(group_id: GroupId, id: i64) -> Self {
        Self { group_id, id }
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
