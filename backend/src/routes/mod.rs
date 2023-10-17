#[axum::debug_handler]
#[tracing::instrument]
pub async fn hello() -> &'static str {
    tracing::warn!("WARNING: YOU ARE BEING GREETED");

    "Hello World!"
}
