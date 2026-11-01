// sort.rs — one item split out of order.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub(in super::super::super) fn sort(plan: &mut RestorePlan) {
    plan.create.sort();
    plan.overwrite
        .sort_by_key(|path| std::cmp::Reverse(depth(path)));
    plan.remove
        .sort_by_key(|path| std::cmp::Reverse(depth(path)));
    plan.unchanged.sort();
}
