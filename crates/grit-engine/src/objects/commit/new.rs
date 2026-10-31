// new.rs — build a commit stamped with the current format and the current time.
// Why: every commit written by this binary must declare FORMAT_NESTED so readers use the nested tree layout.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::{Commit, FORMAT_NESTED};
use crate::hash::Hash;
use chrono::Utc;

impl Commit {
    pub fn new(rt: Hash, parents: Vec<Hash>, author: &str, msg: &str) -> Self {
        Self {
            root_tree: rt,
            parents,
            author: author.into(),
            message: msg.into(),
            timestamp: Utc::now(), // wall clock at creation; used only for display and log order
            branch: None,
            metadata: std::collections::BTreeMap::new(),
            format: FORMAT_NESTED, // rt must name a nested root tree for this format
            changed_paths: Vec::new(), // the snapshot fills this with the paths it changed
        }
    }
}
