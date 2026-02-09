pub mod pdf;

use sha2::{Digest, Sha256};
use std::path::Path;

use crate::error::AppResult;

pub fn compute_file_hash(path: &Path) -> AppResult<String> {
    let bytes = std::fs::read(path)?;
    let hash = Sha256::digest(&bytes);
    Ok(format!("{:x}", hash))
}

pub fn get_file_size(path: &Path) -> AppResult<i64> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len() as i64)
}
