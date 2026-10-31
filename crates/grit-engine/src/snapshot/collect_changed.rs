// collect_changed.rs — merge the four change categories into the single list recorded by the commit and journal.
// Why: the human output reports the categories separately, but history records one flat list of changed paths.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
pub(super) fn collect_changed(
    added: &[String],
    modified: &[String],
    deleted: &[String],
    renamed: &[String],
) -> Vec<String> {
    added
        .iter()
        .chain(modified)
        .chain(deleted)
        .chain(renamed) // renamed entries are "old -> new" labels, recorded as-is
        .cloned()
        .collect()
}
