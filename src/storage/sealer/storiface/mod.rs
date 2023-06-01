use filecoin_proofs::UnpaddedBytesAmount;
use serde::{Deserialize, Serialize};

use self::storage::SectorRef;
pub mod filetype;
pub mod storage;

#[derive(PartialEq, Debug, Eq, Clone, Serialize, Deserialize)]
pub struct UnpaddedPieceSize(pub u64);

#[derive(PartialEq, Debug, Eq, Clone, Serialize, Deserialize)]
pub struct AddPieceArgs {
    pub sector: SectorRef,
    pub piece_sizes: Vec<u64>,
    pub piece_size: UnpaddedPieceSize,
    pub root: String,
}

impl UnpaddedPieceSize {
    #[allow(unused)]
    pub fn as_unpadded_piece_size(self) -> fvm_shared::piece::UnpaddedPieceSize {
        fvm_shared::piece::UnpaddedPieceSize(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserilize() {
        let data = "12";
        let v = serde_json::from_str::<UnpaddedPieceSize>(data).unwrap();
        assert_eq!(v, UnpaddedPieceSize(12));
    }

    #[test]
    fn test_serilize() {
        let v = UnpaddedPieceSize(12);
        assert_eq!("12", serde_json::to_string(&v).unwrap());
    }
}
