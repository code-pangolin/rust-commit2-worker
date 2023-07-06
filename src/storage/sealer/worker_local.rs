use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoStaticStr};
use uuid::Uuid;

use super::{
    statestore::StateStore,
    storiface::{
        storage::{Commit1Out, SectorRef},
        worker::{CallError, CallID},
    },
    worker_calltracker::WorkerCallTracker,
};
use crate::{api::api_storage::WorkerReturn, storage::ipfs::datastore::Datastore};

pub struct LocalWorker<
    T: Datastore,
    R: WorkerReturn + std::marker::Send + std::marker::Sync + Clone + 'static,
> {
    ct: WorkerCallTracker<T>,
    ret: R,
}

impl<T: Datastore, R: WorkerReturn + std::marker::Send + std::marker::Sync + Clone>
    LocalWorker<T, R>
{
    pub fn new(statestore: StateStore<T>, ret: R) -> Self {
        Self {
            ct: WorkerCallTracker::new(statestore),
            ret,
        }
    }

    pub fn seal_commit2(
        &self,
        _sector: SectorRef,
        _phase1_out: Commit1Out,
    ) -> anyhow::Result<CallID> {
        todo!()
    }

    pub fn async_call_seal_commit2<F>(
        &self,
        sector: SectorRef,
        rt: ReturnType,
        work: F,
    ) -> anyhow::Result<CallID>
    where
        F: FnOnce(CallID) -> anyhow::Result<Vec<u8>> + std::marker::Send + 'static,
    {
        let ci = CallID {
            sector: sector.id,
            id: Uuid::new_v4(),
        };

        self.ct.onStart(ci.clone(), rt)?;

        let ret = self.ret.clone();

        let _handler = tokio::spawn(async move {
            match work(ci.clone()) {
                Ok(res) => {
                    ret.return_seal_commit2(ci.clone(), res, None);
                }
                Err(_e) => todo!(),
            };
        });

        todo!()
    }
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

#[derive(Debug, Clone)]
pub struct ManagerReturn {} //TODO: move to MinerApi

impl WorkerReturn for ManagerReturn {
    fn return_seal_commit2(
        &self,
        call_id: CallID,
        proof: Vec<u8>,
        err: Option<&CallError>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
