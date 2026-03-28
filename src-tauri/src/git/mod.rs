pub mod actions;
pub mod diff;
pub mod graph;
pub mod refs;
pub mod status;

use crate::error::AppError;
use std::path::Path;

pub fn open_repo(path: &Path) -> Result<gix::Repository, AppError> {
    gix::open(path).map_err(|e| AppError::Git(e.to_string()))
}
