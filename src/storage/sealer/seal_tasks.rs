#![allow(unused)]

#[derive(Debug, Clone)]
pub enum TaskType {
    TTIDLE,
    TTDataCid,
    TTAddPiece,
    TTPreCommit1,
    TTPreCommit2,
    TTCommit1,
    TTCommit2,

    TTFinalize,
    TTFinalizeUnsealed,

    TTFetch,
    TTUnseal,

    TTReplicaUpdate,
    TTProveReplicaUpdate1,
    TTProveReplicaUpdate2,
    TTRegenSectorKey,
    TTFinalizeReplicaUpdate,

    TTDownloadSector,

    TTGenerateWindowPoSt,
    TTGenerateWinningPoSt,

    TTNoop,
}

impl Default for TaskType {
    fn default() -> Self {
        TaskType::TTNoop
    }
}

impl ToString for TaskType {
    fn to_string(&self) -> String {
        match self {
            TaskType::TTIDLE => "seal/v0/idle".to_string(),
            TaskType::TTDataCid => "seal/v0/datacid".to_string(),
            TaskType::TTAddPiece => "seal/v0/addpiece".to_string(),
            TaskType::TTPreCommit1 => "seal/v0/precommit/1".to_string(),
            TaskType::TTPreCommit2 => "seal/v0/precommit/2".to_string(),
            TaskType::TTCommit1 => "seal/v0/commit/1".to_string(),
            TaskType::TTCommit2 => "seal/v0/commit/2".to_string(),

            TaskType::TTFinalize => "seal/v0/finalize".to_string(),
            TaskType::TTFinalizeUnsealed => "seal/v0/finalizeunsealed".to_string(),

            TaskType::TTFetch => "seal/v0/fetch".to_string(),
            TaskType::TTUnseal => "seal/v0/unseal".to_string(),

            TaskType::TTReplicaUpdate => "seal/v0/replicaupdate".to_string(),
            TaskType::TTProveReplicaUpdate1 => "seal/v0/provereplicaupdate/1".to_string(),
            TaskType::TTProveReplicaUpdate2 => "seal/v0/provereplicaupdate/2".to_string(),
            TaskType::TTRegenSectorKey => "seal/v0/regensectorkey".to_string(),
            TaskType::TTFinalizeReplicaUpdate => "seal/v0/finalize/replicaupdate".to_string(),

            TaskType::TTDownloadSector => "seal/v0/download/sector".to_string(),

            TaskType::TTGenerateWindowPoSt => "post/v0/windowproof".to_string(),
            TaskType::TTGenerateWinningPoSt => "post/v0/winningproof".to_string(),

            TaskType::TTNoop => "".to_string(),
        }
    }
}
