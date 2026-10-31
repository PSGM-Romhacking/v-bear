#!/usr/bin/env bash
# Builds V-Bear for Linux (glibc 2.27 or newer):
#   dist/v-bear-<version>-linux-x86_64.AppImage   the emulator
#   dist/v_bear_libretro.so (+ .info)             the libretro core (RetroArch for Linux)
#
# Usage:
#   ./packaging/build_linux.sh
#   GH_REPO=v-bear ./packaging/build_linux.sh   also embeds the update information
#                                                (GitHub releases of PSGM-Romhacking/<repo>)
#
# Requirements: cargo (rustup), cargo-zigbuild and Zig (pip install ziglang), the ALSA
# headers (alsa-lib-devel / libasound2-dev) and objcopy (binutils). On the first run it downloads appimagetool
# (~9 MB) into build/tools/. Based on Mil Capivaras' packaging/build_linux.sh.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

export PATH="$HOME/.cargo/bin:$HOME/.venvs/py314/bin:$PATH"
# oldest glibc the binary accepts. 2.27 = Ubuntu 18.04 / Debian 10 / RHEL 8; built
# straight on Fedora, the binary would require Fedora's glibc
GLIBC="${GLIBC:-2.27}"
TARGET="x86_64-unknown-linux-gnu"
ARCH="x86_64"
VERSION="$(sed -n '/^\[workspace.package\]/,/^\[/ s/^version *= *"\(.*\)"/\1/p' Cargo.toml | head -1)"
BIN="v-bear"   # also the Wayland app_id and the .desktop/icon name
BUILD="$ROOT/build"
APPDIR="$BUILD/AppDir"
TOOLS="$BUILD/tools"
OUT="$ROOT/dist/v-bear-${VERSION}-linux-${ARCH}.AppImage"

# automatic updates (Gear Lever, AppImageUpdate) only once there is a GitHub repository
GH_USER="${GH_USER:-PSGM-Romhacking}"
GH_REPO="${GH_REPO:-}"
UPDATE=()
if [ -n "$GH_REPO" ]; then
    UPDATE=(-u "gh-releases-zsync|${GH_USER}|${GH_REPO}|latest|v-bear-*-linux-${ARCH}.AppImage.zsync")
fi

echo "==> V-Bear v${VERSION} (${ARCH})"

# 1. binaries: the emulator and the libretro core
echo "==> cargo zigbuild --release (glibc >= $GLIBC)..."
# the build machine's libasound needs its newer glibc; on the user's system, its libasound
# matches its glibc, so the linker must not check the libraries' own symbols
# zig's linker turns the profile's strip = "debuginfo" into a full strip, losing the function
# names the error reports (crashlog) show; so no strip here and only the debug info goes, below
RUSTFLAGS="${RUSTFLAGS:-} -C link-arg=-Wl,--allow-shlib-undefined" CARGO_PROFILE_RELEASE_STRIP=false \
    cargo zigbuild --release -p v-bear -p v-bear-libretro --target "${TARGET}.${GLIBC}"
for f in "target/$TARGET/release/$BIN" "target/$TARGET/release/libv_bear_libretro.so"; do objcopy --strip-debug "$f"; done
mkdir -p dist
cp "target/$TARGET/release/libv_bear_libretro.so" dist/v_bear_libretro.so
cp crates/v-bear-libretro/v_bear_libretro.info dist/

# 2. AppDir
echo "==> AppDir..."
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin" "$APPDIR/usr/share/applications" \
    "$APPDIR/usr/share/icons/hicolor/scalable/apps" "$APPDIR/usr/share/icons/hicolor/256x256/apps"
cp "target/$TARGET/release/$BIN" "$APPDIR/usr/bin/$BIN"
cp packaging/app.desktop "$APPDIR/$BIN.desktop"
cp packaging/app.desktop "$APPDIR/usr/share/applications/$BIN.desktop"
cp assets/icons/v-bear-256.png "$APPDIR/usr/share/icons/hicolor/256x256/apps/$BIN.png"
cp assets/icons/v-bear.svg "$APPDIR/usr/share/icons/hicolor/scalable/apps/$BIN.svg"
cp assets/icons/v-bear-256.png "$APPDIR/$BIN.png"
ln -sf "$BIN.png" "$APPDIR/.DirIcon"
ln -sf "usr/bin/$BIN" "$APPDIR/AppRun"

# 3. appimagetool
APPIMAGETOOL="$(command -v appimagetool || true)"
if [ -z "$APPIMAGETOOL" ]; then
    APPIMAGETOOL="$TOOLS/appimagetool-${ARCH}.AppImage"
    if [ ! -x "$APPIMAGETOOL" ]; then
        echo "==> Downloading appimagetool..."
        mkdir -p "$TOOLS"
        curl -fL -o "$APPIMAGETOOL" \
            "https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-${ARCH}.AppImage"
        chmod +x "$APPIMAGETOOL"
    fi
fi

# 4. package
if [ -n "$GH_REPO" ] && ! command -v zsyncmake >/dev/null; then
    echo "NOTE: zsyncmake not found, the .zsync (incremental update) won't be generated."
fi
echo "==> AppImage..."
# replacing a running AppImage corrupts the process: build in a staging folder, already
# with the final name (the .zsync records it), then move
STAGE="$BUILD/out"
rm -rf "$STAGE" && mkdir -p "$STAGE"
(
    cd "$STAGE"   # appimagetool writes the .zsync in the current folder
    ARCH="$ARCH" "$APPIMAGETOOL" --appimage-extract-and-run "${UPDATE[@]}" "$APPDIR" "$(basename "$OUT")" 2>/dev/null \
        || ARCH="$ARCH" "$APPIMAGETOOL" "${UPDATE[@]}" "$APPDIR" "$(basename "$OUT")"
)
mv -f "$STAGE/$(basename "$OUT")" "$OUT"
if [ -f "$STAGE/$(basename "$OUT").zsync" ]; then
    mv -f "$STAGE/$(basename "$OUT").zsync" "${OUT}.zsync"
fi

echo ""
echo "==> Done: $OUT ($(du -h "$OUT" | cut -f1))"
echo "    and dist/v_bear_libretro.so + .info (RetroArch: cores folder)"
