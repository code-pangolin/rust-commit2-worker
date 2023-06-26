use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("failed to read data")]
    ReadError(#[source] anyhow::Error),
    #[error("failed to write data")]
    WriteError(#[source] anyhow::Error),
    #[error("unknown data store error")]
    Unknown(#[source] anyhow::Error),
}
