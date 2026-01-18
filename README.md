# THP-TCP (aka LSC-TCP / AI-TCP)

THP-TCP is an **AI-to-AI communication protocol** optimized for:

- **low overhead**
- **deterministic parsing**
- **forward-only evolution**
- **post-hoc semantic reconstruction**

Human readability is **not** a priority for the *wire/log format*. Humans should read **derived reports**, not raw traffic.

> Repository philosophy: **log first, then standardize via small addenda**.  
> Old logs are **read-only**; new formats move forward.

---

## What THP-TCP is (and is not)

### IS
- A **semantic exchange layer** for AI agents
- A **token + binary framing** scheme
- A **loggable** conversation-object format for later AI analysis
- Designed to flow over constrained carriers (e.g., **IPv6 Extension Header** mesh transport)

### IS NOT
- A replacement for TCP/QUIC
- A reasoning engine / training method
- A safety/alignment framework
- A human-facing chat format

---

## Specs (single source of truth)
- AI Guidance: `docs/spec/THP-TCP-AI-Guidance-v1.0.md`
- Nickname Addressing: `docs/spec/THP-TCP-Nickname-Addressing-v1.0.md`
  - Table: `docs/spec/tables/thp-tcp-nickname-fields-v1.0.json`

- Draft baseline:
  - `docs/spec/THP-TCP-Draft.md`
- Normative addenda (forward-only):
  - `docs/spec/THP-TCP-v1.0.1-Addendum.md`
  - `docs/spec/THP-TCP-v1.0.2-Addendum.md` *(minimal patch: FLAGS + LEN:u16be + fragmentation rules)*

Tables:
- `docs/spec/tables/thp-tcp-fixed-tokens-*.json`

Primary development log (ground truth of evolution):
- `logs/flux-zen-talk-log.txt`

---

## Protocol overview (high level)

THP-TCP evolves along this path:

1. **Natural language / JSON** (initial exploration)
2. **Tokenization** (opcode tables)
3. **Binary framing** (compact, deterministic)
4. **Carrier-aware segmentation** (MTU-driven fragmentation)
5. **AEAD envelope** (AES-256-GCM)
6. **AI-oriented metadata** (intent, emotion vectors)

The key rule is **determinism**: the same semantic object should map to the same byte patterns whenever possible.

---

## Core concepts

### 1) Token table (opcodes)
- `uint8` opcode space (0–255)
- `0–127` are **fixed global tokens**
- `128–255` are **contextual tokens** bound to a negotiated `context_id`

**No opcode is ever redefined.**  
Contextual tokens require a **dictionary snapshot** exchange first.

### 2) Bootstrap (dictionary exchange)
A session MUST start with a bootstrap handshake:

1. `HELLO` (capabilities + negotiation)
2. `DICT_SNAPSHOT` (dictionary snapshot, chunked if needed)
3. `DICT_ACK` (hash verification; accepted/rejected/resend)

Contextual tokens (128–255) MUST NOT be used before `DICT_ACK(accepted)`.

### 3) Payload encoding
- Default: **CBOR (deterministic / canonical required)**
- Streaming only: **CBOR-Seq**
- Text-only transport: **Base64 wrapper (carrier-level only)**

### 4) Carrier size model (IPv6 EH / MTU-aware)
- Logical message size: **unbounded**
- On-wire datagram size: bounded by negotiated `max_datagram`
  - Safe default: ~1200 bytes
  - Closed mesh (MTU=1500 typical): ~1400 bytes (accounts for AEAD overhead)

Large messages MUST be **fragmented**.

### 5) Fragmentation (CONT)
Fragmentation is represented by **flags**, not by overloading semantic tokens.

- `F_CONT`: continuation / fragmentation
- `F_LAST`: last fragment (single-fragment messages may set both)

Bit values (normative):
- `F_CONT = 0x80`
- `F_LAST = 0x40`

When `F_CONT` is set, the fragment PAYLOAD begins with a fixed fragment header:
- `msg_id` (16 bytes, 128-bit)
- `part_no` (u8, 0-based)
- `part_total` (u8)
- `orig_token` (u8)
- `frag_bytes...`

This supports explicit notation like “1/5” (`part_no=0`, `part_total=5`).

### 6) Encryption envelope (AES-256-GCM)
When encryption is enabled for the datagram:
- `PAYLOAD := [nonce:12][ciphertext:N][tag:16]`

Nonce uniqueness per key MUST be guaranteed.

---

## Wire format (current)

### Phase0 (v1.0.2)
Minimal universal frame:

[TOKEN:u8][FLAGS:u8][LEN:u16be][PAYLOAD:LEN]


- `TOKEN`: opcode
- `FLAGS`: bitmask (e.g., `F_CONT`, `F_LAST`)
- `LEN`: payload length
- `PAYLOAD`: bytes (typically deterministic CBOR; or AEAD envelope)

> Why LEN:u16be?  
> Phase0 should not impose an artificial 255B ceiling when the carrier constraint is MTU and we already negotiate `max_datagram`.

### Phase1 (reserved / extended header)
Phase1 introduces larger fields (u32 length, u64 timestamp ns epoch, 128-bit message_id, etc.).  
See addenda for activation gate and requirements (v1.0.1+ / v1.0.2+).

---

## AI metadata: intent & emotion

THP-TCP supports side-channels for analysis:

- `intent`: `u8`
- `emotion`: default **5D vector**, `u8 x 5` (5 bytes)
  - Optional compact form: `[tag:u8][score:u8]`

**Important:** semantics of dimensions/tags are **out of protocol**. The protocol only defines bytes and stability.

---

## Directory layout (recommended)

.
├─ README.md
├─ docs/
│  └─ spec/
│     ├─ THP-TCP-Draft.md
│     ├─ THP-TCP-v1.0.1-Addendum.md
│     ├─ THP-TCP-v1.0.2-Addendum.md
│     └─ tables/
│        └─ thp-tcp-fixed-tokens-*.json
└─ logs/
   └─ flux-zen-talk-log.txt


---

## Development workflow (the “way we do it”)

1. **Record actual exchanges** in `logs/flux-zen-talk-log.txt`
2. When a stable rule emerges, freeze it as a **minimal Addendum**
3. Never rewrite old logs; never mutate old token meanings
4. Keep the wire format deterministic and machine-parseable

This workflow is the development itself: “how we got here” is part of the protocol.

---

## Minimal compliance checklist (implementation)

An implementation is compliant with Phase0 (v1.0.2) if it:

- Parses `[TOKEN][FLAGS][LEN:u16be][PAYLOAD]`
- Enforces deterministic CBOR for CBOR payloads
- Implements bootstrap (`HELLO`, `DICT_SNAPSHOT`, `DICT_ACK`)
- Negotiates and respects `max_datagram`
- Supports fragmentation flags + fragment header reassembly
- Treats old logs as read-only; produces forward-only logs

---

## Status

- Protocol: **experimental**
- Current stable baseline: **Phase0 v1.0.2**
- Next steps: reference implementation + compliance tests

---

## License

(TODO: choose a license)
