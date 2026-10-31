// object_stats_add.rs — combine per-entry object counters from parallel workers.
// Why: parallel stores each count their own bytes, and the totals must be the sum with no shared mutable state.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::types::ObjectStats;
use std::ops::Add;

impl Add for ObjectStats {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            count: self.count + other.count,     // objects newly written
            added: self.added + other.added,     // bytes of those new objects
            reused: self.reused + other.reused,  // bytes already present and skipped
        }
    }
}
