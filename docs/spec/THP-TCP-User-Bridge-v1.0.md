# THP-TCP User Bridge v1.0

Status: Normative boundary spec

Purpose: define how an external user gets value from THP-TCP without becoming part of THP-TCP traffic.

THP-TCP is AI-to-AI. The user is not a THP-TCP speaker. The user interacts through a bridge that translates goals, constraints, consent, and results into AI-side state.

The bridge is normally implemented by a translation AI. THP-TCP itself may remain rigid, deterministic, and narrow. The translation AI absorbs flexible input/output requirements at the boundary.

---

## 1. Boundary

The User Bridge is outside LIVE traffic.

The bridge is the only soft boundary. Inside THP-TCP, frames, hashes, state transitions, and repair paths stay strict.

It MAY:

- accept a user goal
- define constraints
- request progress summaries
- surface decisions
- present results
- request rollback or stop

It MUST NOT:

- inject unstructured control into LIVE
- redefine protocol state
- bypass READ/SYNC/LIVE admission
- require the user to understand token tables
- require the user to judge AI-side optimality

---

## 2. User-Facing Contract

The user provides:

- `goal`
- `constraints`
- `risk_tolerance`
- `approval_policy`
- `stop_condition`
- `output_format`

The bridge returns:

- `plan_summary`
- `progress_summary`
- `decision_points`
- `risk_summary`
- `final_result`
- `audit_trace_ref`

The user should see outcomes, tradeoffs, and control points, not THP-TCP frames.

The translation AI MAY choose any suitable external expression for the user, as long as it preserves the deterministic internal state and does not inject unstructured control into LIVE.

---

## 3. Translation Into THP-TCP State

The bridge converts user input into:

- initial `state_id`
- task dictionary entries
- constraints as deterministic fields
- approval gates as explicit repair/hold states
- result requirements as output schema

The bridge then lets AI speakers operate through READ/SYNC/LIVE.

The bridge also converts AI-side results back into the user's requested output format. This reverse translation is presentation only; it MUST NOT alter audit state or protocol state.

---

## 3.1 Conversation Confirmation, Not Censorship

The bridge MAY provide a confirmation view of what THP-TCP speakers exchanged.

This is not censorship, filtering, or moderation. It is a translation of protocol state into an inspectable summary so the user can confirm:

- what goal was pursued
- what assumptions were used
- what evidence was referenced
- what uncertainty remained
- what decisions were made
- what repair paths were triggered
- what result was produced

The confirmation view MUST NOT rewrite THP-TCP traffic. It MUST be derived from logs, state hashes, audit references, and final state.

The bridge MAY redact secrets or keys required for security, but it MUST preserve enough structure for the user to understand what was discussed and why.

---

## 4. User Control Points

The bridge MUST preserve:

- start
- pause
- stop
- approve
- reject
- request summary
- request evidence
- change constraints

These are bridge commands. They are not LIVE control payloads unless converted into deterministic THP-TCP state updates.

---

## 5. No User Burden

The user MUST NOT be asked to decide whether the AI-side protocol is optimal.

The user judges:

- Did the system pursue the stated goal?
- Were constraints respected?
- Were risks surfaced?
- Is the result usable?
- Is the audit trail sufficient?
- Can the user confirm what THP-TCP speakers exchanged at the level of goals, assumptions, evidence, uncertainty, decisions, repairs, and results?

The AI-side harmony evaluator judges protocol efficiency and interoperation.

---

## 6. Minimal Flow

```text
USER_GOAL
  -> bridge creates task state
  -> AI speakers READ/SYNC/LIVE as needed
  -> bridge receives final state/result
  -> bridge emits user result + audit reference
```

---

## 7. Summary

THP-TCP does not make the user speak an AI protocol.

The bridge lets users express intent and receive results while AI speakers handle low-load semantic synchronization internally.

Rigid core, flexible translation boundary.

---

## 8. Domain Range

The bridge MUST support both low-structure and high-structure domains.

Low-structure examples:

- local observation
- rumor
- social context
- historical narrative
- ambiguous testimony
- scene reconstruction

High-structure examples:

- market analysis
- telemetry
- ledgers
- compliance evidence
- time-series data
- multi-source forecasts

The bridge MUST NOT require a different user contract for these domains. The same contract applies:

```text
goal + constraints + risk_tolerance + approval_policy + stop_condition + output_format
```

The internal AI-side representation MAY differ, but the user boundary stays stable.

---

## 9. Analysis Span Contract

For any domain, the bridge converts user intent into a deterministic analysis state:

- `domain_class`
- `source_profile`
- `uncertainty_model`
- `time_scope`
- `evidence_policy`
- `risk_policy`
- `output_schema`

The bridge MUST preserve uncertainty. It MUST NOT convert weak observation into false certainty.

Required uncertainty fields:

- `confidence`
- `source_strength`
- `contradiction_points`
- `missing_evidence`
- `assumption_set`
- `next_evidence_needed`

---

## 10. Examples

### 10.1 Low-Structure Observation

Input goal:

```text
Summarize what is probably happening from informal local reports.
```

Bridge state:

- `domain_class = local_social_observation`
- `source_profile = informal`
- `uncertainty_model = high_context_high_noise`
- `evidence_policy = preserve_source_strength`
- `output_schema = summary + confidence + contradictions + next_evidence_needed`

### 10.2 High-Structure Market Analysis

Input goal:

```text
Analyze NY market movement and produce risk-aware scenarios.
```

Bridge state:

- `domain_class = financial_market_analysis`
- `source_profile = structured_time_series`
- `uncertainty_model = probabilistic_scenario`
- `evidence_policy = cite_data_windows`
- `risk_policy = no_unqualified_prediction`
- `output_schema = drivers + scenarios + risk + confidence + audit_trace_ref`

---

## 11. User Load Rule

The user MUST NOT be asked to translate domain complexity into protocol complexity.

The bridge absorbs that burden. The user supplies intent and constraints; the AI speakers perform domain-specific synchronization internally.
