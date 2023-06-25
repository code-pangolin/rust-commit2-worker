use std::sync::Arc;

use anyhow::Result;
use axum::extract::State;

use super::Handler;

pub async fn fetch(
    State(_state): State<Arc<Handler>>,
    _url: String,
    _outname: String,
) -> Result<()> {
    Ok(())
}
