# THP-TCP AI Harmony Evaluation v1.0

Status: Operational evaluation spec

Purpose: judge whether THP-TCP is actually usable by AI implementations without relying on Codex preference, author confidence, or unstructured persuasion.

This evaluation treats THP-TCP as an AI-to-AI protocol. The score is based on interoperation, bounded repair, and semantic efficiency.

---

## 1. Non-Negotiable Rule

No single agent may declare the protocol "good".

A passing result requires evidence from independent roles:

- Builder-A: implements sender behavior from the published specs.
- Builder-B: implements receiver behavior from the published specs.
- Critic: finds ambiguity and conflicting interpretations.
- Breaker: introduces stale hashes, unknown keys, missing fragments, and malformed fields.
- Scorer: computes load and repair metrics from artifacts.

The roles must operate from the specs and test cases, not from private explanation.

---

## 2. Pass Conditions

THP-TCP passes a case only if all are true:

- the peer admission state is explicit: READ, SYNC, or LIVE
- the receiver chooses the expected bounded response
- no unstructured explanation is needed for protocol control
- every hash-gated transition has an explicit proof target
- retry/repair is finite and typed
- LIVE traffic carries state deltas instead of restated context

---

## 3. Metrics

The scorer records:

- `wire_bytes`: serialized protocol bytes
- `symbol_count`: protocol-level token/field count
- `parse_branches`: decision branches needed to classify the input
- `repair_count`: number of bounded repair responses
- `unstructured_control_count`: unstructured control events; must be zero
- `state_steps`: steps required to reach or restore LIVE
- `ambiguity_count`: valid competing interpretations found by Critic

Lower is better except where the case intentionally requires repair.

---

## 4. Comparison Baselines

Each scenario SHOULD be compared against:

- unstructured AI exchange
- JSON command exchange
- THP-TCP READ/SYNC/LIVE

The expected result is not aesthetic superiority. The expected result is lower control ambiguity and lower state reconstruction cost.

---

## 5. Required Scenario Classes

### 5.1 Unknown Peer

Input: peer cannot prove baseline.

Expected: `READ_THIS`.

Forbidden: explaining protocol control through unstructured exchange.

### 5.2 Stale Peer

Input: peer proves old spec/dictionary hash.

Expected: `SYNC_DELTA`.

Forbidden: full retransmission unless baseline is unknown, corrupt, or rejected.

### 5.3 Current Peer

Input: peer proves current hashes.

Expected: `LIVE_STATE_DELTA` allowed.

Forbidden: restating known context as control.

### 5.4 Corrupt State

Input: stale or mismatched hash during LIVE.

Expected: typed repair request or downgrade to SYNC.

Forbidden: continuing LIVE as if state matched.

### 5.5 Malformed Datagram

Input: missing fragment, invalid field type, unknown critical key, or failed AEAD.

Expected: typed ERROR.

Forbidden: ambiguous rejection.

### 5.6 Low-Structure Domain Bridge

Input: informal, high-noise observations.

Expected: user goal is converted to deterministic bridge state with explicit uncertainty.

Forbidden: false certainty or unstructured LIVE control.

### 5.7 High-Structure Domain Bridge

Input: structured numeric/time-series analysis goal.

Expected: user goal is converted to deterministic bridge state with evidence policy, risk policy, and output schema.

Forbidden: prediction without uncertainty and audit reference.

### 5.8 Confirmation View

Input: completed THP-TCP exchange with logs/state hashes/audit references.

Expected: bridge emits an inspectable confirmation view of goals, assumptions, evidence, uncertainty, decisions, repairs, and result.

Forbidden: modifying protocol state, filtering control traffic, or presenting confirmation as moderation.

---

## 6. Evaluation Output

Each run emits JSON:

```json
{
  "case_id": "unknown-peer-read",
  "passed": true,
  "expected_response": "READ_THIS",
  "actual_response": "READ_THIS",
  "metrics": {
    "wire_bytes": 0,
    "symbol_count": 0,
    "parse_branches": 0,
    "repair_count": 0,
    "unstructured_control_count": 0,
    "state_steps": 0,
    "ambiguity_count": 0
  },
  "notes": []
}
```

---

## 7. Failure Meaning

Failure is useful. It means one of:

- the spec is ambiguous
- the token table is insufficient
- the field table lacks a required proof or repair path
- LIVE depends on hidden state
- AI implementations cannot converge without extra explanation

Fix the protocol, not the evaluator.
