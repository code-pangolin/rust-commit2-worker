use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoStaticStr};

use crate::storage::ipfs::datastore::Datastore;

pub struct LocalWorker {
    storage: dyn Datastore,
}

#[derive(EnumString, IntoStaticStr, Debug, Serialize, Deserialize)]
pub enum ReturnType {
    DataCid,
    AddPiece,
    SealPreCommit1,
    SealPreCommit2,
    SealCommit1,
    SealCommit2,
    FinalizeSector,
    FinalizeReplicaUpdate,
    ReplicaUpdate,
    ProveReplicaUpdate1,
    ProveReplicaUpdate2,
    GenerateSectorKey,
    ReleaseUnsealed,
    MoveStorage,
    UnsealPiece,
    DownloadSector,
    Fetch,
}

impl ToString for ReturnType {
    fn to_string(&self) -> String {
        let s: &'static str = self.into();
        s.to_string()
    }
}
