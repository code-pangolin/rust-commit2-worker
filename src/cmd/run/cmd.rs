use std::{
    env,
    fs::{create_dir_all, read_dir, remove_file},
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
};

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use clap::Parser;
use fil_proofs_param::{get_params, get_srs, params_json, srs_json};
use futures::{future, join};
use log::{info, warn};
use parse_size::parse_size;
use tokio::sync::RwLock;
use version::version;

use super::check_env::{set_disk_space, show_env};
use crate::{
    cmd::{
        run::handler::{self, Handler},
        Command,
    },
    storage::sealer::{seal_tasks::TaskType, worker::WorkerInfo},
};

#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "run")]
#[command(about = "Start lotus commit2 worker", long_about = None)]
pub(crate) struct Run {
    #[clap(from_global)]
    worker_repo: String,

    /// host address and port the worker api will listen on
    #[arg(long, default_value = "0.0.0.0:3456", env = "LOTUS_WORKER_LISTEN")]
    listen: String,

    /// extra net address external c2
    #[arg(long, env = "LOTUS_WORKER_EXT_LISTEN")]
    ext_address: Option<String>,

    #[arg(long, hide = true)]
    address: Option<String>,

    /// size of the sectors in bytes, i.e. 32GiB
    #[arg(long, env = "LOTUS_WORKER_SECTOR_SIZE", default_value = "32GiB")]
    sector_size: String,

    /// seal commit2 task total number
    #[arg(long, env = "LOTUS_WORKER_SEAL_NUM", default_value = "1")]
    seal_num: u64,

    /// skip proof param
    #[arg(long, env = "LOTUS_WORKER_SKIP_PARAM", default_value = "false")]
    skip_param: bool,

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

#[async_trait]
impl Command for Run {
    async fn before(&self) -> Result<()> {
        if self.address.is_some() {
            warn!("The '--address' flag is deprecated, it has been replaced by '--listen'")
        }
        Ok(())
    }
    async fn action(&self) -> Result<()> {
        show_env();
        let sector_size_int = match parse_size(&self.sector_size) {
            Ok(o) => o,
            Err(e) => return Err(anyhow!("parse sector_size: {}", e)),
        };

        info!("sector size: {}", sector_size_int);

        if !self.skip_param {
            let (r1, r2) = future::join(
                get_params(params_json(), sector_size_int),
                get_srs(srs_json()),
            )
            .await;
            r1?;
            r2?;
        }

        info!("Starting lotus worker version: {}", version!());
        info!("sector size: {}", self.sector_size);

        let r_addr = match &self.ext_address {
            Some(v) => v.clone(),
            None => self.listen.clone(),
        };

        create_dir_all(&self.worker_repo).context("create worker repo dir")?;
        create_dir_all(PathBuf::from(&self.worker_repo).join("cache"))
            .context("error create lotus cache dir")?;
        create_dir_all(PathBuf::from(&self.worker_repo).join("unsealed"))
            .context("error create lotus unsealed dir")?;
        create_dir_all(PathBuf::from(&self.worker_repo).join("sealed"))
            .context("error create lotus sealed dir")?;
        create_dir_all(PathBuf::from(&self.worker_repo).join("result"))
            .context("error create lotus result dir")?;
        create_dir_all(PathBuf::from(&self.worker_repo).join("tmp"))
            .context("error create lotus tmp dir")?;

        let sealnums_ori = if self.seal_num == 0 { 1 } else { self.seal_num };
        info!("worker seal task number: {}", sealnums_ori);

        set_disk_space(sector_size_int, sealnums_ori);

        let mut task_types = vec![];
        let mut worker_type = String::new();
        let mut sealnums = sealnums_ori;

        if self.precommit1 {
            task_types.push(TaskType::TTAddPiece);
            task_types.push(TaskType::TTPreCommit1);
            worker_type.push_str("p1");
        }

        if self.precommit2 {
            let mut set_p1_key = env::var("FIL_PROOFS_P1_TURBO_NUM").unwrap_or(String::new());
            if set_p1_key.is_empty() {
                set_p1_key = self.seal_num.to_string();
                env::set_var("FIL_PROOFS_P1_TURBO_NUM", set_p1_key.to_string())
            }

            task_types.push(TaskType::TTPreCommit2);
            task_types.push(TaskType::TTCommit1);
            task_types.push(TaskType::TTFinalize);
            worker_type.push_str("p2");
            sealnums = sealnums_ori * 3;
        }

        if self.commit {
            env::set_var("EC_GPU_FRAMEWORK", "cuda");
            env::set_var("BELLMAN_GPU_FRAMEWORK", "cuda");

            task_types.push(TaskType::TTCommit2);
            task_types.push(TaskType::TTFinalize);
            worker_type.push_str("c2");
            sealnums = sealnums_ori * 3;

            let rd =
                read_dir(PathBuf::from(&self.worker_repo).join("tmp")).context("read tmp dir")?;
            for fi in rd {
                match fi {
                    Ok(v) => {
                        if v.file_name()
                            .to_str()
                            .unwrap_or_default()
                            .contains("bellman")
                        {
                            remove_file(v.path())?;
                        }
                    }
                    Err(e) => warn!("delete bellman file error ,err is : {}", e),
                }
            }
        }

        let mut seal_handler = Handler::default();
        seal_handler.worker_info = RwLock::new(WorkerInfo {
            hostname: r_addr,
            seal_num: sealnums,
            sealing: 0,
            worker_type,
            status: TaskType::TTIDLE,
            ignore_resources: true,
            active: true,
            tasks: task_types,
            resources: Default::default(),
            sectors_maps: Default::default(),
            version: Default::default(),
            choose_time: Default::default(),
        });
        seal_handler.worker_repo = self.worker_repo.clone();

        info!("Setting up control endpoint at {}", &self.listen);

        let addr: SocketAddr = self.listen.parse()?;

        let h = Arc::new(seal_handler);

        let handler =
            handler::router(h.clone()).into_make_service_with_connect_info::<SocketAddr>();

        let _ = join!(axum::Server::bind(&addr).serve(handler), handler::log(h));

        Ok(())
    }
}
