// mod_branch_list.rs — `grit branch`: list branches, marking the current one, in human or TOML form.
// Why: automation needs to know which branch is current without parsing the "* " marker.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
fn branch_list(root: &Path, fmt: &str) -> CmdResult {
    let r = Repository::open(root)?;
    let branches = refs::list_branches(&r.root)?;
    let cur = refs::get_current_branch(&r.root).ok();
    if fmt == "toml" {
        println!("{}", report::format_toml(&serde_json::json!({
            "current": cur.clone().unwrap_or_default(), // empty when HEAD names no branch
            "branches": branches,
        })));
        return Ok(ExitCode::Success);
    }
    for b in &branches {
        println!(
            "{}{b}",
            if Some(b.as_str()) == cur.as_deref() {
                "* "
            } else {
                "  "
            }
        )
    }
    Ok(ExitCode::Success)
}
