// mod_vfy.rs — `grit verify`: check every stored object and report counts, in human or TOML form.
// Why: automation needs the same counts as structured keys, and a nonzero corrupt count must still fail the command.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
fn vfy(root: &Path, fmt: &str) -> CmdResult {
    let r = Repository::open(root)?;
    let res = verify::verify(&r.root)?;
    if fmt == "toml" {
        println!("{}", report::format_toml(&serde_json::json!({
            "total_objects": res.total_objects,
            "valid": res.valid,
            "corrupt": res.corrupt,
            "reachable": res.reachable,
            "unreachable": res.unreachable,
            "ok": res.corrupt == 0, // mirrors the exit code: false means the command returns DataErr
            "errors": res.errors,
        })));
    } else {
        println!(
            "Objects: {} total, {} valid, {} corrupt. Reachable: {}, Orphans: {}",
            res.total_objects, res.valid, res.corrupt, res.reachable, res.unreachable
        );
        for e in &res.errors {
            eprintln!("  corrupt: {e}") // human form sends details to stderr, as before
        }
    }
    Ok(if res.corrupt > 0 {
        ExitCode::DataErr
    } else {
        ExitCode::Success
    })
}
