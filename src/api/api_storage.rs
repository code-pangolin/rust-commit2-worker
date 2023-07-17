use fvm_shared::sector::{RegisteredSealProof, SectorID};
use serde::{Deserialize, Serialize};

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
    async fn return_seal_commit2(
        &self,
        call_id: CallID,
        proof: Vec<u8>,
        err: Option<&CallError>,
    ) -> anyhow::Result<()>;
}
