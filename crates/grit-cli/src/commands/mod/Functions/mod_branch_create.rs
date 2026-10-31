// mod_branch_create.rs — `grit branch create NAME`: point a new branch at HEAD, in human or TOML form.
// Why: the lock keeps two writers from racing on the same ref, and TOML callers need the name and target id.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
fn branch_create(root: &Path, name: &str, fmt: &str) -> CmdResult {
    let r = Repository::open(root)?;
    let _lock = store::lock::Lock::acquire(&r.root)?; // serializes the ref update with other writers
    if refs::get_branch_ref(&r.root, name).is_ok() {
        return Err(format!("branch already exists: {name}").into());
    }
    let h = refs::get_head_hash(&r.root)?;
    refs::set_branch_ref(&r.root, name, &h)?;
    if fmt == "toml" {
        println!("{}", report::format_toml(&serde_json::json!({
            "created": name,
            "hash": h.hex(), // full id; the human form shows only the short id
        })));
    } else {
        println!("Branch '{name}' at {}", h.short());
    }
    Ok(ExitCode::Success)
}
