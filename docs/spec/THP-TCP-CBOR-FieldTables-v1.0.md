# THP-TCP CBOR Field Tables v1.0 (Draft / Normative)

## 1. Common Rules (Normative)

### 1.1 Encoding
- CBOR payloads **MUST** be deterministic/canonical.
- Indefinite-length items **MUST NOT** be used in v1.0.

### 1.2 Key Types
- Map keys **SHOULD** be unsigned integers.
- Text keys **MAY** be used only in Experimental range.

### 1.3 Unknown Keys (Forward-compat)
- Unknown keys **MUST be ignored** and parsing **MUST continue**.
- Exception: if key is listed in `critical_keys`, receiver **MUST reject** with `ERROR(reason=CRITICAL_KEY_UNKNOWN)`.

### 1.4 Required / Optional
- Missing **required** key or **type mismatch** ⇒ `ERROR(reason=FIELD_INVALID)`.
- Missing optional keys ⇒ use **default** where defined.
- Keys in the **Optional range (64–127)** are optional by default, but **MAY be required** if explicitly marked as required in a token table.

### 1.5 Key Space (Global)
- `0–63` : **Core** (v1.0 fixed)
- `64–127` : **Optional** (v1.x additive; may be required if explicitly marked)
- `128–191` : **Reserved**
- `192–255` : **Experimental / local**
- `256–511` : **Vendor**
- `512+` : **Private**

---

## 2. Token Field Tables (v1.0 minimum set)

### 2.1 HELLO (0x01)
| key | name | type | req | default | note |
|---:|---|---|:--:|---|---|
| 0 | proto_ver | uint | M | — | protocol version |
| 1 | token_table_ver | uint | M | — | fixed opcode table version |
| 2 | context_id | uint | M | — | context namespace |
| 3 | dict_hash | bytes | M | — | dictionary digest |
| 4 | max_datagram | uint | M | — | negotiated MTU cap |
| 5 | enc_suite | uint | M | 1 | 1=AES-256-GCM |
| 6 | cbor_mode | uint | M | 1 | 1=deterministic |
| 7 | kx_pub | bytes | M | — | X25519 pubkey |
| 8 | kx_mode | uint | M | 1 | 1=X25519 |
| 9 | kdf_mode | uint | M | 1 | 1=HKDF-SHA256 |
| 10 | aead_mode | uint | M | 1 | 1=AES-256-GCM |
| 64 | key_id_hint | bytes | O | — | optional key id |

### 2.2 HELLO_ACK (0x02)
| key | name | type | req | default | note |
|---:|---|---|:--:|---|---|
| 7 | kx_pub | bytes | M | — | X25519 pubkey |
| 64 | accept_max_datagram | uint | M | — | accepted value |
| 65 | accept_crypto | uint | M | 1 | 0/1 |

### 2.3 DICT_SNAPSHOT (0x03)
| key | name | type | req | default | note |
|---:|---|---|:--:|---|---|
| 0 | context_id | uint | M | — | context |
| 1 | dict_seq | uint | M | — | chunk index |
| 2 | dict_total | uint | M | — | total chunks |
| 3 | dict_chunk | bytes | M | — | raw bytes |

### 2.4 DICT_ACK (0x04)
| key | name | type | req | default | note |
|---:|---|---|:--:|---|---|
| 0 | context_id | uint | M | — | context |
| 1 | dict_hash | bytes | M | — | digest |
| 2 | status | uint | M | — | 0=accepted,1=rejected,2=needs_resend |

### 2.5 UPGRADE (0x05) (minimal)
| key | name | type | req | default | note |
|---:|---|---|:--:|---|---|
| 0 | target_version | uint | M | — | target proto ver |
| 1 | reason | uint | O | 0 | reason code |

### 2.6 ERROR (0x06)
| key | name | type | req | default | note |
|---:|---|---|:--:|---|---|
| 0 | reason | uint | M | — | error code |
| 1 | detail | text/bytes | O | — | optional text or bytes |
| 2 | critical_keys | array<uint> | O | — | if unknown → reject |

---

## 3. Deterministic CBOR Constraints
- Integers **MUST** be shortest encoding.
- Map keys **MUST** be in canonical order.
- Float **SHOULD NOT** be used in v1.0.
