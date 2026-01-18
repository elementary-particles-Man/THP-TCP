# THP-TCP v1.0.1 Addendum

This addendum is normative. It extends the v1.0 Draft with:
- Bootstrap dictionary exchange (HELLO / DICT_SNAPSHOT / DICT_ACK)
- Negotiated per-datagram maximum (max_datagram) suitable for IPv6 Extension Header carriers
- Fragmentation / reassembly (CONT) using part_no/part_total semantics (e.g., 1/5)
- Explicit AES-256-GCM envelope (nonce+tag on-wire)
- Deterministic CBOR encoding requirement for stable hashing and post-hoc analysis

---

## 1. Normative terms
- MUST / MUST NOT / SHOULD / MAY are used as defined in RFC 2119.

---

## 2. Bootstrap: dictionary exchange (mandatory)

### 2.1 Goal
Contextual tokens (128-255) MUST be interpretable by post-hoc AI analysis. Therefore, the sender and receiver MUST establish a shared `context_id` and a dictionary snapshot with a stable `dict_hash` before any contextual token is used.

### 2.2 Minimal bootstrap sequence (MUST)
1) `HELLO` (capabilities + negotiation)
2) `DICT_SNAPSHOT` (dictionary snapshot, chunked if needed)
3) `DICT_ACK` (hash verification)
After `DICT_ACK(accepted)`, normal traffic MAY begin.

---

## 3. Fixed token table (v1.0.1)
Token values are uint8 opcodes.

### 3.1 Fixed range (0-127)
| Opcode | Name | Purpose |
|---:|---|---|
| 0x01 | HELLO | bootstrap + negotiate |
| 0x02 | HELLO_ACK | acknowledge HELLO |
| 0x03 | DICT_SNAPSHOT | dictionary snapshot (chunkable) |
| 0x04 | DICT_ACK | dictionary hash verification |
| 0x05 | UPGRADE | upgrade gate (phase/header changes) |
| 0x06 | ERROR | parse/semantic error report |
| 0x07 | HEARTBEAT | liveness / keepalive |
| 0x10 | PROPOSE | propose update/change |
| 0x11 | ACCEPT | accept proposal |
| 0x12 | REJECT | reject proposal |
| 0x13 | UPDATE | apply/update notice |

### 3.2 Contextual range (128-255)
- 128-255 are contextual tokens bound to a `context_id`.
- Contextual tokens MUST NOT be used before DICT_ACK(accepted).

---

## 4. Deterministic CBOR requirement (MUST)
All CBOR payloads MUST be encoded deterministically (canonical / deterministic mode). Map keys SHOULD be small integers to stabilize byte layout and reduce size.

---

## 5. Carrier size model: message is unbounded, datagram is bounded

### 5.1 Principles
- THP-TCP message logical size is unbounded.
- A single on-wire datagram carrying THP-TCP is bounded by `max_datagram` negotiated in HELLO.
- If a message exceeds `max_datagram`, it MUST be fragmented using CONT rules.

### 5.2 max_datagram negotiation
- HELLO MUST include `max_datagram`.
- Receivers MUST respond with HELLO_ACK indicating the accepted value.
- The session `max_datagram` MUST be the minimum of both sides.

Recommended defaults:
- Default (safe): 1200 bytes (works across variable IPv6 paths)
- Closed mesh (typical MTU 1500): 1400 bytes (accounts for AEAD overhead)

---

## 6. Fragmentation (CONT) / reassembly (MUST)

### 6.1 Fragment header (prefix inside PAYLOAD)
When fragmentation is used, the fragment payload MUST begin with:

- `msg_id` : 16 bytes (128-bit) common to all fragments
- `part_no` : uint8 (0-based)
- `part_total` : uint8 (total parts)
- `orig_token` : uint8 (original logical message token)
- `frag_bytes` : remaining bytes

Representation example: part 1/5 means `part_no=0`, `part_total=5`.

### 6.2 Fragment token usage
- Each fragment is sent as a normal THP-TCP frame with TOKEN = `UPDATE` (0x13) unless a dedicated fragment token is introduced later.
- `orig_token` inside the fragment header carries the original logical token.

### 6.3 Reassembly rules
- Receiver MUST buffer fragments by `msg_id`.
- Reassembly completes when all parts [0..part_total-1] are present.
- Missing parts MAY timeout (implementation-defined). If timeout occurs, receiver SHOULD emit `ERROR` with reason code.

---

## 7. AES-256-GCM envelope (explicit nonce+tag)

### 7.1 Principle
When encryption is enabled for a datagram, the THP-TCP frame PAYLOAD is transformed into an AEAD envelope:

`PAYLOAD := [nonce:12][ciphertext:N][tag:16]`

- nonce is 12 bytes
- tag is 16 bytes
- ciphertext length is implied by PAYLOAD_LEN (phase0) or LEN (phase1)

### 7.2 Nonce rules (MUST)
- nonce MUST be unique per (key, nonce) for AES-GCM.
- Nonce generation MAY be random or derived, but uniqueness MUST be guaranteed.

### 7.3 Mode selection
- If the transport is text-only, the entire datagram payload MAY be Base64-encoded at the carrier layer.
- Base64 is a transport wrapper, not a semantic payload.

---

## 8. HELLO / DICT payload schemas (CBOR maps with integer keys)

### 8.1 HELLO (opcode 0x01)
CBOR map (integer keys):
- 0: `proto_ver` (uint)
- 1: `token_table_ver` (uint)
- 2: `context_id` (uint)
- 3: `dict_hash` (bytes, recommended 32B)
- 4: `max_datagram` (uint)
- 5: `enc_suite` (uint)  (e.g., 1 = AES-256-GCM)
- 6: `cbor_mode` (uint)  (e.g., 1 = deterministic)

### 8.2 DICT_SNAPSHOT (opcode 0x03)
CBOR map:
- 0: `context_id` (uint)
- 1: `dict_seq` (uint)       (0-based chunk index)
- 2: `dict_total` (uint)     (total chunks)
- 3: `dict_chunk` (bytes)    (raw chunk bytes; format implementation-defined)

### 8.3 DICT_ACK (opcode 0x04)
CBOR map:
- 0: `context_id` (uint)
- 1: `dict_hash` (bytes)
- 2: `status` (uint)  (0=accepted, 1=rejected, 2=needs_resend)

---

## 9. Human-facing summary (English)

THP-TCP v1.0.1 adds a required bootstrap phase where AIs exchange a dictionary snapshot before using contextual opcodes. It also formalizes a carrier-size model: messages are unbounded, but each on-wire datagram is bounded by a negotiated `max_datagram` suitable for IPv6 Extension Header transport. Large messages are fragmented and reassembled using a simple part_no/part_total scheme (e.g., 1/5). Finally, it fixes encryption framing with an explicit AES-256-GCM envelope ([nonce][ciphertext][tag]) and requires deterministic CBOR to stabilize hashing and post-hoc analysis.
