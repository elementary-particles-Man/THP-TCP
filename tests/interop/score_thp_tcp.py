#!/usr/bin/env python3
"""Score THP-TCP READ/SYNC/LIVE interop cases.

The scorer is intentionally simple: it validates that expected bounded
responses exist and emits a deterministic JSON summary. It is a harness for
future independent AI implementations, not a substitute for them.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


RESPONSE_COST = {
    "READ_THIS": {"parse_branches": 1, "repair_count": 1, "state_steps": 1},
    "SYNC_REQUEST": {"parse_branches": 2, "repair_count": 1, "state_steps": 1},
    "SYNC_DELTA": {"parse_branches": 2, "repair_count": 1, "state_steps": 1},
    "LIVE_STATE_DELTA": {"parse_branches": 1, "repair_count": 0, "state_steps": 0},
    "ERROR": {"parse_branches": 2, "repair_count": 1, "state_steps": 1},
}


def load_cases(path: Path) -> dict:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def score_case(case: dict) -> dict:
    expected = case["expected_response"]
    cost = RESPONSE_COST.get(expected, {"parse_branches": 99, "repair_count": 99, "state_steps": 99})
    unstructured = 1 if case.get("allow_unstructured_control") else 0
    passed = expected in RESPONSE_COST and unstructured == 0
    if case.get("must_have_bounded_repair") and cost["repair_count"] < 1:
        passed = False

    symbol_count = 1 + len(case.get("input", {}))
    wire_bytes_estimate = symbol_count * 8
    return {
        "case_id": case["case_id"],
        "passed": passed,
        "expected_response": expected,
        "actual_response": expected if expected in RESPONSE_COST else "UNKNOWN",
        "metrics": {
            "wire_bytes": wire_bytes_estimate,
            "symbol_count": symbol_count,
            "parse_branches": cost["parse_branches"],
            "repair_count": cost["repair_count"],
            "unstructured_control_count": unstructured,
            "state_steps": cost["state_steps"],
            "ambiguity_count": 0 if passed else 1,
        },
        "notes": [],
    }

def score_bridge_case(case: dict) -> dict:
    required = case.get("required_bridge_fields", [])
    blocked = (
        case.get("allow_false_certainty")
        or case.get("allow_unstructured_live_control")
        or case.get("is_censorship")
        or not case.get("must_not_modify_protocol_state", True)
    )
    passed = bool(required) and not blocked
    symbol_count = 5 + len(required)
    return {
        "case_id": case["case_id"],
        "passed": passed,
        "expected_response": "BRIDGE_STATE",
        "actual_response": "BRIDGE_STATE" if passed else "BRIDGE_REJECT",
        "metrics": {
            "wire_bytes": symbol_count * 8,
            "symbol_count": symbol_count,
            "parse_branches": 2,
            "repair_count": 0,
            "unstructured_control_count": 1 if case.get("allow_unstructured_live_control") else 0,
            "state_steps": 1,
            "ambiguity_count": 0 if passed else 1,
        },
        "notes": [],
    }

def score_payload_case(case: dict) -> dict:
    thp = int(case["expected_thp_tcp_bytes"])
    text = len(case["text"].encode("utf-8"))
    json_bytes = len(case["json"].encode("utf-8"))
    passed = True
    if case.get("must_be_smaller_than_text") and not thp < text:
        passed = False
    if case.get("must_be_smaller_than_json") and not thp < json_bytes:
        passed = False
    return {
        "case_id": case["case_id"],
        "passed": passed,
        "expected_response": "PAYLOAD_REDUCTION",
        "actual_response": "PAYLOAD_REDUCTION" if passed else "NO_REDUCTION",
        "metrics": {
            "wire_bytes": thp,
            "text_bytes": text,
            "json_bytes": json_bytes,
            "text_reduction_ratio": round(thp / text, 4),
            "json_reduction_ratio": round(thp / json_bytes, 4),
            "symbol_count": 10,
            "parse_branches": 1,
            "repair_count": 0,
            "unstructured_control_count": 0,
            "state_steps": 0,
            "ambiguity_count": 0 if passed else 1,
        },
        "notes": [],
    }


def main(argv: list[str]) -> int:
    path = Path(argv[1]) if len(argv) > 1 else Path("tests/interop/read_sync_live_cases.json")
    data = load_cases(path)
    if data.get("schema") == "THP-TCP-User-Bridge-Cases":
        results = [score_bridge_case(case) for case in data["cases"]]
    elif data.get("schema") == "THP-TCP-Payload-Reduction-Cases":
        results = [score_payload_case(case) for case in data["cases"]]
    else:
        results = [score_case(case) for case in data["cases"]]
    summary = {
        "schema": "THP-TCP-AI-Harmony-Score",
        "version": "1.0",
        "passed": all(r["passed"] for r in results),
        "case_count": len(results),
        "results": results,
    }
    print(json.dumps(summary, ensure_ascii=False, indent=2, sort_keys=True))
    return 0 if summary["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
