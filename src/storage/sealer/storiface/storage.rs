use fvm_shared::sector::{RegisteredSealProof, SectorID};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Eq, Clone, Serialize, Deserialize)]
pub struct SectorRef {
    pub id: SectorID,
    pub proof_type: RegisteredSealProof,
}
