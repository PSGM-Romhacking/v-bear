# Changelog

All notable changes to V-Bear.

## [0.5.2] - 2026-11-03

- **Magical Pool: the cue vanished in the shot-power menu.** A sprite's horizontal position wraps at 512 (MAME
  has the same rule, noting this game); V-Bear dropped the pixels past that edge, so a cue pulled back beyond
  -128 disappeared instead of showing its far end at the left of the screen. Checked against the 12 games with a
  4,000-frame random-input script: no other frame changed.
- **Journey to the Laugh: the hero vanished while hit (seen on Windows).** The game hides the hero every other
  frame to flicker it. When the window could not keep up and emulated two or three frames before showing one,
  only the last was shown, which could always be the hidden one. Those frames are now averaged, so the hero
  shows half-transparent instead of disappearing.
- **Sango Fighter: the horizon was cut and wobbled.** The game copies 4,096 words of graphics to VRAM at the start of
  every frame and programs a raster interrupt for line 42 to change the scroll of the layers. V-Bear held the CPU
  8 cycles per word on that copy (about 48 lines), so the interrupt ran 11 to 14 lines late and the top of the
  trees was drawn with the wrong scroll. A copy into the UM6618's own VRAM costs one four-clock bus cycle per
  word (the read; the chip writes on its own bus), as in the MiSTer's `um6619_dma.sv`, so the interrupt now runs
  at its line. Against the MiSTer simulation at frame 300 nothing got worse in the 12 games (Sango Fighter is now
  aligned and its RAM difference fell from 52 to 25 bytes). Debug switches: `V_BEAR_DEBUG=sdma` (sprite DMA with its
  line) and `sprscale` (scaled sprites).
- **Super Dragon Force: the menus showed every frame at once.** The frames are drawn by the ROZ layer, and the game
  hides the ones it does not use with a per-line source-row table. V-Bear read that table only when tile-mode
  bit 14 was set, but the game sets bit 15 alone (`0x8500`), so the table was ignored. Bit 15 now turns it on by
  itself; bit 14 still adds the X tables. The shop menu shows one clean bar; against the MiSTer simulation at frame
  300 no picture changed in the 12 games.
- Windowless mode: `V_BEAR_STATE=<file>` starts from a save state written by the interface, to reproduce a bug
  from the exact moment it was saved; `V_BEAR_VRAMN=<n>` makes `V_BEAR_VRAM` print n words.

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
