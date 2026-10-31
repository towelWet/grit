use super::types::RestorePlan;

pub(super) fn sort(plan: &mut RestorePlan) {
    plan.create.sort();
    plan.overwrite
        .sort_by_key(|path| std::cmp::Reverse(depth(path)));
    plan.remove
        .sort_by_key(|path| std::cmp::Reverse(depth(path)));
    plan.unchanged.sort();
}

fn depth(path: &str) -> usize {
    path.matches('/').count()
}
