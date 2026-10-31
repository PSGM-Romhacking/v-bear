# V-Bear — cargo shortcut
#   make                    builds (release) and copies to ./v-bear-<version>-linux-<arch>
#   make libretro           libretro core: target/release/libv_bear_libretro.so (+ .info)
#   make install-libretro   copies the core into RetroArch's cores folder

# executable name with the version (from Cargo.toml), system and architecture
VERSION := $(shell sed -n '/^\[workspace.package\]/,/^\[/ s/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
BIN     := v-bear-$(VERSION)-linux-$(shell uname -m)

# always runs: cargo already knows what to rebuild
.PHONY: v-bear libretro install-libretro
v-bear:
	cargo build --release -p v-bear
	install -m 755 target/release/v-bear ./$(BIN)   # replaces it even while the program is open

libretro:
	cargo build --release -p v-bear-libretro

# RetroArch's cores folder, read from its retroarch.cfg (Flatpak first, then native). The
# .info goes next to the .so: the Flatpak's info folder is read-only.
RA_CFG := $(firstword $(wildcard $(HOME)/.var/app/org.libretro.RetroArch/config/retroarch/retroarch.cfg $(HOME)/.config/retroarch/retroarch.cfg))
RA_CORES = $(shell sed -n 's/^libretro_directory = "\(.*\)"/\1/p' "$(RA_CFG)" | sed "s|^~|$(HOME)|")

install-libretro: libretro
	@test -n "$(RA_CORES)" || { echo "RetroArch cores folder not found"; exit 1; }
	install -m 755 target/release/libv_bear_libretro.so "$(RA_CORES)/v_bear_libretro.so"
	install -m 644 crates/v-bear-libretro/v_bear_libretro.info "$(RA_CORES)/v_bear_libretro.info"
	@echo "installed in $(RA_CORES)"
