// mod_restore_toml.rs — `grit restore --format toml`: one TOML report with the plan and whether it ran.
// Why: scripts parse stdout as a single document, so the plan, the outcome, and the target id go into one report.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
fn restore_toml(opts: &restore::RestoreOptions, plan: &restore::RestorePlan, hash: &Hash) -> CmdResult {
    let nothing = plan.create.is_empty() && plan.overwrite.is_empty() && plan.remove.is_empty();
    let runs = !nothing && opts.force; // a plan with changes runs only when forced
    if runs {
        restore::execute_restore(opts)?; // runs before printing so the report states the real outcome
    }
    println!("{}", report::format_toml(&serde_json::json!({
        "create": plan.create,
        "overwrite": plan.overwrite,
        "remove": plan.remove,
        "unchanged": plan.unchanged,
        "restored": runs, // false for a dry plan, a clean tree, or a refused run
        "snapshot_id": hash.hex(), // the target snapshot, full id
    })));
    if !nothing && !opts.force {
        return Err("restore would change files; inspect the plan and rerun with --force".into()); // stderr; stdout stays valid TOML
    }
    Ok(ExitCode::Success)
}
