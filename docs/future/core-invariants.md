# Core Invariants

1. The local immutable Grit graph is authoritative; no website or index is.
2. Atoms are content-addressed, verified, immutable, and stored exact-byte.
3. Draft mutations are journaled before an editing tool reports success.
4. Risky operations checkpoint first; undo and recovery are first-class.
5. Public workflow has no persistent staging area or detached state.
6. History is append-only; normal recovery never rewrites earlier Saves.
7. Each edit-capable actor writes through one owned Lane and Projection.
8. AI proposes; deterministic Rust validates, applies, journals, and restores.
9. Search, caches, queues, and forge databases are disposable projections.
10. Receiving, parsing, synchronizing, and importing objects never execute code.
11. Reviews approve one exact immutable Save and one exact target Main Save.
12. Main has one serialized writer and advances only by compare-and-swap.
13. A Combine preserves prior Main unless the approved change explicitly acts.
14. Native Grit work remains usable offline without Git or a forge account.
15. Native state and Git-bridge state remain separate and visibly identified.

Every invariant needs a deterministic test before its feature is declared
complete. A convenience feature that breaks one is rejected, not patched over.
