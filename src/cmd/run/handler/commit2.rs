use std::{collections::HashMap, fmt::format, sync::Arc};

use anyhow::{anyhow, Result};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{response::Parts, Extensions, HeaderMap, Request, StatusCode, Version},
    response::{IntoResponse, Response},
};
use axum_client_ip::SecureClientIp;
use fvm_shared::sector::RegisteredSealProof;

use super::{error::HandlerError, Handler};
use crate::{
    api::api_storage::RemoteCommit2Params, storage::sealer::storiface::storage::Commit1Out,
};

pub async fn seal_commit2(
    secure_ip: SecureClientIp,
    _header: HeaderMap,
    Path(_params): Path<HashMap<String, String>>,
    State(state): State<Arc<Handler>>,
    req: Request<Body>,
) -> Response<Body> {
    let (mut parts, _) = StatusCode::OK.into_response().into_parts();
    parts.status = StatusCode::OK;

    let body = match seal_commit2_inner(State(state), req).await {
        Ok(body) => body,
        Err(e) => {
            parts.status = e.status();
            Body::from(format!("{}", e))
        }
    };

    Response::from_parts(parts, body)
}

async fn seal_commit2_inner(
    State(state): State<Arc<Handler>>,
    req: Request<Body>,
) -> anyhow::Result<Body, HandlerError> {
    let (req_parts, body) = req.into_parts();

    let input = parse_data(
        hyper::body::to_bytes(body)
            .await
            .map_err(|e| HandlerError::InternalServerError(anyhow!("read body {}", e)))?
            .into(),
    )
    .map_err(|e| HandlerError::InternalServerError(anyhow!("parse req {}", e)))?;

    Err(HandlerError::InternalServerError(anyhow!("TODO!")))
}

// https://github.com/filecoin-project/lotus/blob/6e7dc9532abdb3171427347710df4c860f1957a2/storage/sealer/ffiwrapper/sealer_cgo.go#L895

fn parse_data(data: Vec<u8>) -> Result<RemoteCommit2Params> {
    let ppp = RemoteCommit2Params {
        sector: fvm_shared::sector::SectorID {
            miner: 141324,
            number: 3412314,
        },
        proof_type: RegisteredSealProof::StackedDRG2KiBV1P1,
        commit1_out: Commit1Out(vec![2, 3, 4]),
    };

    serde_json::to_string_pretty(&ppp)?;

    println!("{}", serde_json::to_string_pretty(&ppp)?);

    //TODO: gzip decompress
    let param: RemoteCommit2Params = serde_json::from_slice(&data)?;
    return Ok(param);
}
