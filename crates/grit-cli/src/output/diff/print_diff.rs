// print_diff.rs — one item split out of diff.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn print_diff(diff: &DiffResult) {
    if diff.entries.is_empty() {
        println!("(no changes)");
        return;
    }
    for entry in &diff.entries {
        println!(" {} {}", marker(&entry.status), entry.path);
        if let Some(patch) = &entry.patch {
            print!("{patch}");
        } else if let (Some(old), Some(new)) = (entry.old_size, entry.new_size) {
            println!("   binary: {old} -> {new} bytes");
        }
    }
    println!(
        "\n {} files changed, {} insertions(+), {} deletions(-)",
        diff.files_changed, diff.lines_added, diff.lines_deleted
    );
}
