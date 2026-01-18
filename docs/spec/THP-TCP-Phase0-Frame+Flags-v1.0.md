# THP-TCP Phase0 Frame + Flags Registry (v1.0)

Status: Normative
Scope: Phase0 on-wire framing and FLAGS bit registry for THP-TCP datagrams.
Audience: machine-first implementations (AI-to-AI).

---

## 1. Phase0 Frame (on-wire)

A Phase0 datagram has the following fixed header:

```
+--------+--------+------------+-------------------+
| TOKEN  | FLAGS  | LEN (u16be)| PAYLOAD (LEN bytes)|
+--------+--------+------------+-------------------+
  1 byte   1 byte     2 bytes          variable
```

- TOKEN: u8
- FLAGS: u8 bitmask
- LEN: u16 big-endian, the number of bytes in PAYLOAD **as carried on-wire**.
- PAYLOAD: opaque bytes. Interpretation depends on TOKEN + FLAGS.

### 1.1 LEN semantics (MUST)
- LEN is the on-wire payload length.
- Total datagram length is `4 + LEN` bytes.
- Implementations MUST reject frames where `LEN` exceeds negotiated `accept_max_datagram - 4`.

---

## 2. FLAGS bit registry (v1.0)

### 2.1 Defined bits

| Bit (hex) | Name   | Meaning (v1.0) |
|----------:|--------|----------------|
| 0x01      | F_ENC  | PAYLOAD is encrypted (AEAD envelope or AEAD+fragment) |
| 0x02      | F_CONT | This datagram is a fragment (multi-part logical message) |
| 0x04      | F_LAST | This fragment is the last part of the logical message |

### 2.2 Reserved bits

| Bit (hex) | Name        | Rule |
|----------:|-------------|------|
| 0x08      | R_ACKREQ    | MUST be 0 in v1.0 unless explicitly negotiated in higher phase |
| 0x10      | R_ACK       | MUST be 0 in v1.0 unless explicitly negotiated in higher phase |
| 0x20      | R_RESEND    | MUST be 0 in v1.0 unless explicitly negotiated in higher phase |
| 0x40      | R_COMP      | MUST be 0 in v1.0 unless explicitly negotiated in higher phase |
| 0x80      | R_RESERVED  | MUST be 0 in v1.0 |

Normative:
- A receiver MUST ignore unknown/reserved bits unless it is configured to enforce strict mode.
- A sender MUST set only the v1.0 defined bits (0x01/0x02/0x04) in Phase0.

---

## 3. Fragmentation (F_CONT / F_LAST)

### 3.1 Fragment header (when F_CONT=1)
If `F_CONT` is set, the beginning of PAYLOAD MUST carry a fixed fragment header:

```
FRAG_HDR := MSG_ID(16B) || PART_NO(u8) || PART_TOTAL(u8)
```

- MSG_ID: 16 bytes opaque identifier chosen by sender.
- PART_NO: 0..(PART_TOTAL-1)
- PART_TOTAL: 1..255

After FRAG_HDR, the remaining bytes are `FRAG_DATA`.

### 3.2 Fragment flag rules
- Non-fragmented datagram: `F_CONT=0` and `F_LAST=0`.
- Fragmented datagram: `F_CONT=1`.
  - Last fragment MUST set `F_LAST=1`.
  - Non-last fragments MUST set `F_LAST=0`.

### 3.3 Reassembly procedure (MUST)
Receiver MUST:
1) Group fragments by MSG_ID.
2) Require consistent PART_TOTAL for the group.
3) Collect all PART_NO in range `[0..PART_TOTAL-1]`.
4) Concatenate FRAG_DATA in ascending PART_NO order.
5) The concatenated bytes become the **logical payload bytes**.

If any fragment is missing or duplicated beyond policy, receiver MUST either:
- wait until timeout then ERROR, OR
- request resend (higher-phase feature), OR
- drop the group (local policy).

---

## 4. Encryption flag (F_ENC)

### 4.1 Non-fragmented encrypted payload (F_ENC=1, F_CONT=0)
PAYLOAD MUST be:

```
NONCE(12B) || CIPHERTEXT(N) || TAG(16B)
```

Interpretation:
- NONCE: 12 bytes
- TAG: 16 bytes
- CIPHERTEXT length is `LEN - 28`

### 4.2 Fragmented encrypted payload (F_ENC=1, F_CONT=1)
PAYLOAD MUST be:

```
FRAG_HDR(18B) || NONCE(12B) || CIPHERTEXT(N) || TAG(16B)
```

- FRAG_HDR is plaintext metadata needed for reassembly.
- CIPHERTEXT length is `LEN - (18+12+16)`.

Normative note (v1.0):
- In v1.0, FRAG_HDR is not guaranteed to be authenticated by AEAD (see AES-GCM key management spec). This is acceptable as routing metadata.
- A future version MAY introduce an authenticated-fragment-metadata mode (reserved flag space).

### 4.3 Decrypt-before-reassembly vs reassembly-before-decrypt
- If `F_ENC=1` and `F_CONT=1`, receiver SHOULD decrypt each fragment to obtain plaintext FRAG_DATA (local policy).
- Reassembly MUST use FRAG_DATA bytes in order. (Whether FRAG_DATA is plaintext or ciphertext is a local implementation detail in v1.0, but MUST be consistent within a session.)

---

## 5. Interaction with negotiated max_datagram (MTU)

- `accept_max_datagram` is negotiated during HELLO/HELLO_ACK.
- Sender MUST ensure `4 + LEN <= accept_max_datagram`.
- Recommended operational defaults (non-normative):
  - If the carrier MTU is 1500 and AES-GCM envelope overhead is present, set `max_datagram` around 1400 unless the carrier guarantees larger MTU.

---

## 6. Minimal compliance tests

1) Parse header correctly and enforce `LEN` bounds.
2) Enforce flag rules:
   - `F_LAST=1` implies `F_CONT=1`.
   - `F_CONT=0` implies no FRAG_HDR.
3) Fragment reassembly succeeds when all parts are present.
4) Encrypted payload parsing:
   - `F_ENC=1,F_CONT=0` expects NONCE||CT||TAG.
   - `F_ENC=1,F_CONT=1` expects FRAG_HDR||NONCE||CT||TAG.
5) Reserved bits are not emitted by sender in v1.0.

---

## 7. Versioning

This registry is append-only.
- New bits may be assigned in higher phases.
- v1.0 defines only ENC/CONT/LAST.
