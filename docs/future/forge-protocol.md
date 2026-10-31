# Grit Forge Protocol

The protocol is the product boundary; GritHub is one optional implementation.

A compatible forge may offer discovery, web rendering, search, compute,
notifications, artifact delivery, profiles, federation, and object relay. It
may cache the graph but cannot redefine Main, approvals, identities, Issues,
Reviews, Releases, or Receipts.

Protocol operations exchange content-addressed objects, signed events,
capability-scoped requests, inclusion proofs, and resumable bounded streams.
Clients verify all objects locally. Remote input lands in quarantine and remote
indexes are hints, never proof of existence or absence.

Projects remain usable if every forge is offline. The same graph can move among
peers, self-hosted services, Codeberg-like communities, and commercial forges
without changing native IDs or history.

Identity is portable and cryptographic. Handles and profiles are aliases over
keys, with rotation and revocation events. Automated actors are labeled and
delegated narrowly; they cannot impersonate a human approval.

Training and reuse require explicit project policy recorded in the graph.
Hosting, indexing, or running CI grants no implicit right to train on private or
public project data.
