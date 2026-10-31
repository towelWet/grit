# Grit Future Contract

Grit will be a local-first, cryptographically durable development substrate.
It distributes the whole development record, not only source snapshots. A
forge may present or relay that record, but never owns its truth.

This contract supersedes Git-shaped assumptions in the current foundation.
Current implementation claims remain in [README.md](README.md); specification
is not presented as shipped behavior.

## Canonical documents

- [Core invariants](docs/future/core-invariants.md)
- [Durable object model](docs/future/objects.md)
- [Lanes, Grains, and AI](docs/future/lanes-ai.md)
- [Combine and network ingestion](docs/future/combine-receive.md)
- [Grit Forge Protocol](docs/future/forge-protocol.md)
- [Failure register](docs/future/failure-register.md)
- [Personal incident gates](docs/future/personal-incidents.md)
- [Delivery sequence](docs/future/roadmap.md)

## Daily vocabulary

Normal users see Project, Main, Lane, Draft, Save, Timeline, Compare, Undo,
Restore, Shelf, Review Request, Receipt, and Link. They do not manage HEAD,
detached state, staging, rebase, refs, worktrees, or force-push mechanics.

`GritHub` remains a useful codename for one hosted product. It must implement
the open Forge Protocol and remain replaceable by another compatible service.
