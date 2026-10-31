# Personal Incident Gates

These are future acceptance requirements derived from the user's real failures.
They are not claims about the current foundation.

| # | Failure | Required Grit behavior and test |
| --- | --- | --- |
| 1 | Worktree/build sprawl | Lanes share Atoms; projections and build caches are inventoried, quota-bound, and reclaimable without losing Saves |
| 2 | Stale tree held work | Closing archives the Lane; reachable or unreviewed Saves cannot be garbage-collected |
| 3 | Mechanism silently swapped | Task Contract and Receipts prove requested actors/Lanes; substitution requires an explicit event |
| 4 | Isolation failed, then vanished | Lane capability negotiation fails closed before writes; never degrade to a shared checkout |
| 5 | State used wrong root | Project ID and root-owned state bind every Projection; cross-project state is rejected visibly |
| 6 | External drive became EPERM | Report inaccessible, not clean; preserve last durable journal/checkpoint and delegate OS repair |
| 7 | New files and bulk format escaped | Dirty Draft includes new, modified, moved, removed, and ungrained files; scope expansion is previewed |
| 8 | Directory was never protected | Distinguish unprotected, inaccessible, and corrupt; never imply history exists without `.grit` |
| 9 | Git/Grit absence looked like denial | Typed discovery errors identify no Project versus permission failure |
| 10 | Ignore syntax was half cloned | Grit owns one specified, conformance-tested path-rule engine; bridges translate explicitly |
| 11 | Green work was never recorded | Journal before tool success; checkpoints and unsaved Draft state are visible to the next session |
| 12 | Ungrained source of truth risked clean | Ungrained files are journaled/recoverable; cleanup uses preview and protected trash generations |
| 13 | Two writers shared one checkout | One live writer lease per Projection; owner, heartbeat, task, and abandoned state are inspectable |
| 14 | Directory copy dropped branch work | Project derivation/import records ancestry and object reachability; `cp` is never version integration |
| 15 | Runtime reported stale revision | Build Receipt binds exact Save ID, source graph, artifact hash, and installed hash; runtime reports it |
| 16 | Gigabytes nearly entered history | Large/generated payload policy, size preview, quotas, and artifact objects block accidental source Saves |
| 17 | TauHost still had no history | Same protection gate as #8, surfaced continuously until resolved |
| 18 | Stranded branches were invisible | Lane inventory and doctor always show uncombined/reviewed/abandoned work; never auto-combine |
| 19 | Stale worktree could deploy | Deploy policy rejects artifacts not built from current approved Main or an explicitly approved Save |
| 20 | Main was force-moved | Main changes only by serialized CAS Combine or append-only reversal; no force-move command exists |
| 21 | Canonical branch was unknown | One explicit Main; every UI/runtime view shows Project, Main Save, active Lane, and artifact Receipt |
| 22 | Cherry-pick duplicated identity | Normal Combine preserves native object/provenance identity; exceptional Apply Save records equivalence |

The suite must also simulate index loss, drive disappearance, crashed writers,
stale Main, missing isolation support, oversized payloads, and copied directories.
