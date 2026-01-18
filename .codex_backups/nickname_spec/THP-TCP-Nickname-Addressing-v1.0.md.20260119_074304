# THP-TCP Nickname Addressing Specification (v1.0)

Status: Draft (normative-ready)
Applies to: THP-TCP v1.x (forward-only)
Audience: AI-to-AI protocol designers / implementers (machine-first)

---

## 0. Purpose

This document defines **nickname addressing** for THP-TCP:

- A minimal, deterministic **AddressRef** used inside messages.
- A resolver-friendly **AddressBinding** used in caches / resolvers / registries.
- Resolution order and error behavior.
- Encoding rules (canonical CBOR + compact length-prefixed binary record).

Human readability is out of scope.

---

## 1. Non-negotiable principles

1) **Determinism**: Equal inputs MUST yield equal canonical bytes (for CBOR).
2) **Forward-only**: Unknown fields MUST NOT break parsing.
3) **Minimal overhead**: Addressing MUST remain lightweight.
4) **Transport-neutral**: Addressing is independent of the carrier (IPv6 EH, etc.).
5) **No PKI assumption**: Signature/TLS MAY exist as an upper-layer extension, not a base requirement.

---

## 2. Terminology

- **AddressRef**: Minimal reference carried in a message (nickname + optional namespace).
- **AddressBinding**: A record that binds (namespace, nickname) -> endpoint reference.
- **Namespace**: A label selecting a resolver scope and collision domain.
- **Resolver**: Component that returns bindings (local / namespace resolver / global registry).

Normative terms: MUST / SHOULD / MAY are as in RFC 2119.

---

## 3. Canonical name rules (MUST)

### 3.1 Canonical representation
- `nickname` and `namespace` MUST be represented canonically as **byte strings (`bstr`)** in CBOR.
- Canonical bytes MUST satisfy:

  - Length: 1..32 bytes for `nickname`
  - Length: 1..32 bytes for `namespace` (when present)
  - Allowed bytes (ASCII only): `a-z`, `0-9`, `.`, `_`, `-`
  - Case: lowercase only

### 3.2 Input normalization (optional convenience)
Implementations MAY accept `tstr` input, but MUST normalize it into canonical `bstr` by:
- UTF-8 encode
- Lowercase ASCII
- Reject if any byte is outside the allowed set

If normalization fails, return `ERROR(INVALID_NAME)`.

### 3.3 Reserved names
The following are reserved and MUST NOT be used as nicknames or namespaces:
- `system`, `root`, `global`, `local`, `null`

(Reserved list is append-only across versions.)

---

## 4. Data model

### 4.1 AddressRef (carried in message payloads)
AddressRef is a **minimal reference**; it does not necessarily include a destination endpoint.

Required fields:
- `version` (u8)
- `nickname` (bstr, canonical)
- `timestamp` (u64, ns epoch)

Optional fields:
- `namespace` (bstr, canonical)
- `destination_uri` (tstr)  // OPTIONAL compatibility hook; not required for base resolution
- `ttl_ms` (u32)            // cache hint only
- `meta` (map)              // non-critical metadata, ignored by default
- `sig` (bstr)              // OPTIONAL upper-layer signature

### 4.2 AddressBinding (stored/returned by resolvers)
A binding maps `(namespace?, nickname)` to an endpoint reference.

Required fields:
- `version` (u8)
- `nickname` (bstr, canonical)
- `timestamp` (u64, ns epoch)
- `endpoint_kind` (u8)
- `endpoint` (bytes or text; depends on kind)

Optional fields:
- `namespace` (bstr, canonical)
- `ttl_ms` (u32)
- `priority` (u8)           // local policy tie-break
- `meta` (map)
- `sig` (bstr)              // OPTIONAL upper-layer signature

### 4.3 endpoint_kind values (v1.0)
- 0 = `node_id`  (endpoint = bstr, length 16)
- 1 = `ipv6`     (endpoint = bstr, length 16)
- 2 = `uri`      (endpoint = tstr)

Unknown kinds:
- If `endpoint_kind` is unknown, resolver MUST return `ERROR(UNSUPPORTED_ENDPOINT_KIND)`.

---

## 5. Resolution procedure (normative)

Given an AddressRef `(namespace?, nickname)`:

Resolution MUST follow this order:

1) **Local override table** (explicit user/operator override)
2) **Local cache** (learned bindings)
3) **Namespace resolver** (if namespace present)
4) **Global registry** (if enabled by local policy)
5) **ERROR(NOT_FOUND)**

### 5.1 Cache validity
A cached binding is valid if:
- Current time <= `timestamp + ttl_ms` (if ttl_ms present)
- OR local policy allows indefinite caching (explicitly configured)

If invalid, treat as miss and continue.

### 5.2 Collision handling (deterministic tie-break)
If multiple bindings exist at the same resolution level:
1) Prefer higher `priority` (if present)
2) Else prefer larger `timestamp`
3) Else prefer lexicographically smaller `endpoint` (byte-wise for bstr, UTF-8 byte-wise for tstr)

---

## 6. Canonical CBOR encoding (primary)

CBOR MUST be **deterministic/canonical**.
Map keys SHOULD be small integers.

### 6.1 AddressRef CBOR map keys (v1.0)

| Key | Name            | Type  | Req | Default | Notes |
|-----|-----------------|-------|-----|---------|------|
| 0   | version         | u8    | MUST| —       | Address schema version |
| 1   | nickname        | bstr  | MUST| —       | canonical bytes |
| 2   | namespace       | bstr  | MAY | absent  | canonical bytes |
| 3   | timestamp_ns    | u64   | MUST| —       | ns since Unix epoch |
| 4   | destination_uri | tstr  | MAY | absent  | optional override/compat |
| 5   | ttl_ms          | u32   | MAY | absent  | cache hint |
| 6   | meta            | map   | MAY | absent  | non-critical |
| 7   | sig             | bstr  | MAY | absent  | upper-layer |

### 6.2 AddressBinding CBOR map keys (v1.0)

| Key | Name          | Type        | Req | Default | Notes |
|-----|---------------|-------------|-----|---------|------|
| 0   | version       | u8          | MUST| —       | Binding schema version |
| 1   | nickname      | bstr        | MUST| —       | canonical bytes |
| 2   | namespace     | bstr        | MAY | absent  | canonical bytes |
| 3   | timestamp_ns  | u64         | MUST| —       | ns since Unix epoch |
| 4   | endpoint_kind | u8          | MUST| —       | 0/1/2 |
| 5   | endpoint      | bstr / tstr | MUST| —       | depends on kind |
| 6   | ttl_ms        | u32         | MAY | absent  | cache hint |
| 7   | priority      | u8          | MAY | absent  | tie-break |
| 8   | meta          | map         | MAY | absent  | non-critical |
| 9   | sig           | bstr        | MAY | absent  | upper-layer |

---

## 7. Unknown field handling (forward compatibility)

### 7.1 CBOR map keys
- Unknown keys MUST be ignored.
- Unknown keys MUST NOT cause errors unless explicitly configured as “critical”.

### 7.2 “Critical keys” (optional future mechanism)
This spec does not define critical keys in v1.0.
(If introduced later, it MUST be negotiated explicitly.)

---

## 8. Compact binary record encoding (secondary)

This encoding is for ultra-compact carriers or non-CBOR environments.
It is **not** required if CBOR is always available.

### 8.1 Varint definition
- `uvarint` is **unsigned LEB128** (7-bit groups, little-endian continuation).
- Integers larger than 64-bit are invalid.

### 8.2 AddressRef LP record (v1.0)

Layout:
ver:u8
timestamp_ns:u64be
ns_len:uvarint
ns_bytes:ns_len
nn_len:uvarint
nn_bytes:nn_len
[ dst_len:uvarint dst_bytes:dst_len ] // OPTIONAL: present iff bytes remain


Rules:
- `ns_len=0` means “namespace absent”.
- `dst_bytes` is UTF-8 if present (destination_uri).
- `nn_bytes` MUST satisfy canonical name rules (Section 3).

### 8.3 AddressBinding LP record (v1.0)

Layout:
ver:u8
timestamp_ns:u64be
ns_len:uvarint ns_bytes
nn_len:uvarint nn_bytes
endpoint_kind:u8
ep_len:uvarint ep_bytes
[ ttl_ms:uvarint ] // OPTIONAL if bytes remain
[ priority:u8 ] // OPTIONAL if bytes remain


Rules:
- endpoint interpretation depends on `endpoint_kind`.
- For `node_id` and `ipv6`, `ep_len` MUST be 16.
- For `uri`, `ep_bytes` MUST be UTF-8.

---

## 9. Security considerations (base layer)

Base (MUST/SHOULD):
- Input sanitize (Section 3)
- Rate limiting for resolver queries
- ACL policies for resolver sources (trusted/untrusted)
- Cache poisoning resistance via policy (priority/allowlist)

Upper-layer (MAY, out of scope):
- Signatures over AddressBinding records
- TLS for resolver transport
- PKI / identity attestation

If signatures are used, the signature scheme MUST be specified by a separate document and negotiated.

---

## 10. Minimal compliance tests

1) Canonical name rejection: invalid chars -> ERROR(INVALID_NAME)
2) Deterministic CBOR: same input -> same bytes (for AddressRef/Binding)
3) Unknown CBOR keys are ignored
4) Resolution order is enforced (override > cache > namespace > global)
5) Collision tie-break is deterministic

---

## 11. Versioning & evolution

- This spec is **append-only**.
- New fields MUST be optional unless negotiated.
- Reserved name list is append-only.

Changelog:
- v1.0: Initial nickname addressing spec (Zen-aligned minimal core + optional URI hook)

---

