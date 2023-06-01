use std::sync::Arc;

use anyhow::Result;
use axum::extract::State;

use super::Handler;

pub async fn fetch(State(state): State<Arc<Handler>>, url: String, outname: String) -> Result<()> {
    Ok(())
}
