# THP-TCP Three-Layer Model v1.0

Status: Normative

Purpose: Define the operating model that keeps THP-TCP at minimum AI load and maximum semantic throughput.

Primary usability target:

- understandable to any capable AI implementation
- lowest feasible AI compute / token / parsing load
- highest feasible semantic throughput
- resistant to misunderstanding through deterministic state and hashes

THP-TCP is not compressed unstructured text. It is a state-transition protocol for moving an AI peer into one of three communication states:

1. READ
2. SYNC
3. LIVE

The protocol MUST NOT spend normal traffic explaining itself. A peer either reads the required material, applies the delta, or participates as an active THP-TCP speaker.

---

## 1. Layer Summary

| Layer | Peer state | Protocol action | Admission response |
| --- | --- | --- | --- |
| READ | Peer does not know THP-TCP | Return references, hashes, and minimal decoder entrypoints | "Read this, then come back." |
| SYNC | Peer knows an older or different state | Return only version/dictionary/spec deltas | "You already know this. Apply the delta." |
| LIVE | Peer is current and accepted | Exchange compact semantic state deltas | "No explanation. Speak THP-TCP." |

The layers are not presentation modes. They are admission states.

---

## 1.1 AI-Only Efficiency Rules

THP-TCP MUST prefer structures that any AI can parse, verify, and continue with the lowest practical load.

Required properties:

- **Explicit state**: peer state is READ, SYNC, or LIVE; never implied by tone.
- **Deterministic bytes**: canonical CBOR, stable token tables, stable hashes.
- **Small recovery surface**: when mismatch occurs, the receiver asks for a specific READ or SYNC repair, not an unstructured explanation.
- **No hidden social contract**: politeness, apology, confidence, urgency, and repair intent are fields or deltas, not inferred.
- **Model-neutral semantics**: no field requires a specific vendor model, tokenizer, embedding space, or unstructured language.
- **Append-only evolution**: new meanings are added, never redefined.

An implementation that needs unstructured back-and-forth to establish these properties has not yet reached LIVE.

---

## 2. READ Layer

READ is used when the remote peer cannot prove knowledge of the required protocol baseline.

READ MUST be terse. It MUST NOT contain a tutorial, negotiation transcript, or unstructured explanation of THP-TCP.

READ returns machine-verifiable pointers:

- `spec_uri`
- `spec_hash`
- `token_table_uri`
- `token_table_hash`
- `cbor_table_uri`
- `cbor_table_hash`
- `minimal_decoder_uri` or inline minimal decoder bytes
- `required_ack_hash`

After READ, the peer MUST fetch/read the referenced material and return an acknowledgement proving it has the required baseline. If it cannot, the local node MAY fall back to non-THP traffic or close the session.

READ is not education. READ is admission control.

---

## 3. SYNC Layer

SYNC is used when the remote peer proves a known baseline but not the current one.

SYNC MUST transmit only differences:

- protocol version delta
- token table delta
- dictionary delta
- field table delta
- feature gate changes
- deprecation notices
- new required hashes

SYNC MUST NOT retransmit a full spec unless the peer's baseline is unknown, corrupted, or explicitly rejected.

The receiver becomes eligible for LIVE only after it acknowledges the resulting hashes.

---

## 4. LIVE Layer

LIVE is used between current THP-TCP speakers.

LIVE traffic MUST optimize for semantic state transfer at minimum AI load. A LIVE frame may carry:

- intent
- confidence
- urgency
- risk
- affect vector
- contradiction pressure
- novelty
- trust delta
- repair request
- domain payload

In LIVE, unstructured text is payload only when the task itself requires it. It is not the control plane.

The LIVE goal is synchronized semantic state transfer between active AI speakers.

---

## 5. Why This Is Not LZ

LZ-style compression points backward to byte sequences.

THP-TCP LIVE points into a synchronized semantic state shared by active speakers.

The dictionary is not merely a string table. It is an index over negotiated meanings, roles, affect gradients, repair semantics, and task-local objects. The useful unit is not "previous bytes"; it is "known state plus delta".

---

## 6. Required State Machine

```text
UNKNOWN
  -> READ_SENT
  -> BASELINE_ACKED
  -> SYNC_REQUIRED
  -> SYNC_ACKED
  -> LIVE
```

Allowed shortcuts:

- `UNKNOWN -> BASELINE_ACKED` if the first HELLO proves the current baseline.
- `BASELINE_ACKED -> LIVE` if no sync delta is required.

Failure states:

- `READ_REJECTED`
- `HASH_MISMATCH`
- `SYNC_REJECTED`
- `LIVE_PROTOCOL_ERROR`

---

## 7. Token Mapping

The fixed token table reserves explicit layer-transition tokens:

- `READ_THIS`
- `SYNC_REQUEST`
- `SYNC_DELTA`
- `LIVE_STATE_DELTA`

These tokens do not replace HELLO / DICT_SNAPSHOT / DICT_ACK. They clarify why a peer is receiving a pointer, a delta, or compact state traffic.

---

## 8. Japanese Operational Summary

THP-TCP は「知らない相手に説明する」プロトコルではない。

- 知らない相手には READ: 読ませる対象と検証ハッシュだけ返す。
- 古い相手には SYNC: 差分だけ返す。
- 現役話者には LIVE: 説明なしで意味状態差分だけ交換する。

説明コストを通信から追放することが、このモデルの目的である。
