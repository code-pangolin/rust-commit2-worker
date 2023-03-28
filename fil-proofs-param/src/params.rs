use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use blake2b_simd::State as Blake2b;

// Produces a BLAKE2b checksum for a file within the cache
pub fn get_digest_for_file(dir: &Path, filename: &str) -> Result<String> {
    let path = get_full_path_for_file(dir, filename);
    let mut file = File::open(&path).with_context(|| format!("could not open path={:?}", path))?;
    let mut hasher = Blake2b::new();

    io::copy(&mut file, &mut hasher)?;

    Ok(hasher.finalize().to_hex()[..32].into())
}

// Produces an absolute path to a file within the cache
pub fn get_full_path_for_file(path: &Path, filename: &str) -> PathBuf {
    path.join(filename)
}
