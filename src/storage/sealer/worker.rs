use std::collections::HashMap;

use fvm_shared::sector::RegisteredSealProof;

use super::{resources::Resources, seal_tasks};

#[derive(Debug, Default)]
pub struct WorkerInfo {
    pub hostname: String,
    /// IgnoreResources indicates whether the worker's available resources should
    /// be used ignored (true) or used (false) for the purposes of scheduling and
    /// task assignment. Only supported on local workers. Used for testing.
    /// Default should be false (zero value, i.e. resources taken into account).
    pub ignore_resources: bool,
    pub resources: WorkerResources,

    pub active: bool,
    pub sectors_maps: HashMap<String, String>,
    pub version: String,
    pub worker_type: String,
    pub status: seal_tasks::TaskType,
    pub sealing: u64,
    pub seal_num: u64,
    pub tasks: Vec<seal_tasks::TaskType>,
    pub choose_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Default)]
pub struct WorkerResources {
    pub men_physical: u64,
    pub mem_used: u64,
    pub mem_swap: u64,
    pub mem_swap_used: u64,

    pub cpus: u64, // Logical cores
    pub gpus: Vec<String>,

    // if nil use the default resource table
    pub resources: HashMap<seal_tasks::TaskType, HashMap<RegisteredSealProof, Resources>>,
}
