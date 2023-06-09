use std::{collections::HashMap, fmt::format, sync::Arc};

use anyhow::{anyhow, Result};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{response::Parts, Extensions, HeaderMap, Request, StatusCode, Version},
    response::{IntoResponse, Response},
};
use axum_client_ip::SecureClientIp;
use fvm_shared::{address::Address, sector::RegisteredSealProof};

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
            .map_err(|e| HandlerError::BadRequest(anyhow!("read body {}", e)))?
            .into(),
    )
    .map_err(|e| HandlerError::BadRequest(anyhow!("parse req {}", e)))?;

    println!("{:?}", &input.commit1_out);

    // https://github.com/filecoin-project/filecoin-ffi/blob/c149dfa67e6ea3db8c203023580a5052e724f99a/rust/src/proofs/api.rs#L217

    let scp1o = serde_json::from_slice(&input.commit1_out.0)
        .map_err(|e| HandlerError::BadRequest(anyhow!("parse commit1_out {}", e)))?;

    let maddr = Address::new_id(input.sector.miner);

    let mut prover_id: [u8; 32] = [0; 32];
    let payload = maddr.payload().to_bytes();
    prover_id[..payload.len()].copy_from_slice(&payload);

    let result = filecoin_proofs_api::seal::seal_commit_phase2(
        scp1o,
        prover_id,
        filecoin_proofs_api::SectorId::from(input.sector.number),
    )
    .map_err(|e| HandlerError::BadRequest(anyhow!("seal_commit_phase2 {}", e)))?;

    Ok::<Body, HandlerError>(Body::from(result.proof.to_vec()));

    Err(HandlerError::InternalServerError(anyhow!("TODO!")))
}

// https://github.com/filecoin-project/lotus/blob/6e7dc9532abdb3171427347710df4c860f1957a2/storage/sealer/ffiwrapper/sealer_cgo.go#L895

fn parse_data(data: Vec<u8>) -> Result<RemoteCommit2Params> {
    //TODO: gzip decompress
    let param: RemoteCommit2Params = serde_json::from_slice(&data)?;

    return Ok(param);
}
