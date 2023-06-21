// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

mod auth_api;
mod commit2_api;
mod common_api;
mod rpc_http_handler;
mod rpc_util;
mod rpc_ws_handler;

use std::{net::TcpListener, sync::Arc};

use axum::routing::{get, post};
use jsonrpc_v2::{Data, Error as JSONRPCError, Server};
use log::info;
use tokio::sync::mpsc::Sender;

use crate::{
    rpc::{
        commit2_api::seal_commit2,
        common_api::{shutdown, version},
        rpc_http_handler::rpc_http_handler,
        rpc_ws_handler::rpc_ws_handler,
    },
    rpc_api::{auth_api::*, commit2_api::SEAL_COMMIT2, common_api::*, data_types::RPCState},
};

pub type RpcResult<T> = Result<T, JSONRPCError>;

pub async fn start_rpc(
    state: Arc<RPCState>,
    rpc_endpoint: TcpListener,
    forest_version: &'static str,
    shutdown_send: Sender<()>,
) -> anyhow::Result<()> {
    use auth_api::*;

    let block_delay = 0;
    let rpc_server = Arc::new(
        Server::new()
            .with_data(Data(state))
            // Auth API
            .with_method(AUTH_NEW, auth_new)
            .with_method(AUTH_VERIFY, auth_verify)

            // Common API
            .with_method(VERSION, move || version(block_delay, forest_version))
            .with_method(SHUTDOWN, move || shutdown(shutdown_send.clone()))

            // Commit2 API
            .with_method(SEAL_COMMIT2, seal_commit2)

            .finish_unwrapped(),
    );

    let app = axum::Router::new()
        .route("/rpc/v0", get(rpc_ws_handler))
        .route("/rpc/v0", post(rpc_http_handler))
        .with_state(rpc_server);

    info!("Ready for RPC connections");
    let server = axum::Server::from_tcp(rpc_endpoint)?.serve(app.into_make_service());
    server.await?;

    info!("Stopped accepting RPC connections");

    Ok(())
}
