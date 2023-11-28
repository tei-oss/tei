use serde::Serialize;
use tei_core::auxiliary::Audit;
use time::OffsetDateTime;

#[derive(Debug, Serialize)]
pub struct AuditDto {
    pub created_by: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub updated_by: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
    pub version: i32,
}

impl From<Audit> for AuditDto {
    fn from(value: Audit) -> Self {
        Self {
            created_by: value.created.user_id.to_uid(),
            created_at: value.created.timestamp,
            updated_by: value.updated.as_ref().map(|v| v.user_id.to_uid()),
            updated_at: value.updated.as_ref().map(|v| v.timestamp),
            version: value.version,
        }
    }
}
