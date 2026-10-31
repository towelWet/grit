use std::path::{Path, PathBuf};
pub const GRIT_DIR: &str = ".grit";
pub fn grit_dir(r: &Path) -> PathBuf {
    r.join(GRIT_DIR)
}
pub fn objects_dir(r: &Path) -> PathBuf {
    grit_dir(r).join("objects")
}
pub fn refs_dir(r: &Path) -> PathBuf {
    grit_dir(r).join("refs")
}
pub fn heads_dir(r: &Path) -> PathBuf {
    refs_dir(r).join("heads")
}
pub fn tags_dir(r: &Path) -> PathBuf {
    refs_dir(r).join("tags")
}
pub fn snapshots_dir(r: &Path) -> PathBuf {
    refs_dir(r).join("snapshots")
}
pub fn journal_dir(r: &Path) -> PathBuf {
    grit_dir(r).join("journal")
}
pub fn transactions_dir(r: &Path) -> PathBuf {
    grit_dir(r).join("transactions")
}
pub fn tmp_dir(r: &Path) -> PathBuf {
    grit_dir(r).join("tmp")
}
pub fn logs_dir(r: &Path) -> PathBuf {
    grit_dir(r).join("logs")
}
pub fn config_path(r: &Path) -> PathBuf {
    grit_dir(r).join("config.toml")
}
pub fn head_path(r: &Path) -> PathBuf {
    grit_dir(r).join("HEAD")
}
pub fn lock_path(r: &Path) -> PathBuf {
    grit_dir(r).join("lock")
}
