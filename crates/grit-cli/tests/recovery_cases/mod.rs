// mod.rs — the recovery test suite: one module per behaviour, shared helpers in support.
// Why: each file owns one guarantee, so a failure names the broken promise directly.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod commands;
mod empty;
mod line_limit;
mod lock;
mod no_op_snapshot;
mod stall_edits;
mod stall_ignore;
mod stall_verify;
mod partial_scan;
mod round_trip;
mod safety_assertions;
mod safety_round_trip;
mod scenario_a;
mod scenario_b;
mod support;
mod toml_contract;
mod toml_report;
mod toml_restore;
mod transaction;
