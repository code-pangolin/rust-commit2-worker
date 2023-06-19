// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT
/// In general, `forest` wants to support the same RPC messages as `lotus` (go
/// implementation of Filecoin). Current progress is tracked in
/// `ARCHITECTURE.md`.
///
/// Follow the pattern set below, and don't forget to add an entry to the
/// [`ACCESS_MAP`] with the relevant permissions (consult the go implementation,
/// looking for a comment like `// perm: admin`)
use ahash::{HashMap, HashMapExt};
use once_cell::sync::Lazy;

pub mod data_types;

/// Access levels to be checked against JWT claims
pub enum Access {
    Admin,
    Sign,
    Write,
    Read,
}

/// Access mapping between method names and access levels
/// Checked against JWT claims on every request
pub static ACCESS_MAP: Lazy<HashMap<&str, Access>> = Lazy::new(|| {
    let mut access = HashMap::new();

    // Auth API
    access.insert(auth_api::AUTH_NEW, Access::Admin);
    access.insert(auth_api::AUTH_VERIFY, Access::Read);

    // Common API
    access.insert(common_api::VERSION, Access::Read);
    access.insert(common_api::SHUTDOWN, Access::Admin);
    access.insert(common_api::START_TIME, Access::Read);

    access
});

/// Checks an access enumeration against provided JWT claims
pub fn check_access(access: &Access, claims: &[String]) -> bool {
    match access {
        Access::Admin => claims.contains(&"admin".to_owned()),
        Access::Sign => claims.contains(&"sign".to_owned()),
        Access::Write => claims.contains(&"write".to_owned()),
        Access::Read => claims.contains(&"read".to_owned()),
    }
}

/// JSON-RPC API defaults
pub const DEFAULT_MULTIADDRESS: &str = "/ip4/127.0.0.1/tcp/1234/http";
pub const API_INFO_KEY: &str = "FULLNODE_API_INFO";

/// JSON-RPC API definitions

/// Authorization API
pub mod auth_api {
    use chrono::Duration;
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, DurationSeconds};

    pub const AUTH_NEW: &str = "Filecoin.AuthNew";
    #[serde_as]
    #[derive(Deserialize, Serialize)]
    pub struct AuthNewParams {
        pub perms: Vec<String>,
        #[serde_as(as = "DurationSeconds<i64>")]
        pub token_exp: Duration,
    }
    pub type AuthNewResult = Vec<u8>;

    pub const AUTH_VERIFY: &str = "Filecoin.AuthVerify";
    pub type AuthVerifyParams = (String,);
    pub type AuthVerifyResult = Vec<String>;
}

/// Common API
pub mod common_api {
    use chrono::Utc;

    use super::data_types::APIVersion;

    pub const VERSION: &str = "Filecoin.Version";
    pub type VersionParams = ();
    pub type VersionResult = APIVersion;

    pub const SHUTDOWN: &str = "Filecoin.Shutdown";
    pub type ShutdownParams = ();
    pub type ShutdownResult = ();

    pub const START_TIME: &str = "Filecoin.StartTime";
    pub type StartTimeParams = ();
    pub type StartTimeResult = chrono::DateTime<Utc>;
}
