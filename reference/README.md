# Reference: the MiSTer core simulated with Verilator

**English** · [Português](README.pt-BR.md) · [Español](README.es.md)

The tool that compares V-Bear with a second implementation of the same hardware. It simulates, cycle
by cycle, the (System)Verilog of the
[MiSTer core of the Super A'Can](https://github.com/MiSTer-devel/SuperACan_MiSTer), runs a game and
saves the picture and the work RAM, so they can be compared with what V-Bear produces at the same frame.
The method and the results are in [Accuracy and validation](../docs/accuracy.md).

It is not part of the emulator: nothing here is built by `cargo build`.

## Before you use it: what this reference is and is not

**It is a second reconstruction, not the console.** The MiSTer core was written from reverse engineering
(its README, "Development and QA"): videos of real hardware showing the games' attract screens, recorded by
Diskman, disassemblies of the whole library and earlier documentation. Its author did not test it against a
real console: the console costs thousands of dollars and there is no flash cart. Its own README says that
the library runs "apparently bug free".

What follows for whoever reads the results:

- **Identical to the MiSTer core is not identical to the console.** Two reconstructions agreeing is
  evidence that both are right, not proof. If they agree on a mistake, the comparison does not see it.
- **A disagreement does not say who is wrong.** A difference may be in V-Bear, in the MiSTer core or in this
  simulation. A third side (MAME 0.289) was needed to settle the case of window 0.
- **The core's repository is new:** a single commit (2026-09-11, `59ba275`), with no history of fixes that
  would show what has already been found and corrected.

Limitations of the simulation itself (ours, not the core's):

- **The cartridge is read with a fixed delay** (`CART_LATENCY`, 12 `clk_sys` cycles in `sim_top.sv`). On a
  real MiSTer the ROM comes from SDRAM, with a variable delay. It is a guess, and it changes the boot time
  of each game: this is why the two sides are not on the same frame (see "Alignment" below).
- **No input yet:** the simulation presses no buttons. Only the start of each game is compared (the BIOS
  intro and the start of the game), not menus or matches.
- **Slow:** about 1 frame a second (measured: 120 frames in about 2 min 20 s), so 300 frames take ~5 min a
  game and 3000 frames ~50 min.
- **No sound, no battery save, no save state.** Only the picture and the 64 KB of work RAM.
- **The RAM only makes sense at the same point of the frame.** The two sides stop at slightly different
  points of the frame and leave different leftovers at the top of the 68000's stack and in a few counters;
  that is not an emulation error.

## Use

Requirements: Verilator 5 and a C++17 compiler; Python 3. The MiSTer core is not in this repository: the
`Makefile` looks for it in `_referencias/mister-59ba275` (a local copy, outside git, recorded in
[References](../docs/references.md)); `make MISTER=/other/path` points to another copy.

```bash
make -C reference                          # builds reference/acan-ref (~10 s)
cargo build --release                      # builds V-Bear
FRAMES=300 reference/compare.sh target/release/v-bear "roms/Game.zip" ...
```

`compare.sh` runs each game in both and prints, per game and frame: the **lag** (the offset in frames that
was found), the differing **pixels** and the differing **RAM bytes** (0 = identical). Options: `--jobs N`
(games in parallel; each simulation uses one core), `--bios DIR` (the folder with the four files, default
`bios/`), `--window W` (how many frames around to look for, default 4) and `--offset K`. Frames by
variable: `FRAMES="300 600"`.

The reference alone:

```bash
reference/acan-ref GAME.bin --bios bios --shot 300 out.ppm --dump-ram ram.bin
reference/acan-ref GAME.bin --bios bios --frames-out 290 315 prefix   # several frames as PPM
```

V-Bear writes the equivalent with `V_BEAR_SHOT=<file>` and `V_BEAR_RAM=<file>` (windowless mode,
`--frames N`).

## Alignment

The simulation and V-Bear do not reach the same frame at the same time. In the BIOS intro the offset is 1
frame, constant. At the start of each game it varies from 0 to about 13 frames, depending on the game.
So the comparison looks, in a window of V-Bear's frames, for the one that best matches frame N of the
simulation, and prints the offset (`LAG`). On a still picture many frames give 0 pixels and the `LAG` is not
reliable; on an animation it is.

Two different image formats are normalized before comparing: V-Bear always outputs 320×240 (H256 lines
stretched; the 224-line mode centered with 8 black lines at the top and the bottom), and the simulation
outputs the real size (256×224, 320×224, 320×240...). The comparison applies the same conversion as
`crates/v-bear-core/src/video.rs` and compares exact colors (8 bits per channel).

## What to expect

The table of results and the list of findings are in [Accuracy and validation](../docs/accuracy.md). Two
patterns of difference appear that have not been traced to an emulation error (these are
interpretations, not proofs):

- **A fade whose color channels step on different frames** (Sango Fighter, Gambling Lord): the picture is
  identical every fourth frame, and in the frames between, a region differs by 1 level in one or more
  channels (123 against 132), because V-Bear and the simulation update the three channels on different
  frames of the 4-frame cycle. The cause was not found ([Accuracy](../docs/accuracy.md)).
- **Stack leftovers and counters in the RAM** (a few bytes): the two sides stop at slightly different
  points of the frame. In Boom Zoo, 9 bytes: 2 counters one step apart and 7 bytes at the top of the
  stack. This too is a hypothesis: the RAM was only compared at the same frame, not at several points of
  the frame.

## Files

| File | What it is |
|---|---|
| `sim_top.sv` | the top of the simulation: the MiSTer core's console without the MiSTer framework; the cartridge ROM is an array, read through the same request/busy/done protocol as the SDRAM bridge |
| `sim.h` | the simulation: clock, BIOS and ROM loading, video capture, the frame loop, reading the RAM |
| `ref_main.cpp` | the command line of `acan-ref` |
| `Makefile` | builds with Verilator and copies `microrom.mem` and `nanorom.mem` (`fx68k` reads them from the current directory) |
| `compare.py`, `compare.sh` | run the reference and V-Bear and compare |

How Verilator treats the core: the core's code is read unchanged. To build it, `-Wno-BLKANDNBLK` (a known
limitation of `fx68k` in Verilator) and the order of the files (`mos6502_pkg.sv` first) were needed, and
`--public-flat-rw` must **not** be used (it exposes every signal and makes GCC spend more than 25 minutes on
a single file; the ROM array is public through a comment in `sim_top.sv` itself, and the RAM is read through
the core's `ss_wram` port).

## Licenses

| Part | License |
|---|---|
| The MiSTer core (`_referencias/mister-59ba275`, not included here) | GPL-2.0 (its `LICENSE` file) |
| The rest of this folder | GPL-2.0 or later (as in Mil Capivaras: it is built together with the GPL-2.0 core, and V-Bear is GPL-3.0) |

The reference is a separate program: it is not linked to the emulator.
