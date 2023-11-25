use deadpool_postgres::GenericClient;
use tei_core::tag::{Tag, TagId};
use tokio_postgres::Row;

use crate::{
    auxiliary::{self, DbConnection},
    extensions::*,
};

pub struct TagRepository {
    db: DbConnection,
}

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

impl TagRepository {
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, _tag: &Tag) {
        todo!();

        // let result = self.db.execute("insert into tags(group_id, label, color, description, icon, created_by, created_at, version) values($1, $2, $3, $4, $5, $6, $7, $8)", &[
        //     &tag.group_id.as_i32(),
        //     &tag.label,
        //     &tag.color,
        //     &tag.description,
        //     &tag.icon,
        //     &tag.audit.created.user_id.as_i32(),
        //     &tag.audit.created.timestamp,
        //     &tag.audit.version
        // ]).await.unwrap();
    }

    pub async fn get(&self, id: &TagId) -> Option<Tag> {
        println!("tag: {id}");

        let result = self
            .db
            .query(
                "select * from tags where group_id = $1 and id = $2",
                &[&id.group_id.as_i32(), &id.id],
            )
            .await
            .unwrap();

        Some(Tag::from_row(&result[0]))
    }
}
