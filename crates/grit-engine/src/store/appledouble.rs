// appledouble — recognise macOS "._name" sidecar files inside .grit/.
// Why: on exFAT/FAT/SMB volumes macOS stores xattrs in a "._<name>" file next to
// each real file, so .grit/objects gained one "._xx" per object dir and verify
// reported thousands of "corrupt" objects that are really Finder metadata.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use std::path::Path;

pub fn is_appledouble(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with("._")) // grit never writes names starting "._"
}
