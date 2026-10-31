fn diff(root: &Path, a: &Option<String>, b: &Option<String>, fmt: &str) -> CmdResult {
    let r = Repository::open(root)?;
    match (a, b) {
        (Some(ai), Some(bi)) => {
            let ah = refs::resolve_revision(&r.root, ai)?;
            let bh = refs::resolve_revision(&r.root, bi)?;
            let d = diff::diff_commits(&r.root, &ah, &bh)?;
            if fmt == "toml" {
                println!("{}", report::format_toml(&d))
            } else {
                output::print_diff(&d);
            }
        }
        _ => {
            let h = match refs::get_head_hash(&r.root) {
                Ok(h) => h,
                Err(_) => {
                    println!("(no commits)");
                    return Ok(ExitCode::Success);
                }
            };
            let d = diff::diff_working_tree(&r.root, &h, &r.ignore)?;
            if fmt == "toml" {
                println!("{}", report::format_toml(&d))
            } else {
                output::print_diff(&d);
            }
        }
    }
    Ok(ExitCode::Success)
}
