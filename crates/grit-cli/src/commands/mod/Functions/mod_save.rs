// mod_save.rs — `grit snapshot` / `grit save`: record the working tree, or report that nothing changed.
// Why: a clean tree must not grow history, so the human and TOML outputs both say when no commit was made.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
fn save(root: &Path, args: &GritArgs) -> CmdResult {
    let repository = Repository::open(root)?;
    let parent = refs::get_head_hash(&repository.root).ok().map(|hash| hash.hex()); // HEAD before the save
    let metadata = args.metadata.iter().filter_map(|item| {
        item.split_once('=').map(|(key, value)| (key.into(), value.into()))
    }).collect();
    let transaction = transactions::begin(&repository.root, "save")?;
    let result = snapshot::create_snapshot(&snapshot::SnapshotOptions {
        root: &repository.root,
        author: &args.author,
        message: args.message.as_deref().unwrap_or("save"),
        paths: (!args.paths.is_empty()).then_some(args.paths.as_slice()),
        ignore: &repository.ignore,
        metadata: &metadata,
    })?;
    transactions::commit(&repository.root, &transaction)?;
    if args.format == "toml" {
        print_snapshot_report(&repository, result, transaction, parent);
    } else if let Some(hash) = result.commit_hash {
        println!("Saved as {} ({} files)", hash.short(), result.changed_files.len());
    } else {
        println!("Nothing to save (clean)"); // no commit was written, so there is no id to print
    }
    Ok(ExitCode::Success)
}
