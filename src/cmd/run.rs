use anyhow::Ok;
use clap::Parser;
use log::warn;

use super::Command;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "run")]
#[command(about = "Start lotus commit2 worker", long_about = None)]
pub(crate) struct Run {
    /// host address and port the worker api will listen on
    #[arg(long, env = "LOTUS_WORKER_LISTEN", default_value = "0.0.0.0:3456")]
    listen: Option<String>,

    /// extra net address external c2
    #[arg(long, env = "LOTUS_WORKER_EXT_LISTEN")]
    ext_address: Option<String>,

    #[arg(long, hide = true)]
    address: Option<String>,

    /// don't use storageminer repo for sector storage
    #[arg(long, env = "LOTUS_WORKER_NO_LOCAL_STORAGE")]
    no_local_storage: Option<bool>,

    /// don't use swap
    #[arg(long, env = "LOTUS_WORKER_NO_SWAP", default_value = "false")]
    no_swap: bool,

    /// custom worker name, default: hostname
    #[arg(long, env = "LOTUS_WORKER_NAME")]
    name: Option<String>,

    /// enable addpiece
    #[arg(long, env = "LOTUS_WORKER_ADDPIECE", default_value = "true")]
    addpiece: bool,

    /// enable precommit1
    #[arg(long, env = "LOTUS_WORKER_PRECOMMIT1", default_value = "true")]
    precommit1: bool,

    /// enable unsealing
    #[arg(long, env = "LOTUS_WORKER_UNSEAL", default_value = "true")]
    unseal: bool,

    /// enable precommit2
    #[arg(long, env = "LOTUS_WORKER_PRECOMMIT2", default_value = "true")]
    precommit2: bool,

    /// enable commit
    #[arg(long, env = "LOTUS_WORKER_COMMIT", default_value = "true")]
    commit: bool,

    /// enable replica update
    #[arg(long, env = "LOTUS_WORKER_REPLICA_UPDATE", default_value = "true")]
    replica_update: bool,

    /// enable prove replica update 2
    #[arg(
        long,
        env = "LOTUS_WORKER_PROVE_REPLICA_UPDATE2",
        default_value = "true"
    )]
    prove_replica_update2: bool,

    /// enable regen sector key
    #[arg(long, env = "LOTUS_WORKER_REGEN_SECTOR_KEY", default_value = "true")]
    regen_sector_key: bool,

    /// enable external sector data download
    #[arg(long, env = "LOTUS_WORKER_SECTOR_DOWNLOAD", default_value = "false")]
    sector_download: bool,

    /// enable window post
    #[arg(long, env = "LOTUS_WORKER_WINDOWPOST", default_value = "false")]
    windowpost: bool,

    /// enable winning post
    #[arg(long, env = "LOTUS_WORKER_WINNINGPOST", default_value = "false")]
    winningpost: bool,

    /// disable all default compute tasks, use the worker for storage/fetching only
    #[arg(long, env = "LOTUS_WORKER_NO_DEFAULT", default_value = "false")]
    no_default: bool,

    /// maximum fetch operations to run in parallel
    #[arg(long, env = "LOTUS_WORKER_PARALLEL_FETCH_LIMIT", default_value = "5")]
    parallel_fetch_limit: i64,

    /// maximum number of parallel challenge reads (0 = no limit)
    #[arg(long, env = "LOTUS_WORKER_POST_PARALLEL_READS", default_value = "128")]
    post_parallel_reads: i64,

    /// time limit for reading PoSt challenges (0 = no limit)
    #[arg(long, env = "LOTUS_WORKER_POST_READ_TIMEOUT", default_value = "0")]
    post_read_timeout: i64,

    /// used when 'listen' is unspecified. must be a valid duration recognized by golang's time.ParseDuration function
    #[arg(long, env = "LOTUS_WORKER_TIMEOUT", default_value = "30m")]
    timeout: Option<String>,

    #[arg(long, default_value = "30s")]
    http_server_timeout: Option<String>,
}

impl Command for Run {
    fn before(&self) -> anyhow::Result<()> {
        if self.address.is_some() {
            warn!("The '--address' flag is deprecated, it has been replaced by '--listen'")
        }
        Ok(())
    }
    fn action(&self) -> anyhow::Result<()> {
        println!("asdasda");
        Ok(())
    }
}
