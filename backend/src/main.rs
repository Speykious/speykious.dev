use std::net::SocketAddr;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_tree::HierarchicalLayer;

use errors::ReportError;

pub mod errors;
pub mod routes;

#[axum::debug_handler]
#[tracing::instrument(ret, err)]
pub async fn failing_handler() -> Result<String, ReportError> {
    Err(color_eyre::eyre::eyre!("failing_handler is failing!"))?;

    Ok("this message should not happen?".to_string())
}

pub fn setup_tracing() {
    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(
            HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .with(ErrorLayer::default())
        .init();
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    setup_tracing();

    let router = Router::new()
        .route("/", get(routes::hello))
        .route("/error", get(failing_handler))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4444));

    tracing::info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
