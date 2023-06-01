use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{create_dir_all, File},
    os::unix::prelude::MetadataExt,
    path::PathBuf,
    sync::Arc,
};

use anyhow::{anyhow, Context, Ok};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use axum_client_ip::SecureClientIp;
use filecoin_proofs::{PaddedBytesAmount, PieceInfo, UnpaddedBytesAmount};
use fvm_shared::piece::PaddedPieceSize;
use log::{error, info};

use super::{Handler, ResponseError, ResultResponse};
use crate::{
    cmd::run::handler::fetch_sector::fetch,
    storage::{
        paths,
        sealer::storiface::{filetype::sector_name, AddPieceArgs},
    },
};

pub async fn add_piece(
    secure_ip: SecureClientIp,
    _header: HeaderMap,
    Path(_params): Path<HashMap<String, String>>,
    State(state): State<Arc<Handler>>,
    Json(input): Json<AddPieceArgs>,
) -> ResultResponse<Json<PieceInfo>> {
    add_piece_inner(secure_ip, _header, Path(_params), State(state), Json(input))
        .await
        .map_err(|e| ResponseError(e))
}

async fn add_piece_inner(
    secure_ip: SecureClientIp,
    _header: HeaderMap,
    Path(_params): Path<HashMap<String, String>>,
    State(state): State<Arc<Handler>>,
    Json(input): Json<AddPieceArgs>,
) -> anyhow::Result<Json<PieceInfo>> {
    info!(
        "sector:{} AddPiece {}  {}",
        input.sector.id.number, secure_ip.0, input.root
    );

    let _seal = chrono::Utc::now().time();
    let mut write = state.worker_info.write().await;
    write.sealing += 1;
    drop(write);

    /* TODO: defer
        defer func() {
        handler.winfo.Sealing--
    }()
     */
    let mut write = state.worker_info.write().await;
    write.sealing -= 1;
    drop(write);

    let sector = input.sector.id;
    let sector_name = sector_name(sector);

    let mut file_full_path = PathBuf::from(&input.root);
    if input.root != "" && file_full_path.starts_with("http:") {
        let root = PathBuf::from(&input.root);
        let final_name = root
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        create_dir_all(
            PathBuf::from(&state.worker_repo)
                .join(paths::local_ext::PATH_CAR)
                .join(sector_name.clone()),
        )?;

        file_full_path = PathBuf::from(&state.worker_repo.clone())
            .join(paths::local_ext::PATH_CAR)
            .join(&sector_name)
            .join(final_name);

        if let Err(e) = fetch(State(state.clone()), input.root, final_name.to_string()).await {
            error!("fetch error: {} {}", e, final_name);
        }
    }

    let mut file = File::open(file_full_path).context("open file error")?;

    let file_meta = file.metadata().context("read file metadata error")?;

    //TODO: NewInflator read and fill piece_size
    // 		data, err2 = shared.NewInflatorReader(reader, uint64(fi.Size()), args.PieceSize)

    let staged_file_dir = PathBuf::from(state.worker_repo.clone()).join("unsealed");

    if let Err(e) = create_dir_all(&staged_file_dir) {
        error!("create unsealed file dir {}", e);
    }

    let staged_file_path = staged_file_dir.join(sector_name);

    let staged_file = File::open(staged_file_path)?;

    // filecoin_proofs::add_piece(file, staged_file, UnpaddedBytesAmount(input.piece_size.0), input.piece_sizes);
    let ppi = filecoin_proofs::write_and_preprocess(
        file,
        staged_file,
        UnpaddedBytesAmount(input.piece_size.0),
    )
    .context("add piece")?;

    Ok(axum::Json(ppi.0))
}
