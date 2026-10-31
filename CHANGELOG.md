# Changelog

All notable changes to V-Bear.

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
