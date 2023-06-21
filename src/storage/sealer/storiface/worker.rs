use fvm_shared::sector::{RegisteredSealProof, SectorID};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Eq, Clone, Serialize, Deserialize)]
pub struct CallID {
    pub sector: SectorID,
    pub id: uuid::Uuid,
}
