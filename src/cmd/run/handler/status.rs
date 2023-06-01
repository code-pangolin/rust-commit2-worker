use std::{collections::HashMap, sync::Arc};

use axum::extract::{Path, State};

use super::Handler;

pub async fn status(
    Path(_params): Path<HashMap<String, String>>,
    State(db): State<Arc<Handler>>,
) -> String {
    let mut addr = db.ext_commit_c2_addr.write().await;
    addr.push('2');
    addr.to_string()
}
