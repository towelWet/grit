fn status(root: &Path, fmt: &str, paths: &[String]) -> CmdResult {
    let r = Repository::open(root)?;
    let scope = (!paths.is_empty()).then_some(paths); // same --path contract as snapshot
    let st = status::compute_status(&r.root, &r.ignore, scope)?;
    if fmt == "toml" {
        println!(
            "{}",
            report::format_toml(&report::StatusReport {
                repository_root: r.root.display().to_string(),
                branch: refs::get_current_branch(&r.root).unwrap_or_default(),
                changed_files: st.iter().map(|s| s.path.clone()).collect(),
                added_files: st
                    .iter()
                    .filter(|s| s.status == "added")
                    .map(|s| s.path.clone())
                    .collect(),
                modified_files: st
                    .iter()
                    .filter(|s| s.status == "modified")
                    .map(|s| s.path.clone())
                    .collect(),
                deleted_files: st
                    .iter()
                    .filter(|s| s.status == "deleted")
                    .map(|s| s.path.clone())
                    .collect(),
                renamed_files: st
                    .iter()
                    .filter(|s| s.status == "renamed")
                    .map(|s| s.path.clone())
                    .collect()
            })
        )
    } else {
        output::print_status(&st);
    }
    Ok(ExitCode::Success)
}
