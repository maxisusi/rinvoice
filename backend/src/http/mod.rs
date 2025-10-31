use anyhow::Ok;
use axum::{Extension, Router};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

mod customers;

use crate::config::Config;

#[derive(Clone)]
pub struct ApiCtx {
    db: Pool<Sqlite>,
    config: Config,
}

impl ApiCtx {
    fn new(db: Pool<Sqlite>, config: Config) -> Arc<Self> {
        Arc::new(ApiCtx { db, config })
    }
}

pub async fn serve(db: Pool<Sqlite>, config: Config) -> anyhow::Result<()> {
    let ctx = ApiCtx::new(db, config.clone());

    let app = api_router().layer(Extension(ctx));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn api_router() -> Router {
    // We can merge other controllers here
    customers::router()
}
