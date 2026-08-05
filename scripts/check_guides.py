#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""校验每个练习目录是否包含合格的中文 教学.md"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REQUIRED_SNIPPETS = [
    "本题目标",
    "如何运行",
    "cargo test",
]


def main() -> int:
    missing = []
    incomplete = []
    ok = 0
    for cargo in sorted(ROOT.glob("exercises/*/*/Cargo.toml")):
        guide = cargo.parent / "教学.md"
        rel = guide.relative_to(ROOT)
        if not guide.exists():
            missing.append(str(rel))
            continue
        text = guide.read_text(encoding="utf-8")
        bad = [s for s in REQUIRED_SNIPPETS if s not in text]
        # must mention package test form or cd path
        if "cargo test -p" not in text and "cargo test" not in text:
            bad.append("cargo test 命令")
        if bad:
            incomplete.append(f"{rel}: 缺少 {', '.join(bad)}")
            continue
        ok += 1

    print(f"通过: {ok}")
    if missing:
        print("缺失 教学.md:")
        for m in missing:
            print(" ", m)
    if incomplete:
        print("内容不完整:")
        for m in incomplete:
            print(" ", m)
    if missing or incomplete:
        return 1
    print("全部练习教学文档校验通过。")
    return 0


if __name__ == "__main__":
    sys.exit(main())
