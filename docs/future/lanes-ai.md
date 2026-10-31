# Lanes, Grains, and AI

A Lane is a private writable overlay for one task, experiment, human, or agent.
It is not a branch checkout, worktree, copied dependency tree, or user-managed
folder. Unchanged Atoms stay shared; a Projection materializes the active view.

Each Lane records owner, agent, goal, base Save, Task Contract, read set, write
set, touched Grains, leases, checkpoints, build/test Receipts, review state, and
last activity. A writer cannot mutate Main, another Lane, or paths outside its
contract. Preimage validation rejects stale or silently expanded writes.

Grains are the concurrency boundary. Parsers identify functions, methods,
types, config sections, regions, and other semantic units. Independent Grains
combine commutatively even within one file. Same-Grain edits create a structured
Overlap. Unsupported formats fall back to explicit regions or whole artifacts.

An AI command receives a bounded context bundle and returns a typed proposal.
It cannot write the object store, journal, history, Lane metadata, projection,
provenance, or Receipts. Deterministic code applies or rejects the proposal and
records both outcome and actor identity.

AI-generated explanations never become authority. Policies, verification, and
signed evidence decide whether a proposal may become a Save or enter Main.
