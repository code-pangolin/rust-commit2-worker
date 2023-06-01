use std::env;

use log::info;

pub fn show_env() {
    let nilfunc: fn(env: &str) = |_| {};
    info!("TMPDIR: {}", check_env("TMPDIR", true, nilfunc));
    info!(
        "GOLOG_LOG_FMT: {}",
        check_env("GOLOG_LOG_FMT", false, nilfunc)
    );
    info!("GOLOG_FILE: {}", check_env("GOLOG_FILE", false, nilfunc));
    info!("RUST_LOG: {}", check_env("RUST_LOG", false, nilfunc));

    info!(
        "NEPTUNE_GPU_INDEX: {}",
        check_env("NEPTUNE_GPU_INDEX", false, nilfunc)
    );
    info!(
        "BELLMAN_NO_GPU: {}",
        check_env("BELLMAN_NO_GPU", false, nilfunc)
    );
    info!(
        "BELLMAN_FFT_NO_GPU: {}",
        check_env("BELLMAN_FFT_NO_GPU", false, nilfunc)
    );
    info!(
        "BELLMAN_WORKING_GPUS: {}",
        check_env("BELLMAN_WORKING_GPUS", false, |e| {
            let trime = e.trim_end_matches(",");
            env::set_var("BELLMAN_WORKING_GPUS", trime);
        })
    );

    info!(
        "EC_GPU_FRAMEWORK: {}",
        check_env("EC_GPU_FRAMEWORK", false, nilfunc)
    );
}

fn check_env(env: &str, must: bool, cb: fn(env: &str)) -> String {
    let env_var = env::var(env);

    if (env_var.as_ref().is_err() || env_var.as_ref().unwrap().is_empty()) && must {
        panic!("env: {} can't empty", env)
    };

    cb(&env_var.clone().unwrap_or("".to_owned()));
    env_var.unwrap_or("".to_owned()).to_string()
}

pub fn set_disk_space(ssize: u64, num: u64) {
    env::set_var("FIL_PROOFS_P1_THRESHOLD", format!("{}", ssize * (num + 10)))
}
