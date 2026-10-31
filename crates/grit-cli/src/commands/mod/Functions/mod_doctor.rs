fn doctor(root: &Path) -> CmdResult {
    let r = Repository::open(root)?;
    println!("Repo: {}", r.root.display());
    println!("  .grit: {}", r.grit_dir().exists());
    println!(
        "  HEAD: {}",
        refs::get_head(&r.root).as_deref().unwrap_or("(none)")
    );
    println!(
        "  Branch: {}",
        refs::get_current_branch(&r.root)
            .as_deref()
            .unwrap_or("(none)")
    );
    if let Ok(h) = refs::get_head_hash(&r.root) {
        if let Ok(c) = store::read::read_commit(&r.root, &h) {
            println!("  Latest: {} — {}", h.short(), c.message)
        } else {
            println!("  Latest: {} (error)", h.short())
        }
    }
    println!(
        "  Branches: {}",
        refs::list_branches(&r.root).map(|b| b.len()).unwrap_or(0)
    );
    println!(
        "  Tags: {}",
        refs::list_tags(&r.root).map(|t| t.len()).unwrap_or(0)
    );
    Ok(ExitCode::Success)
}
