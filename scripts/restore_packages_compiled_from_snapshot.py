#!/usr/bin/env python3
"""Restore packages_compiled blobs from bytecode_snapshot SystemPackage files."""

from __future__ import annotations

import struct
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SNAPSHOT_DIR = ROOT / "crates/myso-framework-snapshot/bytecode_snapshot/112"
OUT_DIR = ROOT / "crates/myso-framework/packages_compiled"

PACKAGE_MAP = {
    "0x0000000000000000000000000000000000000000000000000000000000000001": "move-stdlib",
    "0x0000000000000000000000000000000000000000000000000000000000000002": "myso-framework",
    "0x0000000000000000000000000000000000000000000000000000000000000003": "myso-system",
    "0x000000000000000000000000000000000000000000000000000000000000000b": "bridge",
    "0x0000000000000000000000000000000000000000000000000000000000000b0c": "orderbook",
    "0x000000000000000000000000000000000000000000000000000000000000da7a": "mydata",
    "0x00000000000000000000000000000000000000000000000000000000000050c1": "myso-social",
    "0x000000000000000000000000000000000000000000000000000000000000e110": "messaging",
    "0x000000000000000000000000000000000000000000000000000000000000c1fe": "contra",
}


def read_uleb128(data: bytes, offset: int) -> tuple[int, int]:
    result = 0
    shift = 0
    while True:
        if offset >= len(data):
            raise ValueError("unexpected end of input while reading uleb128")
        byte = data[offset]
        offset += 1
        result |= (byte & 0x7F) << shift
        if (byte & 0x80) == 0:
            return result, offset
        shift += 7
        if shift >= 64:
            raise ValueError("uleb128 too large")


def write_uleb128(value: int) -> bytes:
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        if value:
            out.append(byte | 0x80)
        else:
            out.append(byte)
            break
    return bytes(out)


def encode_vec_vec_u8(modules: list[bytes]) -> bytes:
    out = bytearray()
    out.extend(write_uleb128(len(modules)))
    for module in modules:
        out.extend(write_uleb128(len(module)))
        out.extend(module)
    return bytes(out)


def decode_vec_vec_u8(data: bytes, offset: int) -> tuple[list[bytes], int]:
    count, offset = read_uleb128(data, offset)
    modules: list[bytes] = []
    for _ in range(count):
        length, offset = read_uleb128(data, offset)
        modules.append(data[offset : offset + length])
        offset += length
    return modules, offset


def decode_system_package(data: bytes) -> list[bytes]:
    if len(data) < 32:
        raise ValueError("snapshot file too small")
    offset = 32
    modules, offset = decode_vec_vec_u8(data, offset)
    # dependencies: Vec<ObjectID>
    dep_count, offset = read_uleb128(data, offset)
    offset += dep_count * 32
    if offset != len(data):
        raise ValueError(f"trailing bytes after decode: {len(data) - offset}")
    return modules


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    for snapshot_path in sorted(SNAPSHOT_DIR.iterdir()):
        if not snapshot_path.is_file():
            continue
        name = PACKAGE_MAP.get(snapshot_path.name)
        if name is None:
            continue
        modules = decode_system_package(snapshot_path.read_bytes())
        out_path = OUT_DIR / name
        out_path.write_bytes(encode_vec_vec_u8(modules))
        print(f"restored {name} ({len(modules)} modules, {out_path.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
