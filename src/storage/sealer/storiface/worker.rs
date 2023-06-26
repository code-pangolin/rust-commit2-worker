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
