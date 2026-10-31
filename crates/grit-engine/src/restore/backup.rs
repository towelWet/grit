use crate::workspace::validate_relative;
use crate::Result;
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};

pub fn root(repository: &Path) -> PathBuf {
    crate::store::paths::tmp_dir(repository)
        .join("restore-backups")
        .join(Utc::now().format("%Y%m%d%H%M%S%f").to_string())
}

pub fn move_aside(root: &Path, relative: &str, backup: &Path) -> Result<()> {
    validate_relative(relative)?;
    let source = root.join(relative);
    if fs::symlink_metadata(&source).is_err() {
        return Ok(());
    }
    let destination = backup.join(relative);
    fs::create_dir_all(destination.parent().unwrap())?;
    if fs::symlink_metadata(&source)?.is_dir() && destination.exists() {
        fs::rename(source, destination.join(".grit-directory"))?;
    } else {
        fs::rename(source, destination)?;
    }
    Ok(())
}
