fn checkout(root: &Path, args: &GritArgs) -> CmdResult {
    let r = Repository::open(root)?;
    let n = args.branch_name.as_ref().ok_or("need branch name")?;
    let hash = refs::get_branch_ref(&r.root, n)?;
    let dirty = !status::compute_status(&r.root, &r.ignore, None)?.is_empty();
    if dirty && !args.force {
        return Err("working tree has changes; use --force to checkout".into());
    }
    restore::execute_restore(&restore::RestoreOptions {
        root: &r.root,
        commit_hash: &hash,
        paths: None,
        force: true,
        safety_save: true,
        author: &args.author,
    })?;
    refs::set_head(&r.root, &format!("ref: refs/heads/{n}"))?;
    println!("Switched to '{n}'");
    Ok(ExitCode::Success)
}
