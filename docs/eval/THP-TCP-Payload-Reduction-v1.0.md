# THP-TCP Payload Reduction v1.0

Status: Evaluation note

Purpose: measure whether THP-TCP reduces payload for LIVE semantic state deltas compared with unstructured text and JSON control payloads.

The goal is not generic compression. The goal is to avoid sending restated context once READ/SYNC/LIVE has established shared state.

---

## 1. Principle

LIVE payloads should carry:

- state id
- delta sequence
- presence bitmap
- compact field values

They should not carry repeated field names, restated context, or explanatory text.

---

## 2. Current Minimal LIVE Encoding

Prototype:

```text
Frame0 := TOKEN(0x0B) || FLAGS(0) || LEN(u16be) || PAYLOAD
PAYLOAD := state_id(u16) || delta_seq(u16) || presence(u16) || values...
```

`presence` indicates which compact values are present:

- bit 0: intent
- bit 1: confidence
- bit 2: urgency
- bit 3: risk
- bit 4: affect_tag
- bit 5: affect_score
- bit 6: contradiction_pressure
- bit 7: novelty
- bit 8: trust_delta
- bit 9: repair_request

This is an evaluation encoding for payload reduction. It can be promoted into a normative addendum after interop review.

---

## 3. Example

For a LIVE delta containing:

- state_id
- delta_seq
- intent
- confidence
- urgency
- risk
- affect tag/score
- contradiction pressure
- novelty
- trust delta

the encoded THP-TCP frame is 19 bytes.

Equivalent JSON is roughly 170+ bytes. Equivalent unstructured text is task-dependent and usually larger once context is included.

---

## 4. Rule

If a semantic control payload can be represented as a LIVE_STATE_DELTA, it SHOULD NOT be sent as JSON or unstructured text inside LIVE.
