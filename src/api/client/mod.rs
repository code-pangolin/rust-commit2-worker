mod error;
use std::{env, sync::Arc};

use async_trait::async_trait;
use jsonrpc_v2::{Error, Id, RequestObject, V2};
use libp2p::{multiaddr::Protocol, Multiaddr};
use log::debug;
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::Mutex;

use self::error::IntoAnyhow;
use super::api_storage::{StorageMiner, StorageMinerError, WorkerReturn};

#[derive(Debug, Clone)]
pub struct StorageMinerRpcClient {}

impl StorageMiner for StorageMinerRpcClient {
    // fn new(addr: String, token: Option<String>) -> Self {
    //     jsonrpc_v2::ServerBuilder::todo!()
    // }
}

pub type ReturnSealCommit2Params = ();
pub const RETURN_SEAL_COMMIT2: &str = "ReturnSealCommit2";

impl WorkerReturn for StorageMinerRpcClient {
    fn return_seal_commit2(
        &self,
        call_id: crate::storage::sealer::storiface::worker::CallID,
        proof: Vec<u8>,
        err: Option<&crate::storage::sealer::storiface::worker::CallError>,
    ) -> anyhow::Result<()> {
        call(RETURN_SEAL_COMMIT2, (call_id, proof, err)).into_anyhow()
    }
}

pub const API_INFO_KEY: &str = "FULLNODE_API_INFO";
pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_MULTIADDRESS: &str = "/ip4/127.0.0.1/tcp/1234/http";
pub const DEFAULT_PORT: u16 = 1234;
pub const DEFAULT_PROTOCOL: &str = "http";
pub const RPC_ENDPOINT: &str = "rpc/v0";

pub struct ApiInfo {
    pub multiaddr: Multiaddr,
    pub token: Option<String>,
}

pub static API_INFO: Lazy<ApiInfo> = Lazy::new(|| {
    // Get API_INFO environment variable if exists, otherwise, use default
    // multiaddress
    let api_info = env::var(API_INFO_KEY).unwrap_or_else(|_| DEFAULT_MULTIADDRESS.to_owned());

    let (multiaddr, token) = match api_info.split_once(':') {
        // Typically this is when a JWT was provided
        Some((jwt, host)) => (
            host.parse().expect("Parse multiaddress"),
            Some(jwt.to_owned()),
        ),
        // Use entire API_INFO env var as host string
        None => (api_info.parse().expect("Parse multiaddress"), None),
    };

    ApiInfo { multiaddr, token }
});

/// Error object in a response
#[derive(Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum JsonRpcResponse<R> {
    Result {
        jsonrpc: V2,
        result: R,
        id: Id,
    },
    Error {
        jsonrpc: V2,
        error: JsonRpcError,
        id: Id,
    },
}

struct Url {
    protocol: String,
    port: u16,
    host: String,
}

/// Parses a multi-address into a URL
fn multiaddress_to_url(multiaddr: Multiaddr) -> String {
    // Fold Multiaddress into a Url struct
    let addr = multiaddr.into_iter().fold(
        Url {
            protocol: DEFAULT_PROTOCOL.to_owned(),
            port: DEFAULT_PORT,
            host: DEFAULT_HOST.to_owned(),
        },
        |mut addr, protocol| {
            match protocol {
                Protocol::Ip6(ip) => {
                    addr.host = ip.to_string();
                }
                Protocol::Ip4(ip) => {
                    addr.host = ip.to_string();
                }
                Protocol::Dns(dns) => {
                    addr.host = dns.to_string();
                }
                Protocol::Dns4(dns) => {
                    addr.host = dns.to_string();
                }
                Protocol::Dns6(dns) => {
                    addr.host = dns.to_string();
                }
                Protocol::Dnsaddr(dns) => {
                    addr.host = dns.to_string();
                }
                Protocol::Tcp(p) => {
                    addr.port = p;
                }
                Protocol::Http => {
                    addr.protocol = "http".to_string();
                }
                Protocol::Https => {
                    addr.protocol = "https".to_string();
                }
                _ => {}
            };
            addr
        },
    );

    // Format, print and return the URL
    let url = format!(
        "{}://{}:{}/{}",
        addr.protocol, addr.host, addr.port, RPC_ENDPOINT
    );

    url
}

/// Utility method for sending RPC requests over HTTP
fn call<P, R>(method_name: &str, params: P) -> Result<R, Error>
where
    P: Serialize,
    R: DeserializeOwned,
{
    let rpc_req = RequestObject::request()
        .with_method(method_name)
        .with_params(serde_json::to_value(params)?)
        .finish();

    let api_url = multiaddress_to_url(API_INFO.multiaddr.to_owned());

    debug!("Using JSON-RPC v2 HTTP URL: {}", api_url);

    let request = global_http_client().post(api_url).json(&rpc_req);
    let request = match API_INFO.token.as_ref() {
        Some(token) => request.header(http::header::AUTHORIZATION, token),
        _ => request,
    };

    let rpc_res = request.send()?.error_for_status()?.json()?;

    match rpc_res {
        JsonRpcResponse::Result { result, .. } => Ok(result),
        JsonRpcResponse::Error { error, .. } => Err(Error::Full {
            data: None,
            code: error.code,
            message: error.message,
        }),
    }
}

pub fn global_http_client() -> reqwest::blocking::Client {
    static CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(reqwest::blocking::Client::new);
    CLIENT.clone()
}
