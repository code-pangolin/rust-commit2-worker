#![deny(clippy::all, clippy::perf, clippy::correctness)]
#![warn(clippy::unwrap_used)]

mod http;
mod params;

use std::{
    convert::TryInto,
    env,
    fs::{self, remove_file},
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::{anyhow, Result};
use backoff::{future::retry, ExponentialBackoff};
use filecoin_proofs::param::has_extension;
use futures::TryStreamExt;
use humansize::{file_size_opts, FileSize};
use log::{debug, error, info, trace, warn};
use storage_proofs_core::parameter_cache::{ParameterMap, GROTH_PARAMETER_EXT};
use tokio::{fs::File, io::BufWriter};
use tokio_util::compat::FuturesAsyncReadCompatExt;

use crate::{
    http::https_client,
    params::{get_digest_for_file, get_full_path_for_file},
};

static DIR_ENV: &str = "FIL_PROOFS_PARAMETER_CACHE";
static PARAM_DIR: &str = "/var/tmp/filecoin-proof-parameters";
pub const DEFAULT_JSON: &str = include_str!("../parameters.json");
pub const DEFAULT_SRS_JSON: &str = include_str!("../srs-inner-product.json");
static GATEWAY_ENV: &str = "IPFS_GATEWAY";
static GATEWAY: &str = "https://proofs.filecoin.io/ipfs/";

fn get_param_dir() -> String {
    match env::var(DIR_ENV) {
        Ok(v) => {
            if v.is_empty() {
                PARAM_DIR.to_string()
            } else {
                v
            }
        }
        Err(_) => PARAM_DIR.to_string(),
    }
}

pub fn params_json() -> &'static str {
    DEFAULT_JSON
}

pub fn srs_json() -> &'static str {
    DEFAULT_SRS_JSON
}

async fn download_parameter_map(
    parameter_map: ParameterMap,
    selected_file_names: Vec<String>,
    get_param_dir: &PathBuf,
) -> Result<()> {
    fs::create_dir_all(&get_param_dir)?;

    // Determine which files are outdated.
    let filenames =
        get_filenames_requiring_download(&parameter_map, selected_file_names, &get_param_dir);
    if filenames.is_empty() {
        debug!("no outdated files, exiting");
        return Ok(());
    };

    let mut tasks = Vec::with_capacity(filenames.len());

    for filename in filenames {
        info!("downloading params file: {}", filename);
        let dir_clone = get_param_dir.clone();
        let info = DownloadInfo {
            cid: parameter_map[&filename].cid.clone(),
            digest: parameter_map[&filename].digest.clone(),
        };

        tasks.push(tokio::task::spawn(async move {
            fetch_verify_params(dir_clone, &filename, info)
                .await
                .map_err(|err| {
                    error!("Error fetching param file {filename}: {err}");
                    err
                })
        }));
    }

    let mut errors = Vec::<anyhow::Error>::new();
    for t in tasks {
        match t.await {
            Err(err) => errors.push(err.into()),
            Ok(Err(err)) => errors.push(err),
            _ => (),
        }
    }

    if !errors.is_empty() {
        let error_messages: Vec<_> = errors.iter().map(|e| format!("{e}")).collect();
        anyhow::bail!(anyhow::Error::msg(format!(
            "Aggregated errors:\n{}",
            error_messages.join("\n\n")
        )))
    };

    Ok(())
}

pub async fn get_params(params_json: &str, size: u64) -> Result<()> {
    let get_param_dir = PathBuf::from(get_param_dir());
    fs::create_dir_all(&get_param_dir)?;

    let parameter_map: ParameterMap = serde_json::from_str(params_json)
        .map_err(|e| {
            error!("failed to parse built-in json, exiting\n{:?}", e);
            exit(1);
        })
        .unwrap();

    let mut filenames: Vec<String> = parameter_map.keys().cloned().collect();
    trace!("json contains {} files", filenames.len());

    // Filter out unwanted sector sizes from params files (.params files only, leave verifying-key
    // files).
    filenames.retain(|filename| {
        let remove = has_extension(filename, GROTH_PARAMETER_EXT)
            || parameter_map.get(filename).unwrap().sector_size != size;
        if remove {
            let human_size = parameter_map[filename]
                .sector_size
                .file_size(file_size_opts::BINARY)
                .unwrap();
            trace!("ignoring file: {} ({})", filename, human_size);
        }
        !remove
    });

    download_parameter_map(parameter_map, filenames, &get_param_dir).await
}

pub async fn get_srs(srs_json: &str) -> Result<()> {
    let get_param_dir = PathBuf::from(get_param_dir());
    fs::create_dir_all(&get_param_dir)?;

    let parameter_map: ParameterMap = serde_json::from_str(srs_json)
        .map_err(|e| {
            error!("failed to parse built-in json, exiting\n{:?}", e);
            exit(1);
        })
        .unwrap();

    let filenames: Vec<String> = parameter_map.keys().cloned().collect();
    trace!("json contains {} files", filenames.len());

    download_parameter_map(parameter_map, filenames, &get_param_dir).await
}

/// Check which files are outdated (or no not exist).
fn get_filenames_requiring_download(
    parameter_map: &ParameterMap,
    selected_filenames: Vec<String>,
    path: &Path,
) -> Vec<String> {
    selected_filenames
        .into_iter()
        .filter(|filename| {
            trace!("determining if file is out of date: {}", filename);
            let file_path = get_full_path_for_file(path, filename);
            if !file_path.exists() {
                trace!("file not found, marking for download");
                return true;
            };
            trace!("params file found");
            let calculated_digest = match get_digest_for_file(path, filename) {
                Ok(digest) => digest,
                Err(e) => {
                    warn!("failed to hash file {}, marking for download", e);
                    return true;
                }
            };
            let expected_digest = &parameter_map[filename].digest;
            if &calculated_digest == expected_digest {
                trace!("file is up to date:{}", &file_path.to_str().unwrap());
                false
            } else {
                trace!("file has unexpected digest, marking for download");
                remove_file(&file_path).expect(
                    format!("failed to remove file: {}", file_path.to_str().unwrap()).as_str(),
                );
                true
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct DownloadInfo {
    cid: String,
    digest: String,
}

async fn fetch_verify_params(dir: PathBuf, name: &str, info: DownloadInfo) -> anyhow::Result<()> {
    fetch_params(&get_full_path_for_file(&dir, name), &info).await?;
    let calculated_digest = get_digest_for_file(&dir, name).unwrap();
    let expected_digest = &info.digest;
    if &calculated_digest != expected_digest {
        return Err(anyhow!("file has unexpected digest"));
    }
    Ok(())
}

// ref https://github.com/ChainSafe/forest/blob/443b2c5736fef668691d6dfec1ac444e1e812171/utils/paramfetch/src/lib.rs#L164
async fn fetch_params(path: &Path, info: &DownloadInfo) -> Result<(), anyhow::Error> {
    let gw = std::env::var(GATEWAY_ENV).unwrap_or_else(|_| GATEWAY.to_owned());
    info!("Fetching param file {:?} from {}", path, gw);
    let url = format!("{}{}", gw, info.cid);
    let result = retry(ExponentialBackoff::default(), || async {
        Ok(fetch_params_inner(&url, path).await?)
    })
    .await;
    debug!("Done fetching param file {:?} from {}", path, gw);
    result
}

async fn fetch_params_inner(url: impl AsRef<str>, path: &Path) -> Result<(), anyhow::Error> {
    debug!("download file from: {}", url.as_ref());
    let client = https_client();
    let req = client.get(url.as_ref().try_into()?);
    let response = req.await.map_err(|e| anyhow::anyhow!(e))?;
    anyhow::ensure!(response.status().is_success());
    let content_len = response
        .headers()
        .get("content-length")
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse::<u64>().ok())
        .ok_or_else(|| anyhow::anyhow!("Couldn't retrieve content length"))?;
    let map_err: fn(hyper::Error) -> futures::io::Error =
        |e| futures::io::Error::new(futures::io::ErrorKind::Other, e);
    let mut source = response
        .into_body()
        .map_err(map_err)
        .into_async_read()
        .compat();
    let file = File::create(path).await?;
    let mut writer = BufWriter::new(file);
    tokio::io::copy(&mut source, &mut writer).await?;
    let file_metadata = std::fs::metadata(path)?;
    anyhow::ensure!(file_metadata.len() == content_len);
    Ok(())
}
