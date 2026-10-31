// describe.rs — pick the right description for one path based on what it is on disk.
// Why: files, directories, and symlinks need different identities, so each kind has its own builder.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod directory_entry;
mod file_entry;
mod symlink_entry;

use crate::index::Index;
use crate::workspace::metadata::file_mode;
use crate::workspace::WorkspaceEntry;
use crate::{GritError, Result};
use std::fs::Metadata;
use std::path::Path;

pub(super) fn describe(
    abs: &Path,
    path: &str,
    metadata: &Metadata,
    index: &Index,
) -> Result<WorkspaceEntry> {
    let mode = file_mode(metadata);
    if metadata.file_type().is_symlink() {
        return symlink_entry::symlink_entry(abs, mode); // checked first: is_dir/is_file follow links
    }
    if metadata.is_dir() {
        return directory_entry::directory_entry(mode);
    }
    if metadata.is_file() {
        return file_entry::file_entry(abs, path, metadata, index, mode);
    }
    Err(GritError::InvalidObject(
        "unsupported filesystem entry".into(), // sockets, devices, FIFOs have no snapshot form
    ))
}
