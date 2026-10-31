use crate::Hash;
use std::path::Path;

pub struct RestoreOptions<'a> {
    pub root: &'a Path,
    pub commit_hash: &'a Hash,
    pub paths: Option<&'a [String]>,
    pub force: bool,
    pub safety_save: bool,
    pub author: &'a str,
}

pub struct RestorePlan {
    pub create: Vec<String>,
    pub overwrite: Vec<String>,
    pub remove: Vec<String>,
    pub unchanged: Vec<String>,
}

impl RestorePlan {
    pub fn changed(&self) -> bool {
        !self.create.is_empty() || !self.overwrite.is_empty() || !self.remove.is_empty()
    }
}
