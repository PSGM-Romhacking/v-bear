# Compatibility

**English** · [Português](compatibility.pt-BR.md) · [Español](compatibility.es.md)

State of each game in V-Bear, with the BIOS. Update after every round of tests, without deleting the
previous ones (the history lives in the dated sections).

ROMs: the 12 from the No-Intro set, checked by CRC32 against the MAME softlist (see `references.md`).

## How to test

Everything windowless, with `--frames N` (the BIOS is found in `bios/`):

```sh
# captures every 450 frames, with a button script (frame:button, held for 8 frames)
V_BEAR_INPUT="1500:start,1560:a" V_BEAR_SHOT=/tmp/g V_BEAR_SHOT_EVERY=450 \
  ./target/release/v-bear "roms - Funtech - Super Acan/<game>.zip" --frames 7200
# sound to WAV (44,744 Hz, stereo)
V_BEAR_WAV=/tmp/game.wav ./target/release/v-bear <rom> --frames 1800
```

The final summary shows IRQs serviced per level, palette/VRAM in use, audio peak, SRAM bytes,
`faults` (illegal exceptions, A-line, F-line of the 68000) and `halted` (CPU stopped by a double fault).
Other keys: `V_BEAR_REGS`, `V_BEAR_VRAM=<word>`, `V_BEAR_SND=<addr>`, `V_BEAR_LIVE=<frame>`
(traces the 68000), `V_BEAR_PCS`, `V_BEAR_DEBUG=snd,t65,dma,irq,watch,vregs,sndr,pal`,
`V_BEAR_SNAPTEST=<frame>` (deterministic savestate), `V_BEAR_RESET=<frame>` (resets there),
`V_BEAR_STATE=<file>` (starts from a save state written by the interface, to reproduce a bug from the moment it was saved), `V_BEAR_RAM=<file>` (writes the 64 KB of work RAM at the end, to compare with `reference/`).
`V_BEAR_SHOT` without `_EVERY` writes only the last screen, as PNG if the name ends in `.png`.

## 2026-10-21

Two scripts: a fixed one (Start/A/directions every 60 frames, 7,200 frames) and a random one
(seed 7, one button every 12 frames, 20,000 frames ≈ 5.5 min of play). In all 12 games:
`faults=0`, `halted=false`.

| Game | How far it got | Notes |
|---|---|---|
| Boom Zoo | puzzle matches against several opponents | — |
| Formosa Duel | title, match | — |
| Gambling Lord | character select, mahjong matches | — |
| Journey to the Laugh | four platform stages, spider boss | — |
| Magical Pool | title, mode menu, map | the random script does not leave the title (it needs Start at the right moment) |
| Monopoly – Adventure in Africa | board, days advancing, buying and selling | — |
| Rebel Star (En) | intro, title, map with dialogue | — |
| Sango Fighter | select, fights in two stages | fixed on this date: the fight was covered by window 0 (IRQ4 did not turn on) |
| Speedy Dragon | intro, several stages | fixed on 2026-10-20: driver × timeout race (waits in the sound RAM) |
| Super Dragon Force | title, dungeon, world map | — |
| Super Taiwanese Baseball League | menus, match with scoreboard | uses the battery SRAM (1,849 bytes written) |
| The Son of Evil | title, Start leads to the lake scene | — |

**Not verified yet:** sound fidelity (only checked that there is music and effects), games played to
the end, 2-player mode, side-by-side comparison with MiSTer/real console.

## 2026-10-21 (2nd round)

After fixing the speed of the 65C02 (it was running ~3.5% fast): same random script of
20,000 frames, `faults=0` and `halted=false` in all 12. New things seen: Magical Pool reaches the
pool table; Gambling Lord reaches the mahjong table with the four players. Speedy Dragon is still fine: with
the right speed, 1 wait cycle on the sound RAM is enough (V-Bear uses 2; with 0 the driver is
reloaded endlessly). Savestate (F5/F9) tested: restoring and running 600 frames gives an identical screen
in all 12 games.

## 2026-10-21 — comparison with MAME 0.289

Tool: `tools/mame/compare.sh <jogo.zip>` (outside git; see `references.md`). It captures the
same frames in both emulators, builds the side-by-side images and counts differing pixels (MAME
shows 224 lines; V-Bear has 8 more at the top and bottom).

- **Image:** in the scenes where both are at the same point, the screens match. Speedy Dragon, title
  at frame 1800: 99.2% of pixels equal, and the difference is only the character's animation frame.
  Formosa Duel, title: 95.6%, the difference is the position of the clouds. Logos (Funtech, Panda, AV
  artisan), titles of Boom Zoo, Gambling Lord, Monopoly, Super Dragon Force, Journey to the Laugh and the
  Super Taiwanese Baseball stadium checked by eye, equal.
- **Sound:** Speedy Dragon, 6 s of title music in a spectrogram: same notes, timbres and tempo
  (the pattern repeats every 1.6 s in both); volume of the same order. The music starts ~0.5 s earlier
  in V-Bear.
- **Sequence:** it diverges, because MAME advances by itself through titles and menus with no input at all (its
  ports stay at rest; cause not investigated, see `references.md`). Examples with no input:
  in MAME, Sango Fighter cuts the intro at frame ~1100 and enters a fight; Formosa Duel goes
  to the opponent selection; Rebel Star and The Son of Evil are left with a black screen or with
  garbage. In V-Bear the games stay on the title and the demos, alternating, with nobody playing.

## 2026-10-21 — savestate on disk

`V_BEAR_SNAPTEST=900 … --frames 2400`: writes the state in the file format at frame 900, runs to
2400, loads it into a new machine, runs again and compares the image and the whole state (byte by byte).
Identical in all 12 games; each state is ~260 KiB. A unit test covers the 65C02 stopped at WAI and the
NMI edge. The F5/F6/F7/F9 keys in the window have not been tested by hand yet.

## 2026-10-21 — usage keys

Reset (F1) checked windowless: Sango Fighter reset at frame 600 and run for 1800 more gives the
same screen (pixel by pixel) and the same IRQ counters as 1800 frames from boot. Screen PNG
identical to the PPM capture. Pause, frame advance, Tab and F12 in the window not yet tested by hand.

## 2026-10-21 — migration to the base interface

The core moved to `crates/v-bear-core` without changing behavior: with the same button script,
the screen at frame 2400 is identical, byte by byte, to that of the minifb version in all 12 games. Per the contract
(`cargo test -p v-bear-core --test contract`, with `V_BEAR_TEST_ROM` and `V_BEAR_TEST_BIOS`): Super
Dragon Force (two parts), Sango Fighter and Rebel Star identified in the database, with image, audio at the
requested rate and exact save state. libretro core: loads Sango Fighter with the BIOS in the
*system* folder (`real_core` test of `libretro-host`). Interface: automatic captures (`EMU_DOCSHOTS`)
with Sango Fighter running, the game sheet and the new folders (BIOS, battery saves). Not yet
tested by hand in the interface: controls, rewind, recording, battery save.

## 2026-10-21 — battery save (SRAM), version 0.5.0

**Hardware:** one 32 KiB 62256 SRAM (U2) on the PCCAR 16003-1B board, at `$EC0000-$ECFFFF`, on the low
half of the bus only: byte `n` sits at `$EC0000 + 2n + 1`. The MiSTer solders the SRAM into every
cartridge; the MAME softlist declares a 32 KiB `nvram` in 6 games (some with "size and presence not
confirmed"). V-Bear's `.sav` has 32,768 bytes in that order, the same as MAME's `nvram` and the MiSTer
`.sav`.

**Measured accesses** (`reads`/`writes` counters in the windowless-mode summary, 3600 frames with Start/A):

| Game | SRAM in MAME | Reads | Writes | Note |
|---|---|---|---|---|
| Super Taiwanese Baseball League | yes | 29,968 | 2,162 | formats on power-on; with the `.sav` written, it reads 10 bytes and does not reformat |
| Super Dragon Force | yes | 24,240 | 0 | looks for saves already at the title |
| Rebel Star | yes | 1 (when choosing LOAD GAME) | 0 | without a save, returns to the title |
| The Son of Evil | yes | 0 | 0 | save and load confirmed by hand on 2026-10-30 |
| Monopoly | yes | 0 | 0 | LOAD GAME opens 5 saves and starts a game without touching `$EC` or any address outside the map (`V_BEAR_DEBUG=unmapped`); where it keeps progress is an open question |
| Gambling Lord | yes (presence not confirmed) | 0 | 0 | |
| the other 6 | no | 0 | 0 | |

**Round trip (baseball):** `V_BEAR_SAV=x.sav` in windowless mode reads the file before and writes it after.
First run: reads 10 bytes, formats (2,162 writes), writes `x.sav`. Second: reads the same 10
bytes, 0 writes, the same screen at frame 3000.

**Fixed:** the interface looked for the `.sav` only in the saves folder (`~/.config/v-bear/saves/`); what the
old version left next to the ROM was ignored. Now, without the file in the saves folder, it uses the
`<game>.sav` from the game's folder and copies it to the saves folder on the next write (checked: identical
copy).

**For the testers** (what can only be checked by playing):
1. Save and load in each game with SRAM: Son of Evil, Super Dragon Force and Rebel Star (save
   points), Monopoly (5 saves), Gambling Lord, baseball (season).
2. Close V-Bear right after saving and check that the save is still there (the `.sav` is written up to ~5 s
   after the change, when closing the game and on exit).
3. Bring a `.sav` from the MiSTer or the MAME `nvram` and see whether the game recognizes it.

## 2026-10-30 — RetroArch core and saves

Core built for Linux (glibc 2.27), Windows, FreeBSD and macOS (only compiled; FreeBSD and macOS were not
run). The Linux `.so` was tested by a program that simulates RetroArch (ctypes):

- BIOS found loose in `system/`, in `system/supracan/`, in a 4-file `.zip` and in MAME's `supracan.zip`
  with `umc6650.bin` loose next to it. MAME 0.289's `supracan.zip` alone has only 3 files
  (without the `umc6650.bin` key), so it is not enough. According to a Reddit report (not verified here), the
  older zip had `internal_68k.bin` and `umc6650.bin`, and the newer one replaced the second with the
  two `internal_6502`: in both cases files are missing for V-Bear.
- Save state: serializes 267 KB and restoring gives the same frame in the games tested (Boom Zoo, baseball).
- Battery save: the core hands over 32 KB; in baseball the game writes 1,849 bytes and the SRAM comes back identical
  in a second session. Son of Evil, Super Dragon Force, Rebel Star, Monopoly and Gambling Lord stay
  with empty SRAM in 3000 frames without buttons (they only write on a real save).
- Confirmed by hand: **The Son of Evil** saves and loads. Super Dragon Force, Rebel Star, Monopoly
  and Gambling Lord remain to be tested.
- The 10 `.bin` files checked have the No-Intro CRC32.

## 2026-10-30 — comparison with the simulated MiSTer core

The 12 games were compared with the Super A'Can MiSTer core, simulated with Verilator (`reference/` folder),
at frame 300, pressing no buttons, with the same BIOS on both sides. Method, numbers and
limits in [accuracy.md](accuracy.md).

- **Image identical, pixel by pixel and color by color, in 10 of the 12 games.** Gambling Lord and Sango Fighter
  differ in some frames: the color channels of a fade are updated on different frames of a 4-frame cycle, and the picture is identical every fourth frame (cause not found).
- **Finding and fix:** window 0 used the `L−1` entry of the table to clip line `L`, and Magical Pool's
  growing circle came out one line lower. The MiSTer (`render_y = vpos + 1`) and MAME 0.289
  agree with each other against V-Bear; fixed in `render.rs`. Before: 76 differing pixels at frame 300;
  after: 0.
- **The MiSTer core is not the console.** It was made by reverse engineering (real hardware videos, disassemblies),
  without testing on a console, and the repository has a single commit. "Identical to the MiSTer" is the agreement of two
  reconstructions, not a proof.
- **RAM:** identical in Rebel Star, with at most 13 differing bytes in 6 more games; Magical Pool,
  Speedy Dragon and Super Dragon Force have differences not yet investigated.
