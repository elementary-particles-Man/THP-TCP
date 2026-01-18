# THP-TCP AES-GCM Key Management Assumptions v1.0 (Draft / Normative)

## 1. Scope
This document specifies session key establishment and AEAD usage assumptions for THP-TCP datagrams:

- KEX: **X25519 (ephemeral)**
- KDF: **HKDF-SHA256**
- AEAD: **AES-256-GCM**, with **explicit on-wire nonce(12B) + tag(16B)**

Out of scope:
- PKI / identity attestation (can be layered later)
- Transport routing / replay protection at network layer

---

## 2. Roles & Session

- Each peer is either **Initiator (I)** or **Responder (R)** for a session.
- Each session MUST have:
  - `context_id` (u32)
  - `dict_hash` (bytes)
  - `key_id` (bytes)

A session begins after:
1. `HELLO` (includes Initiator ephemeral pubkey)
2. `HELLO_ACK` (includes Responder ephemeral pubkey)
3. `DICT_SNAPSHOT` … `DICT_ACK(status=accepted)`
   - After (2), keys are derived; after (3), contextual tokens may be used.

---

## 3. Handshake Payload Fields (CBOR deterministic)

**CBOR encoding MUST be deterministic/canonical.** Map keys SHOULD be small integers.

### 3.1 HELLO (opcode 0x01) CBOR map keys (additions)

- `7: kx_pub` = bytes(32)  // X25519 public key (Initiator)
- `8: kx_mode` = uint      // 1 = X25519
- `9: kdf_mode` = uint     // 1 = HKDF-SHA256
- `10: aead_mode` = uint   // 1 = AES-256-GCM
- `11: key_id_hint` = bytes (optional)

### 3.2 HELLO_ACK (opcode 0x02) CBOR map keys (additions)

- `7: kx_pub` = bytes(32)  // X25519 public key (Responder)
- `12: accept_max_datagram` = uint
- `13: accept_crypto` = uint (0/1)

---

## 4. Key Agreement & Derivation

Let:

- `ss = X25519(sk_I, pk_R) = X25519(sk_R, pk_I)` (32B shared secret)
- `transcript = SHA256( CBOR_det(HELLO) || CBOR_det(HELLO_ACK) || context_id || dict_hash )`
  - `context_id` is encoded as u32 BE
  - `dict_hash` is bytes as-is

HKDF:

- `PRK = HKDF-Extract(salt=transcript, IKM=ss)`
- Expand outputs (labels are ASCII, exact bytes):
  - `key_I_to_R = HKDF-Expand(PRK, info="THP-TCP key I->R", L=32)`
  - `key_R_to_I = HKDF-Expand(PRK, info="THP-TCP key R->I", L=32)`
  - `nonceprefix_I_to_R = HKDF-Expand(PRK, info="THP-TCP nonce I->R", L=4)`
  - `nonceprefix_R_to_I = HKDF-Expand(PRK, info="THP-TCP nonce R->I", L=4)`
  - `key_id = HKDF-Expand(PRK, info="THP-TCP key-id", L=8)` (or 16; fixed by spec)

Direction:

- Initiator uses `key_I_to_R` for sending, `key_R_to_I` for receiving.
- Responder uses `key_R_to_I` for sending, `key_I_to_R` for receiving.

---

## 5. Nonce Rule (MUST be unique per key)

Nonce is **12 bytes** and MUST be constructed as:

`nonce = nonceprefix(4B) || seq(8B_be)`

- `seq` is a per-direction **monotonic u64 counter** starting at 0 at session start.
- Sender MUST increment `seq` for every encrypted datagram.
- Sender MUST NOT reuse a `(key, seq)` pair.
- Rekey MUST occur before `seq` wrap.

Nonce is **explicitly carried on-wire** as part of the AEAD envelope.

---

## 6. AEAD Envelope & AAD

For encrypted datagrams:

`PAYLOAD := [nonce:12][ciphertext:N][tag:16]`

AAD (Additional Authenticated Data) MUST be:

`AAD := [token:u8][flags:u8][len:u16be][context_id:u32be][key_id:8]`

- `len` is the plaintext payload length (before encryption), i.e., the logical PAYLOAD length.
- `key_id` is the derived key identifier for the current session.
- Implementations MAY include additional fields in AAD only if both peers negotiate it (not in v1.0).

---

## 7. Rekey / Rotation Policy

Rekey triggers (minimum set):

- MUST rekey if `seq` would exceed configured safe threshold.
- SHOULD rekey on:
  - elapsed time `T` (e.g., 30 minutes)
  - message count `M` (e.g., 2^20 datagrams)
  - dictionary/context change requiring new transcript binding

Rekey mechanism (v1.0):

- Perform a new HELLO/HELLO_ACK exchange with new ephemeral keys.
- New `key_id` replaces the old; old keys are discarded immediately.

---

## 8. Failure Handling (ERROR codes)

On crypto failure:

- Decryption/tag failure: emit `ERROR(reason=AEAD_TAG_FAIL)`
- Unknown `key_id`: emit `ERROR(reason=KEY_ID_UNKNOWN)`
- Nonce reuse detected (optional check): emit `ERROR(reason=NONCE_REUSE_SUSPECT)`
- Crypto not accepted in HELLO_ACK: abort session establishment

(Reason codes are integers; exact table defined in CBOR field tables step.)

---

## 9. Minimal Compliance Tests

An implementation is compliant if:

1. X25519+HKDF derives identical `key_id` on both peers for same transcript
2. Nonce uniqueness is enforced via seq
3. AAD is exactly as specified; tampering is detected
4. Rekey produces a different `key_id` and invalidates old keys
5. Deterministic CBOR transcript hashing is used (same bytes => same transcript hash)
