# THP-TCP v1.0.2 Addendum


> **Normative Crypto Assumptions:** See `docs/spec/THP-TCP-AESGCM-KeyMgmt-v1.0.md` for X25519/HKDF/AES-GCM session key assumptions, nonce rules, and AAD.

This addendum is normative. It refines v1.0 / v1.0.1 with a minimal, non-breaking clarification for fragmentation flags, frame layout, and CONT vs UPDATE semantics.

---

## 1. Phase0 Frame (v1.0.2) — minimal extension

**Frame0 := [TOKEN:u8][FLAGS:u8][LEN:u16be][PAYLOAD:LEN]**

- FLAGS is common for all tokens.
- LEN is the payload length (bytes), big-endian.
- This change enables consistent F_CONT / F_LAST usage without per-token payload hacks.

---

## 2. Fragmentation flags (normative)

- `F_CONT = 0x80` — indicates the payload begins with a fragmentation header.
- `F_LAST = 0x40` — indicates the final fragment.
- **Single-fragment messages MAY set both flags** (F_CONT|F_LAST) to keep receiver logic uniform.

---

## 3. Fragment header (payload prefix, required when F_CONT)

`[msg_id:16][part_no:u8][part_total:u8][orig_token:u8][frag_bytes...]`

- `part_no` is 0-based.
- Example “1/5”: `part_no=0`, `part_total=5`.
- Receiver MUST reassemble by `msg_id` and process only after all parts are present.

---

## 4. CONT vs UPDATE semantics (strict separation)

- **CONT (F_CONT)**: transport-level continuation / fragmentation only.
- **UPDATE (token 0x13)**: dictionary/content semantic update only.
- UPDATE **MUST NOT** imply fragmentation. CONT **MUST NOT** imply semantic update.

---

## 5. DICT_SNAPSHOT fragmentation handling (clarification)

- DICT_SNAPSHOT **MUST** be fully reassembled before processing.
- Missing fragments MUST trigger `ERROR` after timeout (implementation-defined timeout).

---

## 6. Compatibility

- v1.0.2 does not change token assignments.
- v1.0.2 only adds a FLAGS byte and extends LEN to u16be.
- v1.0.1 HELLO/max_datagram semantics remain unchanged.
