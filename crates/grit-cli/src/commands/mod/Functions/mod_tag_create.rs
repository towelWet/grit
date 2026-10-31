// mod_tag_create.rs — `grit tag create NAME [REV]`: name a snapshot, in human or TOML form.
// Why: a tag is a durable name for a state, so the lock serializes it with other ref writers and TOML callers get the full id.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
fn tag_create(root: &Path, name: &str, hstr: &str, fmt: &str) -> CmdResult {
    let r = Repository::open(root)?;
    let _lock = store::lock::Lock::acquire(&r.root)?; // serializes the ref update with other writers
    let hash = if hstr.is_empty() {
        refs::get_head_hash(&r.root)? // no revision given: tag the current snapshot
    } else {
        refs::resolve_revision(&r.root, hstr)?
    };
    refs::set_tag(&r.root, name, &hash)?;
    if fmt == "toml" {
        println!("{}", report::format_toml(&serde_json::json!({
            "name": name,
            "hash": hash.hex(), // full id, so scripts can use it without another lookup
        })));
    } else {
        println!("Tag '{name}' at {}", hash.short());
    }
    Ok(ExitCode::Success)
}
