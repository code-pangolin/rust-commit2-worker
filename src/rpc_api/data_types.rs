// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use std::sync::Arc;

use jsonrpc_v2::{MapRouter as JsonRpcMapRouter, Server as JsonRpcServer};
use serde::{Deserialize, Serialize};

use crate::{api::client::StorageMinerRpcClient, storage::ipfs::ds_rocksdb::RocksDS};

/// This is where you store persistent data, or at least access to stateful
/// data.
pub struct RPCState {
    pub nodeapi: jsonrpc_core_client::TypedClient,
    pub worker:
        Box<crate::storage::sealer::worker_local::LocalWorker<RocksDS, StorageMinerRpcClient>>,
}

pub type JsonRpcServerState = Arc<JsonRpcServer<JsonRpcMapRouter>>;

/// Represents the current version of the API.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct APIVersion {
    pub version: String,
    pub api_version: Version,
    pub block_delay: u64,
}

/// Integer based value on version information. Highest order bits for Major,
/// Mid order for Minor and lowest for Patch.
#[derive(Serialize, Deserialize)]
pub struct Version(u32);

impl Version {
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self((major as u32) << 16 | (minor as u32) << 8 | (patch as u32))
    }
}
