# Durable Object Model

The canonical graph stores development facts; mutable views are derived.

| Object | Durable meaning |
| --- | --- |
| Atom | Exact content bytes addressed by BLAKE3 |
| Grain | Stable semantic unit with parser and fallback identity |
| Save | Immutable project state plus parents and provenance |
| Lane | Owned writable overlay, goal, policy, and validation state |
| Journal Event | Append-only attempted/applied/rejected operation |
| Review Request | Exact proposal Save, target Save, policy, and state events |
| Issue | Durable problem statement and lifecycle events |
| Comment | Signed discussion event attached to another object |
| Receipt | Signed evidence for an exact Save, command, inputs, and outputs |
| Release | Named Save, artifacts, receipts, notes, and signatures |
| Identity | Portable cryptographic actor identity and delegated keys |
| Link | Optional peer or forge endpoint, never an authority pointer |

Approvals, edits, closures, and revocations append signed events; they do not
silently mutate database rows. Indexes rebuild from the graph. Deleting a UI
record cannot make a Review, Issue, Receipt, or Release disappear.

Every object schema is versioned, canonically encoded, size-bounded, and
hash-verified before graph admission. Unknown versions fail closed.
