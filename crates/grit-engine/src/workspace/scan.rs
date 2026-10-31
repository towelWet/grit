// scan.rs — walk the working tree and produce one WorkspaceEntry per path in scope.
// Why: every command that reads the disk (snapshot, status, diff, restore) goes through this single scan.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::read::read_entry;
use super::WorkspaceEntry;
use crate::ignore::IgnoreRules;
use crate::index::Index;
use crate::Result;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::path::Path;

mod collect_items;
mod included;
mod special;
mod symlinked_selection; // --path through a symlink → loud error

pub fn scan(
    root: &Path,
    ignore: &IgnoreRules,
    selections: Option<&[String]>,
    index: &Index, // stat cache: lets unchanged files skip the read
) -> Result<BTreeMap<String, WorkspaceEntry>> {
    symlinked_selection::symlinked_selection(root, selections)?; // no silent "(clean)" through links
    let items = collect_items::collect_items(root, ignore, selections)?; // sequential walk
    let described: Vec<_> = items
        .par_iter() // per-path work (stat, cache lookup, hashing) runs across cores
        .map(|item| {
            read_entry(root, item, index).map_err(|error| super::with_path(error, item.path()))
        }) // EPERM etc. must say which file
        .collect::<Result<_>>()?; // first error wins; order of results is preserved
    Ok(described.into_iter().flatten().collect()) // the repository root yields None and is dropped
}
