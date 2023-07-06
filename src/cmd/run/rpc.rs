use std::{
    net::{SocketAddr, TcpListener},
    sync::Arc,
};

use anyhow::anyhow;
use jsonrpc_core_client::TypedClient;
use tokio::sync::mpsc::Sender;
use version::version;

use crate::{
    rpc::start_rpc,
    rpc_api::data_types::RPCState,
    storage::{
        ipfs::ds_rocksdb::RocksDS,
        sealer::worker_local::{LocalWorker, ManagerReturn},
    },
};

pub async fn start_rpc_server(
    shutdown_send: Sender<()>,
    addr: &SocketAddr,
    worker: Box<LocalWorker<RocksDS, ManagerReturn>>,
) -> anyhow::Result<()> {
    let _infoenv = std::env::var("MINER_API_INFO").map_err(|_| anyhow!(""));
    let nodeapi: TypedClient = jsonrpc_core_client::transports::http::connect("url")
        .await
        .map_err(|e| anyhow!("{}", e))?;
    let state = Arc::new(RPCState { nodeapi, worker });

    println!("version::::::{}", version!());

    start_rpc(state, TcpListener::bind(addr)?, version!(), shutdown_send).await
}
