// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use jsonrpc_v2::{Data, Error as JsonRpcError, Params};

use crate::rpc_api::{auth_api::*, data_types::RPCState};

/// RPC call to create a new JWT Token
pub(in crate::rpc) async fn auth_new(
    data: Data<RPCState>,
    Params(params): Params<AuthNewParams>,
) -> Result<AuthNewResult, JsonRpcError> {
    data.nodeapi
        .call_method(AUTH_NEW, "returns", params)
        .await
        .map_err(|e| JsonRpcError::from(e))
}

/// RPC call to verify JWT Token and return the token's permissions
pub(in crate::rpc) async fn auth_verify(
    data: Data<RPCState>,
    Params(params): Params<AuthVerifyParams>,
) -> Result<AuthVerifyResult, JsonRpcError> {
    data.nodeapi
        .call_method(AUTH_VERIFY, "returns", params)
        .await
        .map_err(|e| JsonRpcError::from(e))
}
