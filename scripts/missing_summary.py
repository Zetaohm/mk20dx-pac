#!/usr/bin/env python3
"""Quick summary of MISSING_IN_SVD issues from comparison JSON reports."""
import json
import sys

for report_path in sys.argv[1:] or ["reports/mk20d5_comparison.json", "reports/mk20d7_comparison.json"]:
    with open(report_path) as f:
        r = json.load(f)
    print(f"\n=== {report_path} ===")
    missing = r["by_category"].get("MISSING_IN_SVD", [])
    periphs = {}
    for item in missing:
        p = item.get("peripheral") or "(global)"
        periphs[p] = periphs.get(p, 0) + 1
    for p, n in sorted(periphs.items(), key=lambda x: -x[1]):
        print(f"  {p:12s}: {n}")
