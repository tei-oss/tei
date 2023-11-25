use time::OffsetDateTime;

use crate::user::UserId;

#[derive(Debug, Clone)]
pub struct Audit {
    pub created: AuditAction,
    pub updated: Option<AuditAction>,
    pub version: i32,
}

impl Audit {
    #[must_use]
    pub fn created(id: &UserId) -> Self {
        Self {
            created: AuditAction::now(id.to_owned()),
            updated: None,
            version: 1,
        }
    }

    pub fn updated(&mut self, id: &UserId) -> &mut Self {
        self.updated = Some(AuditAction::now(id.to_owned()));
        self.version += 1;

        self
    }

    #[must_use]
    pub fn new(created: AuditAction, updated: Option<AuditAction>, version: i32) -> Self {
        Self {
            created,
            updated,
            version,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuditAction {
    pub user_id: UserId,
    pub timestamp: OffsetDateTime,
}

impl AuditAction {
    #[must_use]
    pub fn now(id: UserId) -> Self {
        Self {
            user_id: id,
            timestamp: OffsetDateTime::now_utc(),
        }
    }

    #[must_use]
    pub fn new(id: UserId, timestamp: OffsetDateTime) -> Self {
        Self {
            user_id: id,
            timestamp,
        }
    }
}
