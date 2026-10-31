// pseudo_random_bytes.rs — deterministic, incompressible-looking bytes from a seed.
// Why: tests need repeatable input, and random-looking data exposes any per-byte storage overhead.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
pub fn pseudo_random_bytes(length: usize, seed: u64) -> Vec<u8> {
    // xorshift64: deterministic and incompressible enough to expose any per-byte overhead.
    let mut state = seed.max(1); // xorshift must never hold zero
    (0..length)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state & 0xff) as u8 // low byte of each step; full-range output
        })
        .collect()
}
