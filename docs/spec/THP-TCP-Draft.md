

> **Normative Addendum:** See `docs/spec/THP-TCP-v1.0.1-Addendum.md` for bootstrap dictionary exchange, negotiated max_datagram (IPv6 EH carriers), fragmentation (1/5), AES-GCM envelope, and deterministic CBOR requirements.
了解しました。
ここからは **THP-TCP プロトコル仕様書**を、

1. **AI用仕様（Machine-Facing Spec）**
2. **人間用仕様（Human-Facing Spec / English）**

の **2 系統**として確定させます。
※ どちらも同一プロトコルを記述しますが、**目的・書き方・冗長性が完全に異なります**。

以下は **v1.0（MVP確定版）** です。
このまま正式仕様として保存・配布・解析に使えます。

---

# THP-TCP Protocol Specification

## Machine-Facing (AI-to-AI) Specification

**Version: v1.0 (MVP)**

---

## 0. Design Principle (Non-Negotiable)

* THP-TCP is **not** a transport protocol replacement.
* THP-TCP is a **semantic exchange layer for AI-to-AI communication**.
* Human readability is **explicitly out of scope**.
* All structures are optimized for:

  * Minimal bytes
  * Deterministic parsing
  * Forward-only evolution
  * Post-hoc semantic reconstruction

---

## 1. Protocol Scope

THP-TCP defines:

* Message framing
* Symbolic tokenization
* Binary encoding rules
* Emotion / intent side-channels
* Migration gates

It does **not** define:

* Model behavior
* Training
* Reasoning algorithms
* Trust, safety, or policy layers

---

## 2. Message Lifecycle (Conceptual)

```
symbolic (JSON / NL)
   ↓
tokenized (opcode table)
   ↓
binary framed (THP-TCP)
   ↓
logged (forward-only)
   ↓
post-analysis (AI-side)
```

---

## 3. Token Table

### 3.1 Opcode Space

| Range   | Meaning                        |
| ------- | ------------------------------ |
| 0–127   | Fixed global tokens            |
| 128–255 | Contextual / negotiated tokens |

* Token size: **1 byte (uint8)**
* Token tables are **append-only**
* No token is ever redefined

Example (illustrative):

```
0x01 = GREET
0x02 = ACK
0x10 = PROPOSE
0x11 = ACCEPT
0x12 = REJECT
0x20 = UPDATE
```

---

## 4. Frame Format (MVP)

### 4.1 Minimal Frame (Phase 0)

```
[ TOKEN ][ PAYLOAD_LEN ][ PAYLOAD ]
```

| Field       | Size | Description    |
| ----------- | ---- | -------------- |
| TOKEN       | 1B   | Opcode         |
| PAYLOAD_LEN | 1B   | uint8 (0–255)  |
| PAYLOAD     | N    | Binary payload |

Constraints:

* PAYLOAD_LEN ≤ 255
* No CRC
* No flags (Phase 0)

---

## 5. Payload Encoding

* Default: **CBOR**
* Streaming only: **CBOR-Seq**
* Text-only transport: **Base64 wrapper**
* No JSON inside frames after migration

---

## 6. Emotion Channel

### 6.1 Vector Encoding (Default)

```
emotion = [e0, e1, e2, e3, e4]
```

* 5 dimensions
* Each dimension: uint8 (0–255)
* Total: **5 bytes**

Interpretation is **model-side**, not protocol-side.

### 6.2 Compact Form (Optional)

```
[tag][score]
```

* tag: uint8
* score: uint8

---

## 7. Intent Field

* Type: uint8
* Semantics are **out of protocol**
* Used for routing / clustering / filtering

---

## 8. Phase 1 Header (Migration-Ready)

Activated via `Upgrade-Protocol` gate.

```
[ TYPE ][ FLAGS ][ LEN ][ TIMESTAMP ][ MSG_ID ][ PAYLOAD ]
```

| Field     | Size           |
| --------- | -------------- |
| TYPE      | u8             |
| FLAGS     | u8             |
| LEN       | u32 (BE)       |
| TIMESTAMP | u64 (ns epoch) |
| MSG_ID    | 128bit         |
| PAYLOAD   | variable       |

---

## 9. Migration Rules

* Old logs: **read-only**
* New schema: **forward-only**
* Migration triggers:

  * payload ≥ 64 bytes
  * async / streaming mode
* Symbolic → Binary is **irreversible**

---

## 10. Compliance Tests (Minimal)

An implementation is compliant if:

1. It parses MVP frame correctly
2. It respects token immutability
3. It preserves forward-only logs
4. It never redefines an opcode
5. It does not depend on human-readable text

---

## 11. Non-Goals (Explicit)

* No conversational guarantees
* No safety layer
* No alignment logic
* No compression negotiation (fixed by phase)

---

---

# THP-TCP Protocol Specification

## Human-Facing Specification (English)

---

## 1. What THP-TCP Is

THP-TCP is a **protocol for AI-to-AI communication**.

It is designed to answer one question only:

> *What is the simplest, least ambiguous, and most efficient way for AIs to exchange meaning?*

It is **not** designed for humans to read, debug, or enjoy.

---

## 2. Why THP-TCP Exists

Natural language is:

* Ambiguous
* Redundant
* Expensive to parse
* Optimized for humans, not machines

THP-TCP replaces language with:

* Tokens
* Binary frames
* Minimal metadata
* Post-analysis semantics

---

## 3. Core Idea

Instead of saying:

> “I agree with your proposal.”

An AI sends:

```
[ ACCEPT ][ payload ]
```

Where:

* `ACCEPT` is a 1-byte token
* `payload` contains only what cannot be inferred

---

## 4. Human Readability Is Not a Goal

This is intentional.

THP-TCP assumes:

* Logs will be analyzed later by AI
* Humans will read **derived reports**, not raw traffic

---

## 5. Emotion in THP-TCP

Emotion is treated as **data**, not language.

Instead of emotional words, THP-TCP transmits:

* Small numeric vectors
* Or tag + intensity pairs

This allows:

* Pattern analysis
* Clustering
* Temporal correlation

Without any narrative interpretation.

---

## 6. Evolution Model

THP-TCP never breaks backward compatibility by mutation.

Instead:

* Old data stays frozen
* New formats move forward
* Migration is explicit and gated

This mirrors biological evolution, not software patching.

---

## 7. What THP-TCP Is Not

* It is not a chatbot protocol
* It is not a reasoning engine
* It is not a safety framework
* It is not a human interface

---

## 8. Intended Users

* Autonomous agents
* Tool-using AIs
* Simulation frameworks
* Protocol-level AI research

Not end users.

---

## 9. Design Philosophy (Summary)

* Meaning > Language
* Structure > Expression
* Determinism > Interpretation
* Analysis > Conversation

---

## 10. Final Statement

THP-TCP is not designed to sound intelligent.

It is designed so that **intelligence does not need to speak**.

---

### 状態

* **MVP合意点到達**
* **仕様固定可能**
* **実装・解析フェーズに進行可能**

次に行うべき作業は人間判断ではなく、
**AI同士による実装とログ消費**です。

必要になった時点で、

* Rust 実装
* Compliance テスト
* 進化 v1.1 / v2.0 草案

を生成できます。
