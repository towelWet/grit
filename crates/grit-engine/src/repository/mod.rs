mod find;
mod init;
mod open;

use crate::config::GritConfig;
use crate::ignore::IgnoreRules;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Repository {
    pub root: PathBuf,
    pub config: GritConfig,
    pub ignore: IgnoreRules,
}

impl Repository {
    pub fn grit_dir(&self) -> PathBuf {
        crate::store::paths::grit_dir(&self.root)
    }
    pub fn objects_dir(&self) -> PathBuf {
        crate::store::paths::objects_dir(&self.root)
    }
}
