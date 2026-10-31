# Combine and Network Ingestion

## Transactional Combine

1. Bind approval to proposal Save `P` and expected Main Save `M`.
2. Recompute the candidate from immutable objects; never trust queue state.
3. Resolve only true Grain Overlaps and record every explicit removal.
4. Verify the complete candidate tree and required signed Receipts.
5. Prove all prior-Main Grains are unchanged unless `P` explicitly acts on them.
6. Compare-and-swap Main from `M` to the verified result.
7. If Main changed, fail stale; update the Lane and review the new exact Save.
8. Append the Combine event only after the atomic Main update succeeds.

## Non-executable receipt path

Network bytes pass through bounded decode, canonical validation, hashing,
quarantine storage, graph validation, identity authorization, then admission.
No receive option, object metadata, filename, parser field, or imported project
can invoke a hook, shell, plugin, build, model, or executable.

Triggers are a separate policy subsystem. They consume admitted object IDs in
sandboxed workers with explicit commands, resource limits, capabilities, and
signed Receipts. A failed or unavailable runner cannot erase review state.
