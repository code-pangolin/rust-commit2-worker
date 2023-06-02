use base64::Engine;
use fvm_shared::sector::{RegisteredSealProof, SectorID};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Eq, Clone, Serialize, Deserialize)]
pub struct SectorRef {
    pub id: SectorID,
    pub proof_type: RegisteredSealProof,
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Commit1Out(pub Vec<u8>);

impl Serialize for Commit1Out {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let base64 = base64::engine::general_purpose::STANDARD.encode(self.0.to_vec());
        String::serialize(&base64, serializer)
    }
}

impl<'de> Deserialize<'de> for Commit1Out {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let base64 = String::deserialize(deserializer)?;
        let v = base64::engine::general_purpose::STANDARD
            .decode(base64.as_bytes())
            .map_err(|e| serde::de::Error::custom(e))?;
        Ok(Commit1Out(v))
    }
}
