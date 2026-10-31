# Accuracy and validation

**English** · [Português](accuracy.pt-BR.md) · [Español](accuracy.es.md)

How V-Bear is compared with other implementations of the Super A'Can, the result, and what is still
missing. **V-Bear has not been compared with a real console**, and the references it was compared with
are themselves reconstructions: read "Limits of the reference" before the numbers.

The references in brackets ([n]) are in [References](references.md).

## How the comparison is done

**1. The two "consoles"**
- **The reference:** the MiSTer core of the Super A'Can is the description of the board's circuit (in
  SystemVerilog). A program (Verilator) simulates that circuit on the PC, clock by clock. It is slow, but it
  is the MiSTer core itself running.
- **V-Bear:** the emulator, written in Rust, fast. When MAME and the MiSTer core disagree it follows the
  MiSTer core ([Hardware](hardware.md)).

If V-Bear is right, the two have to show **the same thing** at the same moment.

**2. The same game**

Both run the same ROM with the same BIOS, with nobody playing: no button is pressed. So what is compared
is the start of each game, up to frame 300 (about 5 seconds), not menus or matches.

**3. Pause and snapshot at the same frame**

After N frames both stop and save:
- **the picture:** at its real size (256 or 320 dots wide, 224 or 240 lines);
- **the work RAM:** the 64 KB the 68000 uses.

**4. Compare, dot by dot**

A script compares the files: how many pixels have a different color (0 = identical picture), and how many
RAM bytes have a different value.

**5. The two never boot at the same speed**

The same game reaches the same picture some frames apart (0 to 13 here, and it varies from game to
game), because the simulation reads the cartridge with a fixed delay that is not the real one. So frame N
of the simulation is compared with a window of V-Bear's frames, and the best one is kept: the one with
the fewest differing pixels, and among equals the one with the fewest differing RAM bytes. The offset
found is part of the result (`LAG`). The consequence is that a still picture proves little about the
alignment, and the RAM is what settles it.

**6. When it comes out different: look at where**

A difference is measured, not guessed. The bounding box of what differs, the values of the pixels and,
when the cause is unclear, a third source (MAME) are looked at before calling it a bug.

## Limits of the reference

This is the part that matters most for how to read the numbers.

- **The MiSTer core is a reconstruction, not the console.** Its README ("Development and QA") says that it
  was built from reverse engineering: videos of real hardware showing the games' attract screens
  (recorded by Diskman), disassemblies of the whole library, and earlier documentation. Its author did not
  test it against a console: the console costs thousands of dollars and there is no flash cart. The same
  README says the library runs "apparently bug free".
- **So "identical to the MiSTer core" means two independent reconstructions agree.** That is evidence that
  both are right, not proof. If they agree on a mistake, this comparison does not see it. If they disagree,
  it does not say which one is wrong: a third side was needed (below).
- **The core's repository is new:** one commit (2026-09-11, `59ba275`), with no history of fixes.
- **The simulation is ours and has its own limits:** the cartridge is read with a fixed delay (a guess); no
  buttons are pressed; no sound, no save state and no battery save are compared; it runs at about one frame a
  second. Details in [`reference/README.md`](../reference/README.md).

## Methodology (technical details)

1. **Reference:** the MiSTer core's SystemVerilog simulated cycle by cycle with Verilator,
   in [`reference/`](../reference/README.md), without changing the core's code: `sim_top.sv` replaces the
   MiSTer framework (clock, cartridge as an array, BIOS loading) and `sim.h` captures the picture from the
   core's video output and reads the work RAM through its savestate port.
2. **Same input:** none (no button pressed), BIOS loaded from the same four files.
3. **Same output:** the picture and the 64 KB of work RAM at frame N.
4. **Normalizing the picture:** V-Bear always outputs 320×240 (H256 lines stretched; the 224-line mode centered
   with 8 black lines at the top and bottom) and the simulation outputs the real size; the comparison applies
   `video.rs`'s conversion to the simulation's frame and compares exact colors (8 bits per channel).
5. **Alignment:** a window of ±12 frames of V-Bear around the expected one, as described above.
6. **Commands:**

```bash
make -C reference
FRAMES=300 reference/compare.sh --jobs 6 --window 12 --cache /tmp/ref-cache target/release/v-bear roms/*.zip
```

7. **Third source:** MAME 0.289 (Flatpak `org.mamedev.MAME`), driven by `tools/mame/drive.lua`, with the
   snapshot of each frame. It was used to decide the window 0 difference below.

## Result

At frame 300, with no button pressed, 12 games, MiSTer core simulated (`LAG` is V-Bear's offset in frames,
and is not reliable on a still picture):

| Game | LAG | Differing pixels | Differing RAM bytes |
|---|---|---|---|
| Boom Zoo | 5 | **0** | 1 |
| Formosa Duel | 9 | **0** | 9 |
| Gambling Lord | 2 | 5,629 | 124 |
| Journey to the Laugh | 9 | **0** | 2 |
| Magical Pool | 3 | **0** | 451 |
| Monopoly: Adventure in Africa | 2 | **0** | 1 |
| Rebel Star | 0 | **0** | **0** |
| Sango Fighter | −2 | 10,643 | 52 |
| Speedy Dragon | 6 | **0** | 37 |
| Super Dragon Force | 16 | **0** | 868 |
| Super Taiwanese Baseball League | 2 | **0** | 13 |
| The Son of Evil | −4 | **0** | 6 |

- **Picture identical, pixel by pixel and color by color, in 10 of 12 games.**
- **RAM:** identical in Rebel Star; at most 13 differing bytes of 65,536 in 6 more (Boom Zoo, Formosa Duel,
  Journey, Monopoly, Baseball, The Son of Evil).
- **Also compared, outside this table:** the BIOS intro (the "A'can" logo), frames 100 to 125 of Boom Zoo, is
  identical in all 26 frames (0 pixels of 71,680 at 5 bits per color channel; checked at 8 bits per
  channel on frame 120 only), with a constant offset of 1 frame.

**What had to be fixed to get there:**

| Problem | Where | Effect before the fix |
|---|---|---|
| Window clipping used table entry L−1 for line L (`render.rs`, `window`) | video | a shape drawn by window 0 (Magical Pool's growing circle) one line too low: 76 pixels in a ring at frame 300, 122 and 176 at frames 310 and 315. After the fix: 0 pixels at frames 290, 300 and 305 (3, 23 and 61 at 295, 310 and 315, not investigated) |

**How the window case was decided.** V-Bear read the table entry of the previous line because the MiSTer
core's source says "the pair fetched during line L-1 clips line L". The core's `render_y` is already
`vpos + 1`, so what it fetches during line L−1 is entry L. MAME indexes the table by the line itself. In
the simulation the circle spans rows 101 to 138 (center 119.5) and V-Bear drew it at 102 to 139 (center
120.5); with V-Bear shifted back one line the difference went from 76 pixels to 4. The circle in MAME's
snapshots is also centered on row 119.5, in the frames measured (298 and 301). Two independent references
against V-Bear.

## What is still different

- **Sango Fighter (picture).** The logo fades in with its three color channels (R, G, B), each going up
  one level on a different frame of a 4-frame cycle. Measured at frames 290 to 305 (same frame number on both
  sides, mean value of each channel): in the simulation the steps happen at frames 291 (G), 292 (R) and
  294 (B), and every 4 frames after that; in V-Bear at 293 (G), 294 (R) and 292 (B). Every fourth frame
  (290, 294, 298, 302, 306, 310, 314) the three channels have the same value on both sides and the picture is
  **identical, 0 pixels, at lag 0**; in the other three frames of each cycle 9,000 to 12,600 pixels differ, by
  1 level in one or more channels. So both produce the same fade, and what differs is **on which frame of the
  cycle each channel is updated**. The cause was not found: the game's own code, the phase of a timer or an
  interrupt, or the simulation's fixed cartridge delay could each produce it.
- **Gambling Lord (picture).** Identical at 8 of the 26 frames from 290 to 315 (292, 294, 298, 302, 306, 309,
  310 and 314) and 1,900 to 5,600 pixels at the others, with the same kind of difference (1 or 2 levels in a
  color channel). Not analyzed channel by channel.
- **Magical Pool, Speedy Dragon and Super Dragon Force (RAM).** 451, 37 and 868 bytes differ with the
  picture identical. Not investigated. Super Dragon Force comes in two ROM files (two mask ROMs); an
  earlier run of the comparison used only one of them (a bug in `compare.py`, fixed), so its number was
  discarded. The 868 is with the whole 3 MB and a window of ±40 frames (the best alignment, 16, is inside it).
- **A few bytes in the other games' RAM** (1 to 13): in Boom Zoo at frame 120, 9 bytes: 2 counters one step
  apart and 7 bytes at the top of the 68000's stack. Probably because the two sides stop at slightly
  different points of the frame (not proven; the RAM was compared only at one point of the frame).
- **Later frames, menus and matches.** Nothing past frame 300 and no button pressed was compared, except the
  tests against MAME in [Compatibility](compatibility.md).

## Classification

V-Bear does **not** claim a level on the Emulation General Wiki scale: it was not assessed, and the
reference it was compared with is not the console. What can be said is limited to "compared with the MiSTer
core simulated, in the cases above, and with MAME 0.289 in the scenes listed in
[Compatibility](compatibility.md)".

## Validation against the console

None. There are no measurements taken on a real Super A'Can, by us or by others, and no test ROMs with
photos of a real console. The MiSTer core's own validation (reverse engineering from videos) is the closest
there is, and it is described above.

## Status

| # | Item | Status |
|---|---|---|
| 1 | Window 0 clipped by the wrong table entry (a shape one line too low) | **done:** Magical Pool, 0 pixels at frames 290, 300 and 305 |
| 2 | Sango Fighter and Gambling Lord: the color channels of a fade are updated on different frames of the cycle | pending: cause unknown; the picture is identical every 4th frame |
| 3 | RAM differs in Magical Pool, Speedy Dragon and Super Dragon Force | pending: not investigated |
| 4 | Compare past frame 300 and with buttons pressed | pending: the simulation has no input yet |
| 5 | The simulation reads the cartridge with a fixed delay (a guess) | pending: a different delay would change the alignment, not the result |
| 6 | Compare sound | pending: the simulation has no sound output |
| 7 | Compare with a real console | no change possible: there is none to measure |

## How each part was validated

Each part of the emulation, with its proof. Only the MiSTer core simulated and MAME serve as references;
none of them is the console.

| Part | Proof | Strength |
|---|---|---|
| Picture (tiles, sprites, backgrounds, priorities, palette) | 0 differing pixels against the simulated MiSTer core in 10 of 12 games at frame 300; BIOS intro identical in 26 frames | strong, within the first 300 frames |
| Windows and per-line tables | Magical Pool's circle: 0 pixels at frames 290, 300 and 305, decided with the simulation and with MAME | strong for window 0; the other windows were not isolated |
| 68000 and 65C02 (the `m68k` and `mos6502` crates) | indirect: the games run and the RAM is identical in Rebel Star and within 13 bytes in 6 more | weak: no instruction test suite was run |
| Interrupts and timing | same RAM and picture as the simulation; the Sango Fighter and Gambling Lord fade is **not** identical | partial |
| Sound | none against a reference: the simulation has no audio output | none |
| Controls | scenes played against MAME 0.289, listed in [Compatibility](compatibility.md) | partial |
| Battery save | saved and loaded by hand in 2 of the 6 games with a battery | partial |
| Two-part game (Super Dragon Force) | joined file with CRC32 `c4fe1ca1`, the one in the game database (`games.rs`) | strong |

## Each item in detail

**2. Sango Fighter and Gambling Lord.** To find the cause, log the 68000's writes to the palette
(`$F00200` to `$F003FF`: frame, line, entry, value) in V-Bear and in the simulation, and see on which frame
each channel is written. If the game writes them on the same frame on both sides and only the picture
differs, it is a rendering question; if it writes them on different frames, the difference is in the timing
that drives the game (an interrupt or a timer), in V-Bear or in the simulation.

**3. The RAM.** The simulation stops at the end of a frame and V-Bear at the end of its own; a game that
keeps a counter or a table that changes during the frame will differ by a few bytes at any alignment. To
separate that from a real difference, compare the RAM at several points of the frame, and list which
addresses differ.

**4. Buttons.** `--press-start` in the Mil Capivaras comparison presses Start at fixed frames in both. The
simulation would need the pads wired in `sim_top.sv` (the `pad` input already exists) and a script of
presses in `sim.h`; V-Bear already has `V_BEAR_INPUT`.

## Suggested order

Item 1 is done. What is left, from the cheapest to the most expensive:

| # | Item | Effort | Gain |
|---|---|---|---|
| 3 | RAM in Magical Pool, Speedy Dragon and Super Dragon Force: compare at several points of the frame | small | tells a real difference from a stop-point artifact |
| 2 | Sango Fighter and Gambling Lord: log the palette writes on both sides | medium | finds (or rules out) a timing error in V-Bear |
| 4 | Buttons in the simulation | medium | extends the comparison beyond the first 300 frames |
| 5 | Cartridge delay in the simulation | small | only changes the alignment |
| 6 | Sound | large | needs an audio output in the simulation |
