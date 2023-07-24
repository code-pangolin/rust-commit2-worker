use std::{fmt::Display, sync::Arc};

use async_trait::async_trait;
use fvm_shared::sector::{RegisteredSealProof, SectorID};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::storage::sealer::storiface::{
    storage::Commit1Out,
    worker::{CallError, CallID},
};

#[derive(PartialEq, Debug, Eq, Clone, Serialize, Deserialize)]
pub struct RemoteCommit2Params {
    pub sector: SectorID,
    pub proof_type: RegisteredSealProof,
    pub commit1_out: Commit1Out,
}

pub trait StorageMiner: WorkerReturn {
    // fn new(addr: String, token: Option<String>) -> Self;
}

pub trait WorkerReturn: Sized {
    fn return_seal_commit2(
        &self,
        call_id: CallID,
        proof: Vec<u8>,
        err: Option<&CallError>,
    ) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct StorageMinerError {
    pub code: i64,
    pub message: String,
}

unsafe impl Send for StorageMinerError {}

impl std::error::Error for StorageMinerError {}

impl Display for StorageMinerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("code {}, msg: {}", self.code, self.message))
    }
}
