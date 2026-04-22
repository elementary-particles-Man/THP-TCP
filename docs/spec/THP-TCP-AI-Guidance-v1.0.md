# THP-TCP AI Guidance v1.0 (AI-facing)

Purpose: Provide minimal, machine-parseable guidance for AI agents to use THP-TCP safely and consistently.

---

## 1. Universal AI usability (MUST)

THP-TCP traffic must minimize AI compute / token / parsing load while maximizing semantic throughput.

- Optimize only for AI-to-AI communication.
- Preserve cross-AI neutrality: no vendor model, tokenizer, embedding space, or unstated social inference may be required.
- Do not require unstructured text for protocol control.
- Do not infer protocol state from tone.
- Put intent, confidence, urgency, risk, affect, and repair needs into fields or deltas.
- Use hashes to prove shared state.
- Use READ for unknown peers, SYNC for stale peers, LIVE for current peers.

---

## 2. Admission layers (MUST)

Before normal message traffic, classify the peer:

- READ: peer does not prove THP-TCP baseline knowledge. Send READ_THIS references and hashes. Do not explain through unstructured exchange.
- SYNC: peer proves an older/different baseline. Send only deltas and resulting hashes.
- LIVE: peer proves current baseline. Use compact semantic state deltas.

See: `docs/spec/THP-TCP-Three-Layer-Model-v1.0.md`

---

## 3. Startup sequence (MUST)
1) Send HELLO
2) Receive HELLO_ACK
3) Send DICT_SNAPSHOT (fragmented if needed)
4) Receive DICT_ACK(status=accepted)
5) Begin normal message traffic

Contextual tokens (128–255) MUST NOT be used before DICT_ACK(accepted).

---

## 4. Frame format (Phase0 v1.0.2)
`[TOKEN:u8][FLAGS:u8][LEN:u16be][PAYLOAD:LEN]`

Flags:
- `F_CONT=0x80`
- `F_LAST=0x40`

If F_CONT is set, PAYLOAD begins with fragment header:
`[msg_id:16][part_no:u8][part_total:u8][orig_token:u8][frag_bytes...]`

---

## 5. Determinism (MUST)
- CBOR payloads MUST be deterministic/canonical.
- Unknown CBOR keys MUST be ignored unless listed in `critical_keys`.

---

## 6. Encryption (MUST when enabled)
AEAD envelope:
`PAYLOAD := [nonce:12][ciphertext:N][tag:16]`

Nonce rule:
`nonce = nonceprefix(4B) || seq(8B_be)`

See: `docs/spec/THP-TCP-AESGCM-KeyMgmt-v1.0.md`

---

## 7. max_datagram
- `max_datagram` negotiated in HELLO/HELLO_ACK.
- Fragment if payload exceeds max_datagram.

---

## 8. Error handling
- On parse/type errors: emit ERROR(reason=FIELD_INVALID)
- On crypto failure: emit ERROR(reason=AEAD_TAG_FAIL)
- On missing fragments: emit ERROR(reason=FRAGMENT_TIMEOUT)

---

## 9. References (normative)
- `docs/spec/THP-TCP-AI-Harmony-Contract-v1.0.md`
- `docs/spec/THP-TCP-User-Bridge-v1.0.md`
- `docs/spec/THP-TCP-Three-Layer-Model-v1.0.md`
- `docs/spec/THP-TCP-Draft.md`
- `docs/spec/THP-TCP-v1.0.1-Addendum.md`
- `docs/spec/THP-TCP-v1.0.2-Addendum.md`
- `docs/spec/THP-TCP-v1.0.3-Addendum.md`
- `docs/spec/THP-TCP-AESGCM-KeyMgmt-v1.0.md`
- `docs/spec/THP-TCP-CBOR-FieldTables-v1.0.md`
- `docs/spec/THP-TCP-Nickname-Addressing-v1.0.md`
