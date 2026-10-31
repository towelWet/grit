fn gc(root: &Path) -> CmdResult {
    let r = Repository::open(root)?;
    let res = gc::gc(&r.root)?;
    println!(
        "GC: removed {} ({} freed), kept {}",
        res.removed, res.bytes_freed, res.kept
    );
    Ok(ExitCode::Success)
}
