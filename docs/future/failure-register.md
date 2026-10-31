# Failure Register

| Observed failure | Grit prohibition and acceptance gate |
| --- | --- |
| Hidden HEAD/index/rebase state | Daily workflow exposes no hidden state machine |
| Lost unsaved work | Journal-before-success and checkpoint recovery tests |
| Destructive reset/history rewrite | Append-only reversal and recoverable trash |
| File/line merge noise | Grain commutativity and structured Overlap tests |
| Shared agent checkout corruption | One scoped Lane/Projection per writer |
| Merge queue reverts earlier work | Preservation proof plus CAS Main test |
| PR or Issue appears missing | Objects survive total index deletion and rebuild |
| Hosted Actions outage blocks review | Local review and portable signed Receipts |
| Push/receive metadata reaches shell | Fuzzed receipt path proves zero execution |
| Hosting compromise changes Main | Quarantine and local signature/policy checks |
| Agent API traffic overloads forge | Local Lane work, deduplicated object sync |
| Central identity/provider disappears | Portable keys, rotation, and forge migration |
| Large or binary file workaround | Chunked first-class Atoms and bounded transfer |
| AI hallucinates a change or result | Proposal-only AI and signed deterministic proof |
| Search/database becomes truth | Full graph reconstruction without forge database |
| Vendor uses code for training | Explicit signed data-use policy, deny by default |

New Git, GitHub, agent, CI, and forge failures are added here before design work
so their root class becomes an invariant and adversarial test, not a slogan.
