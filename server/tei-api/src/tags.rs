use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::convert::Into;
use tei_core::tag::Tag;
use tei_data::tags::TagsDb;

use crate::{common::AuditDto, errors::ApiError, Deps};

pub fn create_router() -> Router<Deps> {
    Router::new()
        .route("/", get(root))
        .route("/:label", get(by_label))
}

async fn root() -> Result<(), ApiError> {
    todo!()
}

async fn by_label(
    Path((group_id, label)): Path<(i32, String)>,
    State(deps): State<Deps>,
) -> Result<Json<TagDto>, ApiError> {
    let db = deps.db.get().await?;

    let tag = TagsDb::get_by_label(&db, &group_id.into(), &label)
        .await?
        .ok_or_else(|| ApiError::not_found(label))?;

    Ok(Json(tag.into()))
}

#[derive(Debug, Serialize)]
pub struct TagDto {
    pub id: String,
    pub label: String,
    pub color: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub group_id: String,
    #[serde(flatten)]
    pub audit: AuditDto,
}

impl From<Tag> for TagDto {
    fn from(value: Tag) -> Self {
        Self {
            id: value.id.as_uid(),
            label: value.label.into(),
            color: value.color.map(Into::into),
            description: value.description.map(Into::into),
            icon: value.icon.map(Into::into),
            group_id: value.group_id.as_uid(),
            audit: value.audit.into(),
        }
    }
}
