use super::Repository;
use crate::config::GritConfig;
use crate::ignore::IgnoreRules;
use crate::store::paths::*;
use crate::{GritError, Result};
use std::fs;
use std::path::Path;

impl Repository {
    pub fn init(path: &Path) -> Result<Self> {
        let root = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let grit = grit_dir(&root);
        if grit.exists() {
            return Err(GritError::AlreadyInitialized(root.display().to_string()));
        }
        let directories = [
            grit.clone(),
            objects_dir(&root),
            heads_dir(&root),
            tags_dir(&root),
            snapshots_dir(&root),
            journal_dir(&root),
            transactions_dir(&root),
            tmp_dir(&root),
            logs_dir(&root),
            grit.join("index"),
        ];
        for directory in directories {
            fs::create_dir_all(directory)?;
        }
        let config = GritConfig::default();
        config.save(&root)?;
        crate::refs::set_head(&root, "ref: refs/heads/main")?;
        let ignore = IgnoreRules::new(&config.save.exclude);
        Ok(Self {
            root,
            config,
            ignore,
        })
    }
}
