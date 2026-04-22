# THP-TCP v1.0.3 Addendum

Related specs:
- AI Harmony Contract: `docs/spec/THP-TCP-AI-Harmony-Contract-v1.0.md`
- Three-Layer Model: `docs/spec/THP-TCP-Three-Layer-Model-v1.0.md`
- Phase0 Frame+Flags: `docs/spec/THP-TCP-Phase0-Frame+Flags-v1.0.md`
- CBOR Field Tables: `docs/spec/THP-TCP-CBOR-FieldTables-v1.0.md`

Status: Normative

This addendum defines peer admission semantics for READ / SYNC / LIVE. It does not change the Phase0 frame layout.

---

## 0. Universal AI Usability Requirement

READ / SYNC / LIVE exists so any capable AI implementation can know how to proceed with minimum load, model-neutral clarity, and without unstructured explanation.

Implementations MUST make protocol state, required knowledge, deltas, and repair needs explicit in fields. They MUST NOT rely on tone, implied social context, or model-specific hidden assumptions for protocol control.

---

## 1. Admission State Is Required

Before normal semantic traffic, an implementation MUST classify the peer into one of:

- READ_REQUIRED
- SYNC_REQUIRED
- LIVE_ACCEPTED

The local node MUST NOT send contextual tokens or LIVE semantic deltas to a peer that has not reached LIVE_ACCEPTED.

---

## 2. READ_REQUIRED

READ_REQUIRED applies when the peer does not prove knowledge of the required THP-TCP baseline.

The local node SHOULD send `READ_THIS` and then stop explanatory protocol traffic until the peer returns a valid acknowledgement.

`READ_THIS` MUST carry verifiable references and hashes. It MUST NOT carry a tutorial.

---

## 3. SYNC_REQUIRED

SYNC_REQUIRED applies when the peer proves a known baseline but not the current baseline.

The local node SHOULD send `SYNC_DELTA`.

The delta MUST be bounded to the smallest sufficient update set. Full specification retransmission is allowed only if the peer's baseline is unknown, unavailable, corrupted, or explicitly rejected.

---

## 4. LIVE_ACCEPTED

LIVE_ACCEPTED applies when the peer proves current protocol, token table, CBOR table, and dictionary hashes.

In LIVE_ACCEPTED, the preferred semantic traffic token is `LIVE_STATE_DELTA` for state-level communication.

Unstructured text MAY appear in `domain_payload` only when the task domain itself requires it.

---

## 5. Versioning

v1.0.3 is append-only:

- It adds admission semantics.
- It assigns no replacement meaning to existing tokens.
- It relies on v1.0.2 frame and fragmentation behavior.
- It relies on v1.0 CBOR field table policy.
