# Architecture and how it works

**English** · [Português](architecture.pt-BR.md) · [Español](architecture.es.md)

How the code is organized, how the emulation works and the reasoning behind each decision.
V-Bear is a Cargo workspace with the console separate from the
interface; they only know each other through a small contract (`emu-api`).

## Structure

```
crates/api/             core ↔ frontend contract (no dependencies)
crates/v-bear-core/   Funtech Super A'Can: machine, memory, game database, tests
crates/frontend/        reusable frontend (egui, menus, library, controllers,
                        save states, rewind, recording, covers, languages, config)
crates/v-bear/        executable: core + frontend + branding (name, icon, About)
crates/v-bear-libretro/   v_bear_libretro.so = core + libretro API
crates/libretro-host/, libretro-player/, toy*/   from the base: a host for ready-made libretro cores
                        and a toy example; V-Bear does not use them
assets/                 icons, fonts, bezels, i18n/<code>.json, games/
docs/                   the documentation (list in the README)
```

- `v-bear-core` → only `emu-api` (+ the libraries the emulation needs).
- `emu-frontend` → only `emu-api`: it knows no console.
- `crates/v-bear` ties the two together; the libretro core uses the core and `emu-api`,
  never the frontend.

Dependencies of the core (`crates/v-bear-core/Cargo.toml`): `m68k` 0.14 (MIT, the 68000, with its `serde`
feature) and `mos6502` 0.10 (BSD-3-Clause, the 65C02), both from crates.io, with nothing vendored or forked;
`serde`, `serde_json` and `bincode` for the save states; `zip` to read a ROM or the BIOS from a `.zip`.
The crates' licences are listed in the README.

## Contract (`emu-api`), following libretro conventions

| Aspect | Decision |
|---|---|
| Controls | **RetroPad** bitmask per player (B, Y, Select, Start, ↑ ↓ ← →, A, X, L, R, L2, R2, L3, R3); the system declares which ones it uses and the name of each |
| Video | `&[u32]` **XRGB8888** + width/height/aspect ratio per frame |
| Audio | **interleaved stereo f32**, rate chosen by the frontend |
| Save state | opaque bytes (`retro_serialize`); the file header (magic, ROM hash, thumbnail) belongs to the frontend |
| Memory | `system_ram()` (`RETRO_MEMORY_SYSTEM_RAM`) |
| Metadata | `SystemInfo`: name, extensions, libretro-thumbnails name, cover shape, buttons, FPS, resolution, identification by hash |
| Frame rate | `Core::fps()` (default: the one in `SystemInfo`) |
| Options | `Core::options()` / `set_option()`, like libretro *core options* |
| Debugging | `debug_views()`: images (with optional grid) and tables in groups |

What the Super A'Can does with each row:

| Aspect | In V-Bear |
|---|---|
| Controls | 2 players. The pad is SNES-style: Up, Down, Left, Right, A, B, X, Y, L, R, Select and Start; each RetroPad bit goes to the pad bit of the same name (`PAD_MAP` in `core.rs`); L2, R2, L3 and R3 are not used |
| Video | always 320 × 240; the 256-dot modes are stretched to the same width, as a TV shows them, and the 224-line mode blanks 8 lines at the top and bottom. Pixels are the palette's xBGR555 expanded to XRGB8888; 4:3 aspect |
| Audio | the sampler's 44,744.3 Hz stereo stream, multiplied by 4 (it leaves a lot of headroom) and resampled to the frontend's rate |
| State | a file header (`VBEARST`, format version, the ROM's CRC32) followed by the whole machine state; ~267 KB. The ROM, the BIOS and the battery SRAM are left out |
| RAM | the 64 KB of work RAM, seen from `$FC0000`, for the cheats and the debugger |
| Battery | `battery_ram()`: the cartridge's 32 KB SRAM (`.sav`) |
| Options | none |
| FPS | MCLK / (3420 × 262) = 53,693,175 / 896,040 ≈ 59.92 Hz (`FPS_NUM`/`FPS_DEN` in `core.rs`): the 53.693175 MHz crystal, 3,420 master clocks per line and 262 lines |

## The core crate

| File | Contents |
|---|---|
| `lib.rs` | `pub mod`s and `pub use core::{INFO, …Core}` |
| `core.rs` | the **adapter**: `static INFO: SystemInfo` and `impl emu_api::Core` (RetroPad → native input, picture → XRGB8888, audio → stereo f32, state ↔ bytes) |
| `console.rs` | the machine: wires the two CPUs, the bus, video, sound and pads, and runs them line by line (`run_frame`); also `snapshot`/`restore` and the save-state and SRAM files |
| `bus.rs` | the 68000 bus: address decode, the UM6619's host port, interrupts, the free-running counter and the two host DMA channels |
| `bios.rs` | the four BIOS files, identified by CRC32 |
| `cart.rs` | the cartridge: byte-order detection, `.zip` with several mask ROMs, the SRAM |
| `video.rs`, `render.rs` | the UM6618: registers, palette, VRAM and the renderer (tilemaps, ROZ, windows, sprites, mixer) |
| `sound.rs` | the UM6619's sound side: the 65C02, its RAM and I/O page, the pad ports, the mailboxes with the 68000 and the 16-channel sampler |
| `audio.rs` | resamples the sampler's output to the device's rate |
| `input.rs` | the pad's 16-bit shift register |
| `state.rs` | the save-state file format |
| `debug.rs` | debug switches read once from `V_BEAR_DEBUG` (`snd`, `t65`, `dma`, `irq`...), for the windowless test mode |
| `games.rs` | game database (identification by CRC32) and the games' texts |

## The frontend

`emu-frontend` is shared as is. Each file in `app/` is an `impl App` block (emulation loop,
input, controller navigation, menus, screen, library, save states, recording, controls,
dialogs). It reads everything about the system from `SystemInfo`; the only system-specific
bits are what the app passes in: `brand::set(Brand { name, id, version, icon_png, about,
repo_url, bezels, free_games, thumbnails_url })` and the app's own language texts
(`i18n::add_catalog`).

## Tests

`cargo test --workspace` runs everything that needs no game:

- `games.rs`: every game has its texts in all languages, and a game is found in either byte order;
- `sound.rs`: the 65C02's save state keeps the wait state and the NMI edge, the pad is read most
  significant bit first, the mailbox and the latches, and a looping sampler voice;
- `state.rs`: the save-state file round-trips and rejects another game, another format version and junk;
- `crates/v-bear/src/main.rs`: the games' texts are valid (checked by the frontend);
- `crates/v-bear-libretro`: the libretro C API matches the core and the states round-trip.

`crates/v-bear-core/tests/contract.rs` runs a real game through the `emu-api` contract (900 frames:
a picture most of the time, the right amount of sound, and a save state that makes the next 120 frames
identical). It needs files that are not in the repository, so it is skipped unless `V_BEAR_TEST_ROM` (a
game `.zip`) and `V_BEAR_TEST_BIOS` (a folder with the four BIOS files) are set.

The comparison with other implementations is not part of `cargo test`: see
[Accuracy and validation](accuracy.md) and [Compatibility](compatibility.md).

## How the emulation works

**Time base.** The master clock is the 53,693,175 Hz crystal (`MCLK`, `console.rs`). A line is 3,420
master clocks and a frame 262 lines, which is 59.92 frames a second. The 68000 runs at MCLK / 5 (684
cycles a line) and the 65C02 at that clock divided by 3.

**One frame** (`Console::run_frame`). For each of the 262 lines:

1. `start_line` raises the line's interrupts: vertical blank on line 240 (and the sound CPU's NMI, if
   enabled), the line interrupt on lines 1 to 239 and the raster "line on" / "line off" compare;
2. the UM6618 builds the next line into a line buffer while this one is shown (in the MiSTer core,
   `render_y = vpos + 1`), so `render_line(next)` runs now: line L sees the registers and VRAM as they are
   when line L−1 begins;
3. the line is run in 4 slices, alternating the 68000 (`run_cpu`, 171 cycles) and the 65C02 and sampler (57 of their cycles):
   after every batch of instructions the horizontal position is derived from the cycles used.

Instructions are never split: the cycles run past the end of a slice (`overshoot`) are taken from the next
one. A DMA holds the 68000 off the bus (`stall`) while the counter keeps running.

**CPU.** The 68000 is the `m68k` crate, run in cycle batches with a hook that refreshes the interrupt
level after each instruction; illegal, A-line and F-line instructions count in `faults`, which is how a crash
shows up in the tests. The 65C02 is the `mos6502` crate (`Mos65C02`), inside `sound.rs`, with the UM6619's
I/O page decoded around it.

**Video** (`video.rs`, `render.rs`). Each of the 240 visible lines is built from the layers in the
mixer's order: four tilemaps, the ROZ layer, window 0 and window 1, then the sprites (up to 128 entries in the
table, with scaling, mosaic and masks). The mixer works in palette-index space; the palette is 256 entries of
xBGR555. The width is 256 or 320 dots (`$F00008`, bit 8), and bit 9 blanks 8 lines at the top and the
bottom. The details of the chip are in [Hardware](hardware.md).

**Sound.** The 16-channel sampler produces one stereo sample every 80 cycles of the sound CPU
(44,744.3 Hz). `audio.rs` multiplies it by 4 and resamples it to the rate the frontend asks for.

**Save states.** `Console::snapshot` copies the machine between frames: the 68000 goes through its `serde`
form (JSON), the rest (`Bus`, which holds the sound CPU too) through `bincode`. The ROM, the BIOS and the
battery SRAM are not saved, so a state loaded later does not undo a save in the game. A state is exact:
loading it and running 120 frames gives the same picture as running them in the first place (checked by
`tests/contract.rs`).

## The reasoning behind each decision

| Decision | Why, and what it costs |
|---|---|
| When MAME and the MiSTer core disagree, follow the MiSTer core | it models the whole board and runs the whole library; the disagreements are recorded in [Hardware](hardware.md). Cost: the MiSTer core is itself a reconstruction, with one commit and no test on a real console ([Accuracy](accuracy.md)) |
| The 68000 and the 65C02 are crates, not written here | existing, MIT and BSD-3-Clause crates. Cost: V-Bear depends on how they count cycles; the 68000 is not `Clone`, so the state goes through its `serde` form |
| The machine runs line by line, in 4 slices per line | enough for the effects the games use (raster interrupts, per-line scroll and window tables) and cheap. Cost: it is not cycle-accurate inside a line, and the horizontal position is derived, not simulated |
| The renderer builds a whole line at once, one line ahead | it is what the MiSTer core does (`render_y = vpos + 1`). Cost: line L sees the registers and the VRAM as they were when line L−1 began |
| Fixed 320 × 240 output | the frontend works with a fixed size per system; the 256-dot lines are stretched, as a TV shows them, and the 224-line mode is shown with black bars. Cost: the real size of the picture is not visible; the comparison with the MiSTer core converts it back |
| 4:3 aspect | a television; the console's pixel aspect ratio was not measured |
| The BIOS is required and identified by CRC32 | the 68000's boot ROM, the sound CPU's two ROMs and the lockout key are needed to boot; the file name does not matter. Cost: the user has to find the four files, which are not distributed |
| Games are identified by CRC32 against a database of 12 games | the frontend shows box art and the game's sheet by content. Cost: a ROM that is not in the database (a homebrew, a hack) has no sheet |
| Save states use a header with the ROM's CRC32 | a state from another game is refused. Cost: the format has a version; changing the saved structures makes old states unreadable |
| Everything the interface needs is embedded | icon, fonts, translations and the game database are in the executable; there is nothing to install. Cost: a bigger binary |
