use grit_engine::status::StatusEntry;

pub fn print_status(entries: &[StatusEntry]) {
    if entries.is_empty() {
        println!("(clean)");
        return;
    }
    for entry in entries {
        println!(" {} {}", marker(&entry.status), entry.path);
    }
}

fn marker(status: &str) -> &str {
    match status {
        "modified" => "M",
        "added" => "A",
        "deleted" => "D",
        "renamed" => "R",
        "type-changed" => "T",
        "permission-changed" => "P",
        "symlink-changed" => "L",
        _ => "?",
    }
}
