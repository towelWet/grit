// mod_tag_list.rs — `grit tag`: list tags with their target ids, in human or TOML form.
// Why: automation needs each tag's full id, which the human short form does not carry.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
fn tag_list(root: &Path, fmt: &str) -> CmdResult {
    let r = Repository::open(root)?;
    let mut tags = Vec::new();
    for t in refs::list_tags(&r.root)? {
        if let Ok(h) = refs::get_tag(&r.root, &t) {
            tags.push((t, h)); // skip tags whose ref cannot be read, as the human form always did
        }
    }
    if fmt == "toml" {
        let entries: Vec<_> = tags
            .iter()
            .map(|(name, hash)| serde_json::json!({"name": name, "hash": hash.hex()}))
            .collect();
        println!("{}", report::format_toml(&serde_json::json!({"tags": entries})));
        return Ok(ExitCode::Success);
    }
    for (name, hash) in &tags {
        println!("{name}  {}", hash.short());
    }
    Ok(ExitCode::Success)
}
