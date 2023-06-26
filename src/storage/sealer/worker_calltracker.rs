use serde::{Deserialize, Serialize};

use super::{statestore::StateStore, storiface::worker::CallID, worker_local::ReturnType};
use crate::storage::ipfs::datastore::Datastore;

#[derive(Debug, Deserialize, Serialize)]
pub struct Call {
    pub id: CallID,
    pub return_type: ReturnType,

    pub state: CallState,

    pub result: ManyBytes, // json bytes
}

pub(in crate::storage::sealer) struct WorkerCallTracker<'a, T: Datastore> {
    st: StateStore<'a, T>,
}

impl<'a, T: Datastore> WorkerCallTracker<'a, T> {
    pub(in crate::storage::sealer) fn onStart(
        &self,
        ci: CallID,
        rt: ReturnType,
    ) -> anyhow::Result<()> {
        self.st.begin(
            &ci.clone(),
            &Call {
                id: ci,
                return_type: rt,
                state: CallState::CallStarted,
                result: ManyBytes([].to_vec()),
            },
        )
    }

    pub(in crate::storage::sealer) fn onDone(
        &self,
        ci: CallID,
        ret: Vec<u8>,
    ) -> anyhow::Result<()> {
        let st = self.st.get(ci);
        st.mutate::<_, Call>(|cs| {
            cs.state = CallState::CallDone;
            cs.result = ManyBytes(ret)
        })
    }

    pub(in crate::storage::sealer) fn onReturned(&self, ci: CallID) -> anyhow::Result<()> {
        let st = self.st.get(ci);
        st.end()
    }

    pub(in crate::storage::sealer) fn unfinished(&self) -> anyhow::Result<Vec<Call>> {
        //TODO: self.st.list()
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CallState {
    CallStarted,
    CallDone,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ManyBytes(Vec<u8>);
