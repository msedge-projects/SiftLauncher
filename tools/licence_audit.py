#!/usr/bin/env python3
"""Licence and provenance audit for SiftLauncher."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
NOTICE_FILE = ROOT / "THIRD_PARTY_NOTICES.md"
GPL_WORD = "G" + "P" + "L"
LGPL_WORD = "L" + GPL_WORD
AGPL_WORD = "A" + GPL_WORD

CODE_EXTENSIONS = {".rs", ".toml", ".json", ".jsonc", ".yaml", ".yml", ".md"}
ASSET_DIRS = (ROOT / "assets", ROOT / "data")
EXCLUDED_PARTS = {".git", "target", ".github", "docs"}
EXCLUDED_FILES = {NOTICE_FILE.resolve(), Path(__file__).resolve()}


def fail(messages: list[str]) -> None:
    for message in messages:
        print(f"FAIL: {message}")
    sys.exit(1)


def walk_files():
    for path in ROOT.rglob("*"):
        if path.is_dir():
            continue
        if any(part in EXCLUDED_PARTS for part in path.relative_to(ROOT).parts):
            continue
        if path.resolve() in EXCLUDED_FILES:
            continue
        yield path


def cargo_packages() -> list[dict]:
    proc = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--locked"],
        cwd=ROOT,
        text=True,
        capture_output=True,
    )
    if proc.returncode != 0:
        proc = subprocess.run(
            ["cargo", "metadata", "--format-version", "1"],
            cwd=ROOT,
            text=True,
            capture_output=True,
        )
    if proc.returncode != 0:
        print(proc.stderr, file=sys.stderr)
        fail(["cargo metadata failed"])
    data = json.loads(proc.stdout)
    return data.get("packages", [])


def main() -> None:
    errors: list[str] = []

    for package in cargo_packages():
        license_text = str(package.get("license") or "")
        has_copyleft = any(word in license_text for word in (GPL_WORD, LGPL_WORD, AGPL_WORD))
        has_permissive = any(word in license_text for word in ("MIT", "Apache-2.0", "ISC", "BSD", "OFL", "Unicode"))
        if has_copyleft and not has_permissive:
            errors.append(f"copyleft dependency: {package.get('name')} {license_text}")

    lower_banned = [GPL_WORD.lower(), LGPL_WORD.lower(), AGPL_WORD.lower(), "general public license", "copyleft"]
    source_patterns = [r"\.vue", r"\.scss", r"\.ts:", r"PalantirMC", r"ModrinthApp", r"modrinth/app"]

    for path in walk_files():
        try:
            text = path.read_text(errors="ignore")
        except OSError:
            continue
        lower = text.lower()
        if any(word in lower for word in lower_banned):
            errors.append(f"copyleft marker in {path.relative_to(ROOT)}")
        for pattern in source_patterns:
            if re.search(pattern, text):
                errors.append(f"source-format citation {pattern!r} in {path.relative_to(ROOT)}")

    if NOTICE_FILE.exists():
        notices = NOTICE_FILE.read_text(errors="ignore")
    else:
        errors.append("THIRD_PARTY_NOTICES.md is missing")
        notices = ""

    for directory in ASSET_DIRS:
        if not directory.exists():
            continue
        for path in directory.rglob("*"):
            if path.is_dir():
                continue
            rel = str(path.relative_to(ROOT))
            if rel not in notices:
                errors.append(f"unlisted asset: {rel}")

    cargo_toml = (ROOT / "Cargo.toml").read_text()
    m = re.search(r"license\s*=\s*\"([^\"]+)\"", cargo_toml)
    if not m:
        errors.append("workspace.package license missing")
    else:
        print(f"workspace license: {m.group(1)}")

    license_path = ROOT / "LICENSE"
    if not license_path.exists():
        errors.append("LICENSE missing")
    else:
        print(f"LICENSE: {license_path.read_text(errors='ignore').splitlines()[0] if license_path.read_text(errors='ignore') else ''}")

    if errors:
        fail(errors)
    print("audit: ok")


if __name__ == "__main__":
    main()
