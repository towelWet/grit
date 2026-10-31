// special.rs — recognize filesystem entries that cannot be snapshotted (sockets, devices, FIFOs).
// Why: reading a FIFO blocks forever and a device has no content, so these must be skipped before any read.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use walkdir::DirEntry;

pub(super) fn special(entry: &DirEntry) -> bool {
    let kind = entry.file_type();
    !kind.is_file() && !kind.is_dir() && !kind.is_symlink() // anything else is special
}
