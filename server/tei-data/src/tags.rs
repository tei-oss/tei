use deadpool_postgres::GenericClient;
use tei_core::{
    group::GroupId,
    tag::{Tag, TagId},
};
use thiserror::Error;
use tokio_postgres::Row;

use crate::{
    auxiliary,
    extensions::{FromRow, RowEx},
};

pub struct TagsDb {}

#[derive(Error, Debug)]
pub enum TagsError {
    #[error("postgres error")]
    DataStore(#[from] tokio_postgres::Error),
}

pub type Result<T> = core::result::Result<T, TagsError>;

impl FromRow for Tag {
    fn from_row(row: &Row) -> Self {
        let audit = auxiliary::create_audit(row);

        let group_id: i32 = row.get("group_id");
        let id: i64 = row.get("id");

        Self {
            id: TagId::new(group_id.into(), id),
            label: row.get_boxed_str("label"),
            color: row.get_opt_boxed_str("color"),
            description: row.get_opt_boxed_str("description"),
            icon: row.get_opt_boxed_str("icon"),
            group_id: group_id.into(),
            audit,
        }
    }
}

impl TagsDb {
    pub async fn insert<T: GenericClient>(db: &T, tag: &Tag) -> Result<()> {
        let result = db.execute("insert into tags(group_id, label, color, description, icon, created_by, created_at, version) values($1, $2, $3, $4, $5, $6, $7, $8)", &[
            &tag.group_id.as_i32(),
            &tag.label,
            &tag.color,
            &tag.description,
            &tag.icon,
            &tag.audit.created.user_id.as_i32(),
            &tag.audit.created.timestamp,
            &tag.audit.version
        ]).await?;

        assert_eq!(
            result, 1,
            "Insert must either fail due to constrain violation or return =1 row"
        );

        Ok(())
    }

    pub async fn get_by_id<T: GenericClient>(db: &T, id: &TagId) -> Result<Option<Tag>> {
        let rows = db
            .query(
                "select * from tags where group_id = $1 and id = $2",
                &[&id.group_id.as_i32(), &id.id],
            )
            .await?;

        assert!(
            rows.len() <= 1,
            "PK lookup must not return more than one row"
        );

        Ok(rows.first().map(Tag::from_row))
    }

    pub async fn get_by_label<T: GenericClient>(
        db: &T,
        group_id: &GroupId,
        label: &str,
    ) -> Result<Option<Tag>> {
        let rows = db
            .query(
                "select * from tags where group_id = $1 and label = $2",
                &[&group_id.as_i32(), &label],
            )
            .await?;

        assert!(
            rows.len() <= 1,
            "Label lookup must not return more than one row"
        );

        Ok(rows.first().map(Tag::from_row))
    }
}
