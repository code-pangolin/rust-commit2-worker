use std::sync::Arc;

use fvm_shared::address::Address;
use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoStaticStr};
use uuid::Uuid;

use super::{
    statestore::StateStore,
    storiface::{
        storage::{Commit1Out, SectorRef},
        worker::{CallError, CallID, ErrorCode},
    },
    worker_calltracker::WorkerCallTracker,
};
use crate::{api::api_storage::WorkerReturn, storage::ipfs::datastore::Datastore};

pub struct LocalWorker<
    T: Datastore + std::marker::Send + std::marker::Sync + 'static,
    R: WorkerReturn + std::marker::Send + std::marker::Sync + Clone + 'static,
> {
    ct: Arc<WorkerCallTracker<T>>,
    ret: R,
}

impl<
        T: Datastore + std::marker::Send + std::marker::Sync + 'static,
        R: WorkerReturn + std::marker::Send + std::marker::Sync + Clone,
    > LocalWorker<T, R>
{
    pub fn new(statestore: StateStore<T>, ret: R) -> Self {
        Self {
            ct: Arc::new(WorkerCallTracker::new(statestore)),
            ret,
        }
    }

    pub fn seal_commit2(
        &self,
        sector: SectorRef,
        phase1_out: Commit1Out,
    ) -> anyhow::Result<CallID> {
        let ci = CallID {
            sector: sector.clone().id,
            id: Uuid::new_v4(),
        };

        let _rt = ReturnType::SealCommit2;

        // self.ct.blocking_lock().onStart(ci.clone(), rt)?;

        let ret = self.ret.clone();
        let callid = ci.clone();
        let ct = self.ct.clone();

        let _handler = tokio::spawn(async move {
            let scp1o = serde_json::from_slice::<filecoin_proofs_api::seal::SealCommitPhase1Output>(
                phase1_out.0.as_slice(),
            );

            if let Err(e) = scp1o {
                ret.return_seal_commit2(
                    callid.clone(),
                    vec![],
                    Some(&CallError::new(ErrorCode::ErrUnknown, Some(e.into()))),
                );
                return;
            }

            let maddr = Address::new_id(sector.id.miner);

            let mut prover_id: [u8; 32] = [0; 32];
            let payload = maddr.payload().to_bytes();
            prover_id[..payload.len()].copy_from_slice(&payload);

            let res = filecoin_proofs_api::seal::seal_commit_phase2(
                scp1o.unwrap(),
                prover_id,
                filecoin_proofs_api::SectorId::from(sector.id.number),
            );

            ct.onDone(callid.clone(), res.as_ref().unwrap().proof.clone());

            //TODO: retry
            ret.return_seal_commit2(callid.clone(), res.unwrap().proof, None);
            ct.onReturned(callid);
        });

        Ok(ci)
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
    async fn return_seal_commit2(
        &self,
        _call_id: CallID,
        _proof: Vec<u8>,
        _err: Option<&CallError>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
