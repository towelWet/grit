// now_ns.rs — the current time in nanoseconds since the Unix epoch.
// Why: the racy-timestamp guard compares file mtimes against the moment a scan began, in the same units.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since| since.as_nanos() as u64)
        .unwrap_or(0) // clock before 1970: zero makes the guard refuse every cached entry
}
