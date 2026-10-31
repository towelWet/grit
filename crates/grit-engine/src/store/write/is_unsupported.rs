// is_unsupported — does an I/O error mean the volume cannot do no-clobber renames?
// Why: exFAT/FAT volumes reject RENAME_EXCL with ENOTSUP, which std maps to ErrorKind::Unsupported; the raw code is checked too.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.

pub(super) fn is_unsupported(error: &std::io::Error) -> bool {
    error.kind() == std::io::ErrorKind::Unsupported           // std maps ENOTSUP here
        || error.raw_os_error() == Some(45)                   // macOS ENOTSUP/EOPNOTSUPP raw code
}
