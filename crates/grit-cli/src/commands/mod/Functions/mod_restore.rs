// mod_restore.rs — `grit restore REV`: print the plan, then apply it when --force is given.
// Why: restores are destructive, so the plan is shown first; TOML output is delegated so stdout stays one document.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
fn restore(root: &Path, args: &GritArgs) -> CmdResult {
    let r = Repository::open(root)?;
    let id = args.id.as_ref().ok_or("need snapshot id")?;
    let hash = refs::resolve_revision(&r.root, id)?;
    let opts = restore::RestoreOptions {
        root: &r.root,
        commit_hash: &hash,
        paths: if args.paths.is_empty() {
            None
        } else {
            Some(&args.paths)
        },
        force: args.force,
        safety_save: !args.no_safety_save,
        author: &args.author,
    };
    let plan = restore::plan_restore(&opts)?;
    if args.format == "toml" {
        return restore_toml(&opts, &plan, &hash); // one TOML document; no human lines mixed in
    }
    println!("Restore plan for {}:", hash.short());
    for p in &plan.create {
        println!("  create  {p}")
    }
    for p in &plan.overwrite {
        println!("  modify  {p}")
    }
    for p in &plan.remove {
        println!("  remove  {p}")
    }
    if plan.overwrite.is_empty() && plan.remove.is_empty() && plan.create.is_empty() {
        println!("Nothing to restore");
        return Ok(ExitCode::Success);
    }
    if !args.force {
        return Err("restore would change files; inspect the plan and rerun with --force".into());
    }
    restore::execute_restore(&opts)?;
    println!("Restored {}", hash.short());
    Ok(ExitCode::Success)
}
