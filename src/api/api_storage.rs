use fvm_shared::sector::{RegisteredSealProof, SectorID};
use serde::{Deserialize, Serialize};

use crate::storage::sealer::storiface::storage::Commit1Out;

#[derive(PartialEq, Debug, Eq, Clone, Serialize, Deserialize)]
pub struct RemoteCommit2Params {
    pub sector: SectorID,
    pub proof_type: RegisteredSealProof,
    pub commit1_out: Commit1Out,
}
