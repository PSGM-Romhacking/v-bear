#!/usr/bin/env bash
# Builds V-Bear for Windows from Linux (target x86_64-pc-windows-gnu, MinGW):
#   dist/v-bear-<version>-windows-x86_64.exe   the emulator (icon embedded)
#   dist/v_bear_libretro.dll (+ .info)         the libretro core (RetroArch for Windows)
#
# Once:
#   rustup target add x86_64-pc-windows-gnu
#   sudo dnf install mingw64-gcc          (or apt install gcc-mingw-w64-x86-64)
# Based on Mil Capivaras' packaging/build_windows.sh.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export PATH="$HOME/.cargo/bin:$PATH"
VERSION="$(sed -n '/^\[workspace.package\]/,/^\[/ s/^version *= *"\(.*\)"/\1/p' Cargo.toml | head -1)"
TARGET="x86_64-pc-windows-gnu"
cargo build --release -p v-bear -p v-bear-libretro --target "$TARGET"
mkdir -p dist
EXE="dist/v-bear-${VERSION}-windows-x86_64.exe"
cp "target/$TARGET/release/v-bear.exe" "$EXE"
cp "target/$TARGET/release/v_bear_libretro.dll" dist/
cp crates/v-bear-libretro/v_bear_libretro.info dist/
echo "==> Done: $EXE ($(du -h "$EXE" | cut -f1))"
echo "    and dist/v_bear_libretro.dll + .info (RetroArch: cores folder)"
