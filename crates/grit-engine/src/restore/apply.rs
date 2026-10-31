use crate::store::read::CommitEntries;
use crate::Result;
use std::path::Path;

mod directories;
mod files;

pub fn entries(root: &Path, target: &CommitEntries, paths: Option<&[String]>) -> Result<()> {
    directories::apply(root, &target.entries, paths)?; // directories first so files have parents
    files::apply(root, target, paths) // files read through their stored object ids
}
