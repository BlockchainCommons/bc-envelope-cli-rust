#!/usr/bin/env python3

"""Generate 20 XID versions with encrypted keys/generator and sequenced attachments."""

from __future__ import annotations

import argparse
import secrets
import subprocess
import sys


def run(cmd: list[str]) -> str:
    result = subprocess.run(cmd, text=True, capture_output=True)
    if result.returncode != 0:
        stderr = result.stderr.strip()
        stdout = result.stdout.strip()
        detail = stderr or stdout or f"command exited with code {result.returncode}"
        raise RuntimeError(f"{' '.join(cmd)} failed: {detail}")
    return result.stdout.strip()


def modify_options(password: str) -> list[str]:
    return [
        "--private",
        "encrypt",
        "--generator",
        "encrypt",
        f"--password={password}",
        f"--encrypt-password={password}",
        "--verify",
        "inception",
        "--sign",
        "inception",
    ]


def add_sequence_attachment(
    xid_doc: str,
    seq: int,
    vendor: str,
    conforms_to: str,
    password: str,
) -> str:
    payload = run(["envelope", "subject", "type", "number", str(seq)])
    return run(
        [
            "envelope",
            "xid",
            "attachment",
            "add",
            "--vendor",
            vendor,
            "--conforms-to",
            conforms_to,
            "--payload",
            payload,
            *modify_options(password),
            xid_doc,
        ]
    )


def remove_sequence_attachments(xid_doc: str, vendor: str, password: str) -> str:
    attachments = run(["envelope", "xid", "attachment", "find", "--vendor", vendor, xid_doc])
    for attachment in (line.strip() for line in attachments.splitlines() if line.strip()):
        xid_doc = run(
            [
                "envelope",
                "xid",
                "attachment",
                "remove",
                attachment,
                *modify_options(password),
                xid_doc,
            ]
        )
    return xid_doc


def generate_xid_versions(count: int, password: str, vendor: str, conforms_to: str) -> list[str]:
    keypairs = run(["envelope", "generate", "keypairs"])
    keypair_parts = keypairs.split()
    private_keys = keypair_parts[0] if keypair_parts else ""
    if not private_keys:
        raise RuntimeError("Failed to parse private keys from `envelope generate keypairs` output.")

    xid_doc = run(
        [
            "envelope",
            "xid",
            "new",
            private_keys,
            "--private",
            "encrypt",
            "--generator",
            "encrypt",
            f"--encrypt-password={password}",
            "--sign",
            "inception",
        ]
    )

    versions: list[str] = []
    xid_doc = add_sequence_attachment(xid_doc, 0, vendor, conforms_to, password)
    versions.append(xid_doc)

    for seq in range(1, count):
        xid_doc = run(
            [
                "envelope",
                "xid",
                "provenance",
                "next",
                *modify_options(password),
                xid_doc,
            ]
        )
        xid_doc = remove_sequence_attachments(xid_doc, vendor, password)
        xid_doc = add_sequence_attachment(xid_doc, seq, vendor, conforms_to, password)
        versions.append(xid_doc)

    return versions


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Generate XID documents with provenance sequence 0..N-1 and "
            "print each version as ur:xid."
        )
    )
    parser.add_argument(
        "--count",
        type=int,
        default=20,
        help="Number of versions to emit (default: 20).",
    )
    parser.add_argument(
        "--password",
        default=None,
        help="Password used for encrypted private keys and encrypted provenance generators.",
    )
    parser.add_argument(
        "--vendor",
        default="com.blockchaincommons.examples.xid-seq",
        help="Attachment vendor value.",
    )
    parser.add_argument(
        "--conforms-to",
        default="https://developer.blockchaincommons.com/xid/provenance-seq/v1",
        help="Attachment conformsTo URI.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.count < 1:
        print("--count must be at least 1.", file=sys.stderr)
        return 2

    password = args.password or secrets.token_urlsafe(24)
    versions = generate_xid_versions(args.count, password, args.vendor, args.conforms_to)
    print("\n".join(versions))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(f"Error: {exc}", file=sys.stderr)
        raise SystemExit(1)
