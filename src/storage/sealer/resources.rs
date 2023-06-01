#[derive(Debug, Clone)]
pub struct Resources {
    /// What Must be in RAM for decent perf
    pub min_memory: u64,
    /// Memory required (swap + ram; peak memory usage during task execution)
    pub max_memory: u64,

    /// GPUUtilization specifes the number of GPUs a task can use
    pub gpu_utilization: f64,

    /// MaxParallelism specifies the number of CPU cores when GPU is NOT in use
    pub max_parallelism: i32, // -1 = multithread

    /// MaxParallelismGPU specifies the number of CPU cores when GPU is in use
    pub max_parallelism_gpu: i32, // when 0, inherits MaxParallelism
    /// What Must be in RAM for decent perf (shared between threads)
    pub base_min_memory: u64,

    /// Maximum number of tasks of this type that can be scheduled on a worker (0=default, no limit)
    pub max_concurrent: i32,
}
