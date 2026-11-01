// mod_dispatch.rs — route each parsed command to its handler, passing the output format where it applies.
// Why: every command that can emit `--format toml` must receive the format, or scripts silently get human text.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
pub fn dispatch(args: &GritArgs) -> CmdResult {
    let root = resolve_root(args);
    let root = root.as_path();
    match &args.command {
        Command::Init => init(root),
        Command::Status => status(root, &args.format, &args.paths),
        Command::Diff => diff(root, &args.snapshot_a, &args.snapshot_b, &args.format),
        Command::Save => save(root, args),
        Command::Log => log(root, args.limit, &args.format),
        Command::Show => show(root, &args.id, &args.format),
        Command::Undo => undo(root, &args.format),
        Command::Restore => restore(root, args),
        Command::Checkout => checkout(root, args),
        Command::Branch {
            sub: BranchSub::List,
        } => branch_list(root, &args.format),
        Command::Branch {
            sub: BranchSub::Create { name },
        } => branch_create(root, name, &args.format),
        Command::Tag { sub: TagSub::List } => tag_list(root, &args.format),
        Command::Tag {
            sub: TagSub::Create { name, hash },
        } => tag_create(root, name, hash, &args.format),
        Command::Verify => vfy(root, args.full, &args.format),
        Command::Gc => gc(root),
        Command::Doctor => doctor(root),
    }
}
