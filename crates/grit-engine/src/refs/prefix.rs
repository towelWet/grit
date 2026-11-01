use crate::store::paths::objects_dir;
use crate::{GritError, Hash, Result};
use std::path::Path;
mod commit_matches;
pub(super) use commit_matches::commit_matches;
mod bad_path;
use bad_path::bad_path;
