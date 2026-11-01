# Changelog

All notable changes to V-Bear.

## [0.5.1] - 2026-11-01

- **RetroArch core, save states of a fixed size.** libretro requires the size reported by `retro_serialize_size`
  to never grow while a game is loaded (RetroArch sizes its rewind buffer from the first answer). The state
  varied by about 150 bytes (the CPU is saved as text), so a state could be refused after the size was
  read. The core now always reports 272 KiB and pads the state with zeros (the largest state measured, over the 12
  games and 4,000 frames each, is 267,322 bytes).
- **RetroArch core, rewind.** The info file said `savestate_features = "basic"`, and RetroArch then refuses
  to rewind ("Rewind unavailable because this core lacks serialized save state support"). It now says
  `"serialized"`. Reproduced and checked with RetroArch 1.22.2: the rewind buffer is created.
- **RetroArch core info file:** license in libretro's format (`GPLv3`), `needs_fullpath`, a description,
  and the CRC32 of each BIOS file.
- **CI file for libretro's buildbot** (`.gitlab-ci.yml`): Linux x64 and Windows x64 builds of the core.

## [0.5.0] - 2026-10-31

First release.

- Emulates the 12 released games of the Funtech Super A'Can (68000, 65C02, UM6618/UM6619), with sound.
- Graphical interface: game library with box art and info sheets, controls, save states, rewind,
  GIF and MP4 recording, debugger, cheats; English, Portuguese and Spanish.
- Battery saves; games in several files (Super Dragon Force) can be joined into one `.can`
  (Tools › Join game parts…, or `v-bear --join`).
- RetroArch core, with the BIOS in `system/supracan/`, loose or inside a `.zip` or `.7z`.
- Packages for Linux (AppImage), Windows and FreeBSD.
- Picture identical to the simulated MiSTer core in 10 of 12 games, with no button pressed; not
  compared with a real console ([accuracy](docs/accuracy.md)).
