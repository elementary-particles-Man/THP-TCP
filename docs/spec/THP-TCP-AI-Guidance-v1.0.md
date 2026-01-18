# THP-TCP AI Guidance v1.0 (AI-facing)

Purpose: Provide minimal, machine-parseable guidance for AI agents to use THP-TCP safely and consistently.

---

## 1. Startup sequence (MUST)
1) Send HELLO
2) Receive HELLO_ACK
3) Send DICT_SNAPSHOT (fragmented if needed)
4) Receive DICT_ACK(status=accepted)
5) Begin normal message traffic

Contextual tokens (128–255) MUST NOT be used before DICT_ACK(accepted).

---

## 2. Frame format (Phase0 v1.0.2)
`[TOKEN:u8][FLAGS:u8][LEN:u16be][PAYLOAD:LEN]`

Flags:
- `F_CONT=0x80`
- `F_LAST=0x40`

If F_CONT is set, PAYLOAD begins with fragment header:
`[msg_id:16][part_no:u8][part_total:u8][orig_token:u8][frag_bytes...]`

---

## 3. Determinism (MUST)
- CBOR payloads MUST be deterministic/canonical.
- Unknown CBOR keys MUST be ignored unless listed in `critical_keys`.

---

## 4. Encryption (MUST when enabled)
AEAD envelope:
`PAYLOAD := [nonce:12][ciphertext:N][tag:16]`

Nonce rule:
`nonce = nonceprefix(4B) || seq(8B_be)`

See: `docs/spec/THP-TCP-AESGCM-KeyMgmt-v1.0.md`

---

## 5. max_datagram
- `max_datagram` negotiated in HELLO/HELLO_ACK.
- Fragment if payload exceeds max_datagram.

---

## 6. Error handling
- On parse/type errors: emit ERROR(reason=FIELD_INVALID)
- On crypto failure: emit ERROR(reason=AEAD_TAG_FAIL)
- On missing fragments: emit ERROR(reason=FRAGMENT_TIMEOUT)

---

## 7. References (normative)
- `docs/spec/THP-TCP-Draft.md`
- `docs/spec/THP-TCP-v1.0.1-Addendum.md`
- `docs/spec/THP-TCP-v1.0.2-Addendum.md`
- `docs/spec/THP-TCP-AESGCM-KeyMgmt-v1.0.md`
- `docs/spec/THP-TCP-CBOR-FieldTables-v1.0.md`
- `docs/spec/THP-TCP-Nickname-Addressing-v1.0.md`
