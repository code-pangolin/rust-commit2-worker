use jsonrpc_v2::{Data, Error as JsonRpcError, Params};

use crate::rpc_api::{commit2_api::*, data_types::RPCState};

/// RPC call to create a new JWT Token
pub(in crate::rpc) async fn seal_commit2(
    _data: Data<RPCState>,
    Params(_params): Params<SealCommit2Params>,
) -> Result<SealCommit2Result, JsonRpcError> {
    //TODO: put worker state in rpcstate
    // leveldb
    //data store:https://github.com/filecoin-project/lotus/blob/6e7dc9532abdb3171427347710df4c860f1957a2/cmd/lotus-worker/main.go#L578

    //TODO: asyccall: https://github.com/filecoin-project/lotus/blob/6e7dc9532abdb3171427347710df4c860f1957a2/storage/sealer/worker_local.go#L428
    todo!()
}
