use super::Repository;
use crate::config::GritConfig;
use crate::ignore::IgnoreRules;
use crate::Result;
use std::path::Path;

impl Repository {
    pub fn open(path: &Path) -> Result<Self> {
        let root = super::find::root(path)?;
        let config = GritConfig::load(&root)?;
        let ignore = IgnoreRules::new(&config.save.exclude);
        Ok(Self {
            root,
            config,
            ignore,
        })
    }
}
