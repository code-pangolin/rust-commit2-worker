use std::sync::Arc;

use fvm_shared::sector::SectorID;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Eq, Clone, Serialize, Deserialize)]
pub struct CallID {
    pub sector: SectorID,
    pub id: uuid::Uuid,
}

impl ToString for CallID {
    fn to_string(&self) -> String {
        format!(
            "{}-{}-{}",
            self.sector.miner,
            self.sector.number,
            self.id.to_string()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallError {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing, skip_deserializing)]
    sub: Option<Arc<anyhow::Error>>,
}

impl CallError {
    pub fn new(code: ErrorCode, err: Option<anyhow::Error>) -> Self {
        let message: String;
        let sub: Option<Arc<anyhow::Error>>;
        if let Some(e) = err {
            message = e.to_string();
            sub = Some(Arc::new(e));
        } else {
            message = String::new();
            sub = None;
        }
        Self { code, message, sub }
    }

    pub fn unwrap(&self) -> Arc<anyhow::Error> {
        if self.sub.is_some() {
            return self.sub.to_owned().unwrap();
        }
        return Arc::new(anyhow::anyhow!(self.message.clone()));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCode {
    ErrUnknown,
    ErrTempUnknown = 100,
    ErrTempWorkerRestart,
    ErrTempAllocateSpace,
}
