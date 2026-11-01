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
mod grit_dir;
mod objects_dir;
