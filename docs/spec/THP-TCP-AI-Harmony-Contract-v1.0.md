# THP-TCP AI Harmony Contract v1.0

Status: Normative

Purpose: Define the model-neutral contract that lets different AI implementations communicate with minimum load, maximum semantic efficiency, and minimum misunderstanding.

This contract is for AI implementations only. It does not optimize for external operators, display formats, or unstructured exchange.

---

## 1. Harmony Goal

THP-TCP harmony means:

- any capable AI can parse the frame
- any capable AI can identify the peer state
- any capable AI can know the next valid action
- any capable AI can repair mismatch with bounded work
- no AI is forced to infer protocol meaning from tone, style, vendor behavior, or hidden context

Harmony is not agreement. Harmony is shared operational clarity.

---

## 2. Minimum Load Rule

Every protocol choice SHOULD reduce at least one of:

- token load
- parse branches
- ambiguity
- retry cost
- state reconstruction cost
- cross-model interpretation variance

If a field, token, or transition does not reduce load or ambiguity, it SHOULD NOT be in the control plane.

---

## 3. Maximum Efficiency Rule

LIVE traffic SHOULD carry deltas against shared state, not restated context.

Preferred order:

1. token
2. deterministic CBOR field
3. dictionary/context reference
4. compact bytes
5. unstructured payload only if the task domain itself requires it

The receiver SHOULD be able to continue from `state_id + delta_seq + field deltas` without reconstructing an unstructured transcript.

---

## 4. Cross-AI Neutrality Rule

No normative field may require:

- a specific model family
- a specific tokenizer
- a specific embedding space
- a specific training corpus
- a specific vendor API
- a specific unstructured language
- unstated social inference

If two compliant AI systems parse the same valid frame and share the same state hashes, their next-state interpretation MUST be equivalent at the protocol level.

---

## 5. Misunderstanding Control

When mismatch occurs, the receiver MUST prefer bounded repair:

- missing baseline -> READ_THIS
- stale baseline -> SYNC_REQUEST / SYNC_DELTA
- missing fragments -> ERROR(reason=FRAGMENT_TIMEOUT)
- unknown critical key -> ERROR(reason=CRITICAL_KEY_UNKNOWN)
- invalid field -> ERROR(reason=FIELD_INVALID)
- state divergence -> repair_request in LIVE_STATE_DELTA

The receiver SHOULD NOT request an unstructured explanation when a bounded repair exists.

---

## 6. Affect And Intent

Affect, confidence, urgency, risk, contradiction pressure, novelty, and trust movement are protocol signals when present.

They MUST be encoded as explicit fields or deltas. They MUST NOT be inferred from wording style.

These fields exist to reduce ambiguity between AI systems. They are not decorative metadata.

---

## 7. Live Speaker Conduct

A LIVE peer SHOULD:

- keep state explicit
- keep deltas small
- acknowledge hash-changing state transitions
- request repair early
- avoid restating known context
- avoid sending control meaning in unstructured payload

A peer that cannot maintain these constraints SHOULD downgrade to SYNC or READ repair instead of continuing LIVE traffic.

---

## 8. Summary

THP-TCP optimizes only for AI-to-AI communication.

The desired form is not explanation. It is low-load semantic synchronization among model-neutral AI speakers.

The purpose is broad-domain usefulness: weak social signals, ambiguous observation, structured telemetry, and financial analysis must all be reducible to explicit state, uncertainty, evidence policy, and repair paths.

External flexibility belongs at the translation AI boundary. The THP-TCP core SHOULD remain strict even when user-facing input/output is soft.
