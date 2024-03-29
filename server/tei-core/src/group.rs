use derive_more::Display;

use crate::{auxiliary::Audit, tag::TagId, user::UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct GroupId {
    id: i32,
}

impl GroupId {
    #[must_use]
    pub fn as_i32(self) -> i32 {
        self.id
    }

    #[must_use]
    pub fn as_uid(&self) -> String {
        // TODO: Base64URL
        self.id.to_string()
    }
}

impl From<i32> for GroupId {
    fn from(value: i32) -> Self {
        Self { id: value }
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    pub id: GroupId,
    pub title: Box<str>,
    pub tags: Box<[TagId]>,
    pub audit: Audit,
}

#[derive(Debug, Clone)]
pub struct GroupMember {
    pub user_id: UserId,
    pub role: Role,
    pub audit: Audit,
}

#[derive(Debug, Clone, Copy)]
pub enum Role {
    Viewer,
    Uploader,
    Admin,
    Owner,
}
