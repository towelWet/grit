mod generated;

use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::Path;
#[derive(Debug, Clone)]
pub struct IgnoreRules {
    inner: GlobSet,
    prefixes: Vec<String>,
}
mod new;
mod empty;
mod is_ignored;
