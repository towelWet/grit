// acquire.rs — one item split out of acquire.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl Lock {
    pub fn acquire(root: &Path) -> Result<Self> {
        let path = lock_path(root);
        if let Ok(owner) = fs::read_to_string(&path) {
            if owner.trim().parse::<u32>().is_ok_and(process_is_live) {
                return Err(GritError::Locked(owner.trim().parse().unwrap()));
            }
            fs::remove_file(&path)?;
        }
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(lock_error)?;
        write!(file, "{}", std::process::id())?;
        file.sync_all()?;
        Ok(Self {
            path,
            acquired: true,
        })
    }
}
