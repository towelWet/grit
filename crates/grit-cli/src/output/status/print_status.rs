// print_status.rs — one item split out of status.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn print_status(entries: &[StatusEntry]) {
    if entries.is_empty() {
        println!("(clean)");
        return;
    }
    for entry in entries {
        println!(" {} {}", marker(&entry.status), entry.path);
    }
}
