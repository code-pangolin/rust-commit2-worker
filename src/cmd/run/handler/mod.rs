mod add_piece;
pub mod fetch_sector;
mod status;

use std::{collections::HashMap, sync::Arc, time};

use axum::{
    extract::Path,
    http::HeaderMap,
    routing::{delete, get, post, put},
    Router,
};
use axum_client_ip::{SecureClientIp, SecureClientIpSource};
use log::info;
use tokio::{sync, time::sleep};

use crate::storage::sealer::worker::WorkerInfo;

#[derive(Debug, Default)]
pub struct Handler {
    pub(crate) worker_repo: String,
    pub(crate) worker_info: sync::RwLock<WorkerInfo>,
    // seal_lock: sync::Mutex<u32>,
    pub(crate) ext_commit_c2_addr: sync::RwLock<String>,
    pub(crate) enalbe_ext_commit2: sync::RwLock<bool>,

    pub(crate) sector_map: sync::RwLock<HashMap<String, String>>,
    pub(crate) save_params: sync::RwLock<bool>,

    pub(crate) bneedlastpiece: sync::RwLock<bool>,
    pub(crate) bneddtreedfile: sync::RwLock<bool>,
}

type ResultResponse<T> = std::result::Result<T, ResponseError>;
pub struct ResponseError(anyhow::Error);

impl From<anyhow::Error> for ResponseError {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}

impl axum::response::IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        let status_code = if let Some(error) = self.0.downcast_ref::<ApiError>() {
            match error {
                ApiError::NotFound => axum::http::StatusCode::NOT_FOUND,
                ApiError::BadRequest => axum::http::StatusCode::BAD_REQUEST,
            }
        } else {
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        };
        (status_code, self.0.to_string()).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
enum ApiError {
    #[error("not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
}

impl Handler {
    pub fn default() -> Self {
        Handler {
            ..Default::default()
        }
    }
}

pub async fn log(handler: Arc<Handler>) {
    loop {
        sleep(time::Duration::from_secs(300)).await;
        let workerinfo = handler.worker_info.read().await;
        if workerinfo.worker_type != "U1C2" {
            info!(
                "sealSector {},{},{}",
                workerinfo.worker_type, workerinfo.hostname, workerinfo.sealing
            );
        }
    }
}

pub fn router(state: Arc<Handler>) -> Router {
    // let app_state = Arc::new(self);
    Router::new()
        .route("/remote/seal/addpiece", post(add_piece::add_piece))
        .route("/remote/seal/precommit1", post(root))
        .route("/remote/seal/precommit2", post(root))
        .route("/remote/seal/commit1", post(root))
        .route("/remote/seal/commit2", post(root))
        .route("/remote/seal/push", post(root))
        .route("/remote/seal/fetch", post(root))
        .route("/remote/seal/status", get(status::status))
        .route("/remote/seal/updateStatus", post(root))
        .route("/remote/:type/:id/:fix", get(root))
        .route("/remote/:type/:id/:fix", put(root))
        .route("/remote/:type/:id", get(path_param_test))
        .route("/remote/:type/:id", delete(root))
        .route("/remote/snapdeal/replica", get(root))
        .route("/remote/snapdeal/replica", post(root))
        .route("/remote/snapdeal/provereplica1", get(root))
        .route("/remote/snapdeal/provereplica1", post(root))
        .route("/remote/snapdeal/provereplica2", get(root))
        .route("/remote/snapdeal/provereplica2", post(root))
        .route("/remote/snapdeal/generatesectorkey", get(root))
        .route("/remote/snapdeal/generatesectorkey", post(root))
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .with_state(state)
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn path_param_test(
    secure_ip: SecureClientIp,
    _header: HeaderMap,
    Path(params): Path<HashMap<String, String>>,
) -> String {
    println!("{:?}", secure_ip);
    format!(
        "{{type: {}, id: {}}}",
        params.get("type").unwrap().to_string(),
        params.get("id").unwrap().to_string()
    )
}
