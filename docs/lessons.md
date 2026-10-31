# Lessons learned

**English** · [Português](lessons.pt-BR.md) · [Español](lessons.es.md)

What the project taught, for someone doing the same on another system. Each lesson comes from something
that happened here; the numbers are in [Accuracy and validation](accuracy.md) and
[Compatibility](compatibility.md).

## Approach

- **Have two implementations to compare with, not one.** V-Bear was written from the MiSTer core and
  MAME. Neither is the console: the MiSTer core is a reconstruction too (one commit, no test on a real
  console). When they disagree, a third opinion is needed. The window bug (below) was only decided
  because MAME agreed with the MiSTer core against V-Bear.
- **A comment in the source is a claim, not a measurement.** V-Bear clipped each line with the window
  table entry of the *previous* line because the MiSTer core says "the pair fetched during line L-1 clips
  line L". What the sentence leaves out is that the core's `render_y` is already `vpos + 1`, so the pair it
  fetches is entry L. The picture was wrong by one line until the comparison showed it.
- **"Identical to the reference" never means "identical to the console".** Say which reference, which
  frames and which games, and keep what was not compared on the list of what is missing.

## Comparing with a simulated FPGA core

- **Simulate the core's Verilog with Verilator and compare picture and RAM.** It is the only way here to
  run the MiSTer hardware without a MiSTer. The cost is speed: about one frame a second for this console,
  so 300 frames take five minutes a game.
- **Do not build with `--public-flat-rw`.** It makes Verilator expose every signal of a core with tens
  of thousands of them, and the C++ compiler spends more than 25 minutes on a single generated file. Mark
  only what you read with a `public_flat_rw` comment, and read the rest through ports the core already has
  (the work RAM here is read through the savestate port).
- **Small build details that cost an afternoon:** the 6502 package has to come first in the file list;
  `-Wno-BLKANDNBLK` for the 68000 core (`fx68k`); and `fx68k` reads two `.mem` microcode files from the
  *current directory*, so the simulation has to run next to them or the 68000 does not work, with only a
  warning on the screen.
- **Two implementations never boot at the same speed.** The same game reaches the same picture some
  frames apart (0 to 13 here, and it varies per game), because the cartridge read takes a different time.
  Compare frame N of one with a window of frames of the other, and report the lag.
- **A tie is not a result.** On a still picture many frames give 0 differing pixels; taking the first one
  compared the RAM at the wrong frame (15,000 differing bytes in one game). Break ties by the frame
  closest to the expected one.
- **Look at what differs before deciding it is a bug.** Two groups of differences were not
  traced to an error: a fade whose three color channels step on different frames of a 4-frame cycle (the
  picture is identical every fourth frame; the cause was not found), and a few bytes of the 68000's stack and counters, probably because
  the two sides stop at slightly different points of the frame (not proven). One difference was real: a
  shape clipped by the video chip's window, drawn one line too low.
- **Check that the reference ran the same game.** The first comparison fed the simulation only one of the two
  files of Super Dragon Force (a game on two ROM chips; the zip lists the 1 MB chip first), so its RAM result
  (8,682 bytes) meant nothing: with the parts joined it was 868. When a number looks odd, check the input
  before blaming either emulator.
- **Measure where a shape is, not only how many pixels differ.** The window bug looked like noise (76
  pixels in a ring); the circle's bounding box showed that it was exactly one line lower, and the count
  dropped from 76 to 4 when it was shifted back.

## Interface and RetroArch

- **Identify the BIOS by CRC32, not by name.** The same four files come from MAME, MiSTer and dumps under
  many names, and MAME's sets do not hold all of them (the `umc6650.bin` key is in its own set).
- **RetroArch caches the core information.** After a new `.info` the core showed as `v_bear_libretro`, with no
  description or licence, and the file browser hid `.bin` and `.can`, until the cache was cleared. The
  symptom looks like a bug in the core and is not.
- **Test the libretro core without RetroArch first.** A small program that plays RetroArch's part through
  the C API (BIOS lookup, frames, save state, battery RAM) finds the obvious faults in seconds; what is
  left is what only RetroArch shows, like the cache above.
- **A frontend base saves months,** but the system's own needs (a required BIOS in several files, battery
  saves, games in two files) go beyond the base's contract and have to be added with care.

## Hardware model

- **Follow the board, not the manual.** When MAME and the MiSTer core disagree on clocks or timing, the
  core that models the whole board and runs the whole library wins, and the disagreement is written down
  ([Hardware](hardware.md)).
- **Give the 68000 time on the sound-RAM window.** Speedy Dragon loses a race between a timeout and the
  sound driver's initialization if the access costs nothing, and reloads the driver forever.
