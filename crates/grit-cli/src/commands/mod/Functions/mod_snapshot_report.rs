// mod_snapshot_report.rs — print the `--format toml` report for a save, including the no-commit case.
// Why: scripts need `saved`, `id`, and `files` on every save, and `snapshot_id` must still name the current snapshot.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
fn print_snapshot_report(repository: &Repository, result: snapshot::SnapshotResult, transaction: String, parent: Option<String>) {
    let saved_id = result.commit_hash.map(|hash| hash.hex()); // Some only when a commit was written
    let saved = saved_id.is_some();
    let files = result.changed_files.len() as u64; // same count the human output reports
    let snapshot_id = saved_id.clone().or_else(|| parent.clone()).unwrap_or_default(); // HEAD after the command
    println!("{}", report::format_toml(&report::SnapshotReport {
        repository_root: repository.root.display().to_string(),
        snapshot_id,
        parent_snapshot_id: parent,
        branch: refs::get_current_branch(&repository.root).unwrap_or_default(),
        changed_files: result.changed_files,
        added_files: result.added,
        modified_files: result.modified,
        deleted_files: result.deleted,
        renamed_files: result.renamed,
        object_count: result.object_count,
        bytes_added: result.bytes_added,
        bytes_reused: result.bytes_reused,
        verification_status: "ok".into(),
        transaction_id: transaction,
        saved,
        id: saved_id,
        files,
    }));
}
