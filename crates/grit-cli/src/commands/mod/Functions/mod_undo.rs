fn undo(root: &Path, _fmt: &str) -> CmdResult {
    let r = Repository::open(root)?;
    let e = journal::read_journal(&r.root, 2)?;
    if e.len() < 2 {
        return Err("Nothing to undo".into());
    }
    let h = e[1]["hash"].as_str().ok_or("no hash")?;
    let hash = refs::resolve_revision(&r.root, h)?;
    restore::execute_restore(&restore::RestoreOptions {
        root: &r.root,
        commit_hash: &hash,
        paths: None,
        force: true,
        safety_save: false,
        author: "undo",
    })?;
    println!("Undone");
    Ok(ExitCode::Success)
}
