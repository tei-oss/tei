pub mod common;
pub mod errors;
pub mod tags;

use axum::{response::Json, routing::get, Router};
use color_eyre::eyre::Result;
use deadpool_postgres::Pool;
use meilisearch_sdk::Client;
use serde::Serialize;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tei_hosting::install()?;

    let router = build_router()?;
    let endpoint = tei_core::env::get("TEI_API_BIND").unwrap_or("0.0.0.0:3000".to_owned());
    let listener = tokio::net::TcpListener::bind(&endpoint).await?;

    info!("tei api is listening on {endpoint} (configured using TEI_API_BIND)");
    axum::serve(listener, router).await?;

    Ok(())
}

fn build_router() -> Result<Router> {
    let db_pool = tei_data::create_pool()?;
    let meili = tei_filter::create_client();

    let deps = Deps { db: db_pool, meili };

    let router = Router::new()
        .nest("/groups/:group/tags", tags::create_router())
        .route("/", get(get_readme))
        .with_state(deps)
        .layer(TraceLayer::new_for_http());

    Ok(router)
}

#[derive(Clone)]
pub struct Deps {
    pub db: Pool,
    pub meili: Client,
}

async fn get_readme() -> Json<HomeDto> {
    Json(HomeDto::default())
}

#[derive(Debug, Serialize)]
pub struct HomeDto {
    pub message: &'static str,
    pub home: &'static str,
}

impl Default for HomeDto {
    fn default() -> Self {
        Self {
            message: "This is tei http api. Visit our repository for more information.",
            home: "https://github.com/tei-oss/tei",
        }
    }
}
