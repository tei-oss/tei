use tei_core::auxiliary::{Audit, AuditAction};
use time::OffsetDateTime;

pub fn create_audit(row: &tokio_postgres::Row) -> Audit {
    let created_by: i32 = row.get("created_by");
    let created_at: OffsetDateTime = row.get("created_at");
    let created = AuditAction::new(created_by.into(), created_at);

    let updated_by: Option<i32> = row.get("updated_by");
    let updated_at: Option<OffsetDateTime> = row.get("updated_at");
    let updated = match (updated_by, updated_at) {
        (Some(updated_by), Some(updated_at)) => {
            AuditAction::new(updated_by.into(), updated_at).into()
        }
        _ => None,
    };

    let version = row.get("version");

    Audit::new(created, updated, version)
}
