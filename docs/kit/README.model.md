<!--
  Kit template (see docs/kit-docs.md): the emulator's README, which is both the manual and a
  technical reference of the system, in the structure of Mil Capivaras' README (15 sections).
  Replace the {{PLACEHOLDERS}} with tools/new-docs.sh and resolve every TODO(...).
  [GENERIC] sections are written (the interface is the same for every emulator): adjust
  the details. [SYSTEM] sections are an outline with what Mil Capivaras documented there: write
  them from your own sources, and cite each one ([W1], [M2]… listed in section 14). Where
  the sources disagree, show the discrepancy. Delete this comment.
-->
# {{NAME}}

<!-- TODO(translate): language switcher line: **English** · [Português](README.pt-BR.md) · [Español](README.es.md) -->

**{{NAME}}** is an emulator of the **{{SYSTEM}}** (<!-- TODO(system): year, maker -->) written in **Rust**.
<!-- TODO(system): in two or three lines: what it was reimplemented from, how it was validated, what it runs (counts of games), the headline numbers. -->
It has a complete graphical interface:
- library with box art;
- save states and rewind;
- GIF and MP4 recording;
- debugger and cheats;
- 3 languages.

It also ships as a **libretro core**, for RetroArch.

![Library with box art](screenshots/library-covers.png)

<!-- TODO(system): why the name, and the icon, if there is a story. -->

This document is both the emulator's manual and a technical reference for the console.
Everything here was taken from the code, from measurements or from the sources listed in
[References](#14-references). Where the sources disagree, the discrepancy is shown.

| | |
|---|---|
| Accuracy | <!-- TODO(system): level and how it was measured (section 9) --> |
| Compatibility | <!-- TODO(system): N/N games, how it was checked (section 10) --> |
| Speed | <!-- TODO(system): frames/s on which CPU, and the multiple of real time --> |
| Platforms | Linux (Wayland and X11, AppImage) and Windows; RetroArch on any system that runs the core |
| License | {{LICENSE}} ([`LICENSE`](LICENSE)) |

---

## Contents

1. [The {{SYSTEM}}](#1-the-{{SYSTEM_ANCHOR}})
2. [Hardware in detail](#2-hardware-in-detail)
3. [How the implementations differ](#3-how-the-implementations-differ)
4. [Emulator features](#4-emulator-features)
5. [Installation and usage](#5-installation-and-usage)
6. [Code architecture](#6-code-architecture)
7. [How the emulation works](#7-how-the-emulation-works)
8. [The reasoning behind each decision](#8-the-reasoning-behind-each-decision)
9. [Accuracy and validation](#9-accuracy-and-validation)
10. [Compatibility](#10-compatibility)
11. [libretro core (RetroArch)](#11-libretro-core-retroarch)
12. [Project history](#12-project-history)
13. [Lessons learned](#13-lessons-learned)
14. [References](#14-references)
15. [Licenses](#15-licenses)

---

## 1. The {{SYSTEM}}  [SYSTEM]

<!-- TODO(system):
  ### History: release date and price, competition, end of life, successors, number of games
    and their serials; vacant numbers; the controller (with a photo credit if any).
  ### The library: the games, one table (title, year, publisher, serial) and how complete it is.
  Mil Capivaras: section 1 of its README. -->

## 2. Hardware in detail  [SYSTEM]

<!-- TODO(system): the console's hardware as the emulator needs it, from the sources:
  Specification (CPU, clocks, RAM, video chip, sound chip), the board and the chip, clocks,
  a frame on the console (timing, interrupts), memory map, I/O ports, video, sound, controls,
  interrupts, cartridges and connector, and the minimum to program it.
  Put tables of registers and addresses here: they are what the emulator is checked against. -->

## 3. How the implementations differ  [SYSTEM]

<!-- TODO(system): the other emulators/FPGA cores for this system, what each got right and
  wrong (cite them), test ROMs by others and how this emulator fares on them, and the models
  the emulator offers (if more than one) and when to use each. -->

## 4. Emulator features  [GENERIC]

<!-- Screenshots: generate with the app itself (docs/kit-docs.md, "Screenshots"). -->
| | |
|---|---|
| ![Library: gameplay view](screenshots/library-screens.png) | ![Game info sheet](screenshots/game-info.png) |
| Library: gameplay view | Game info sheet |
| ![Debugger: memory](screenshots/debugger-memory.png) | ![Cheats](screenshots/cheats.png) |
| Debugger: memory | Cheats |

### Library

It is what the window shows when no game is open:
- grid with the **box art** or with the **title screen / gameplay** images;
- search, change folder, and click to open;
- **F5** (or **File › Refresh game list**) reads the ROM folder again, for games added,
  removed or renamed with the emulator open;
- **first run:** when the folder has no games, a welcome screen with the **Choose the ROM
  folder…** button (or the controller's A) and links to **free, legal games**
  (`Brand.free_games`);
- right-click: **Open** and **Game info…**;
- games are recognized **by content** (CRC32), so the file name does not matter;
- box art comes from **libretro-thumbnails**, downloaded in the background and cached,
  looked up by the ROM's No-Intro name, as in RetroArch.

Accepted formats: `.{{EXT}}`, and those same ones inside `.zip` or `.7z`.
<!-- TODO(system): why these extensions; the box shape in the library (SystemInfo::cover_aspect). -->

### Game info

Title, original title, year, publisher, developer, product code, file name and full path,
and a description in the current language, from the project's game database
(`assets/games/`; format in [`docs/sheets.md`](docs/sheets.md)).
<!-- TODO(system): how many games have a sheet and where the database came from. Delete if the system has no game database. -->

### Playing

| Feature | Detail |
|---|---|
| **Save states** | 9 slots per game, with a thumbnail of the saved moment and date/time; the file stores the ROM's fingerprint and rejects a state from another game |
| **Rewind** | one state every 2 frames, up to ~30 s; hold Backspace or L2 <!-- TODO(system): memory it takes, from the size of the core's state --> |
| **Fast forward** | hold (Space / R2) or toggle on and off; 2x, 3x, 4x or 8x |
| **Pause** | P or Guide; pauses by itself when unfocused (optional) and with a menu or dialog open |
| **Players** | the 1st controller is P1, the 2nd is P2…; the keyboard has P1 and P2 (P2 on WASD) <!-- TODO(system): the number of players --> |
| **Remappable controls** | each action has a key and one input per controller, saved by driver name |

### Video

| Feature | Detail |
|---|---|
| Window 1x–6x | N screen pixels per game pixel, exact even on Wayland |
| Integer scaling | integer multiples only (uniform pixels) or filling the window |
| Filters | None, Scanlines, CRT |
| **Frames** | fill the sides in fullscreen: <!-- TODO(system): the bezels the app registers (Brand.bezels), plus "Game cover" (blurred and darkened), which is always there --> |
| Fullscreen | F11; the menu bar disappears and comes back when the mouse touches the top |
| FPS and speed | optional indicator |

### Sound

<!-- TODO(system): the default volume and mute, the audio rate, any smoothing/filters, the channels the debugger can mute. -->

### Recording

| Feature | Detail |
|---|---|
| Screenshot | F12: PNG in `~/Pictures/{{NAME}}/` |
| GIF | F9 |
| MP4 with sound | F10: H.264 + AAC via `ffmpeg`, at the exact frame rate (`Core::fps()`) |

Recordings go to `~/Videos/{{NAME}}/` (the XDG Pictures and Videos folders; the localized
name follows the system). Encoding runs in a separate thread, and closing the program in
the middle of a recording finishes the file before exiting.

### Tools

- **Debugger** (separate window): live RAM as a hex editor, with the bytes that changed in
  the last frame highlighted and an inspector with editing; plus whatever the core adds
  through `Core::debug_views()` (<!-- TODO(system): e.g. tiles, registers, video state, sound channels with a mute for each -->).
  Run state: running / paused, pause, advance 1 frame.
- **Cheats**, in two tabs:
  - **Codes:** the game's codes (editable value, live current value, **Write now**,
    **Remove** and **Hold**, which locks the value every frame) and the **new code** form;
  - **Search the RAM:** start the search; play until the value changes and say what
    happened (Increased, Decreased, Changed, Did not change, Is equal to), repeating until
    few addresses are left; **Use** takes the chosen address to the Codes tab.
  - Values in decimal or hexadecimal with `0x`. Saved per game in RetroArch's `.cht`
    format, which also works in RetroArch with the libretro core.
  - <!-- TODO(system): a tested example (game, address, value), and quirks such as scores stored in BCD. -->

### Controller and couch mode

To play from the couch, on a TV or a handheld, with no keyboard or mouse:

- **Quick menu, the controller's "Esc":** opens and closes anywhere with the
  **Home/Guide** button, or **Select + Start held for half a second** (for controllers
  without Home; optional), or **Esc** in couch mode. It pauses the game and offers:
  Continue, Save state, Load state, Reset, Back to the library, **Couch mode** and Quit.
- **Library by controller:** the d-pad (or the left stick, or the arrow keys) moves a
  selection between the covers; the bottom button opens the game; **Select tapped alone**
  changes the image (cover, title, gameplay).
- **Couch mode** (**Video › Couch mode**, or `--couch`): fullscreen, no menu bar, bigger
  covers and a line of button hints. It stays saved until it is turned off in the quick menu.
- **The button doesn't "leak" into the game:** while Select + Start are held together, the
  game doesn't get them; and after opening a game or closing the menu with the controller,
  game input is ignored until every button is released.

### Interface

- Desktop-style menu bar (File, Emulation, Audio, Video, Recording, Tools, Help).
- **Languages:** English (default), Portuguese and Spanish, switched live; more languages
  need only a JSON file ([`docs/translating.md`](docs/translating.md)).
- Preferences, ROM folder, recent games, keys and controllers are saved.
- <!-- TODO(system): its own icon and the entry in the applications menu (make install), if the Makefile has it. -->

---

## 5. Installation and usage  [GENERIC]

### Download

<!-- TODO(system): only when there is a release pipeline. The binaries are in the [releases]({{REPO}}/releases):
| File | System |
|---|---|
| `{{SLUG}}-{{VERSION}}-linux-x86_64.AppImage` | Linux x86-64 |
| `{{SLUG}}-{{VERSION}}-windows-x86_64.exe` | Windows 10 and 11, 64-bit |
| `{{ID}}_libretro-{{VERSION}}-linux-x86_64.zip` / `-windows-x86_64.zip` | the libretro core and its `.info` |
| `SHA256SUMS` | checksums of every file |
On Linux: `chmod +x` the AppImage and run it. Each version's notes come from [`CHANGELOG.md`](CHANGELOG.md). -->

### Build

```bash
cargo build --release -p {{ID}}
cargo build --release -p {{SLUG}}-libretro    # the libretro core
```

Requirements: Rust (cargo) and the ALSA audio library (`libasound2-dev` and `pkg-config`
on Debian and Ubuntu, `alsa-lib-devel` on Fedora and openSUSE, `alsa-lib` on Arch).
Optional at runtime: `ffmpeg` (MP4 recording), `curl` (downloading box art) and
`xdg-open` (opening folders). Without them, only those features are unavailable.

### What it takes to run

Just the executable: icon, translations, game database, fonts, theme and frames are
embedded. When run, it creates:

| Folder | Contents |
|---|---|
| `~/.config/{{ID}}/config.json` | preferences, ROM folder, recent games, keys and controllers |
| `~/.config/{{ID}}/states/` | save states and thumbnails |
| `~/.config/{{ID}}/cheats/` | cheats (`.cht`) |
| `~/.config/{{ID}}/covers/` | your own box art (optional) |
| `~/.config/{{ID}}/languages/` | your own translations (optional) |
| `~/.config/{{ID}}/cache/covers/` | downloaded box art (can be deleted at will) |
| `~/Pictures/{{NAME}}/`, `~/Videos/{{NAME}}/` | screenshots and recordings |

Everything except screenshots and recordings is in **one folder**, so a backup or a sync
needs only `~/.config/{{ID}}/`. On Windows it is `%APPDATA%\{{ID}}\`, with the same layout.

### Portable mode

Create an empty file named **`portable.txt`** next to the executable, and everything
(preferences, saves, cheats, box art, languages, screenshots and recordings) goes into a
**`{{ID}}_data/`** folder next to it instead of your user folder. Paths inside the program's
folder are saved relative to it, so the folder keeps working on another PC or a USB stick.
If the folder can't be written to, the emulator says so at startup and uses the normal one.

### Folders, your own box art and new languages

**Emulation › Folders…** lists every folder (ROMs, save states, cheats, own box art,
languages, screenshots, recordings), each with **Open**, **Change…** and **Default**.

**Own box art.** Put PNGs in the box art folder with the libretro-thumbnails layout:
`Named_Boxarts/` (cover), `Named_Titles/` (title screen) and `Named_Snaps/` (gameplay). The
file name is the ROM's No-Intro name. Your images win over the downloaded ones.

**New languages.** **Help › Language / Idioma › Export translation template…** saves a JSON
file with every text of the interface; translate the values, name the file after the
language code (`fr.json`), put it in the languages folder and restart. The full guide, with
the rules and **how to send a translation**, is in [`docs/translating.md`](docs/translating.md).

### Command line

```bash
{{ID}}                       # opens the library
{{ID}} game.{{EXT}}          # opens a game (also .zip, .7z)
{{ID}} /path/to/folder/      # opens the library in that folder
{{ID}} game.{{EXT}} --scale 4
```

<!-- TODO(system): the other options the app implements (--couch, --identify, --info, --controls, --memory-map, --bench, --shot, --dump-ram…). The ones that are not in main.rs do not exist. -->

### Default controls

| Action | Keyboard | Controller |
|---|---|---|
| Directions | arrow keys | d-pad and left analog stick |
| <!-- TODO(system): the system's buttons and their keys --> | | |
| Pause / reset | P / F1 | — |
| Quick menu (open and close) | Esc (in couch mode) | Home/Guide; Select + Start held for half a second (option) |
| Save / load state; slot −/+ | F2 / F4; F6 / F7 | — |
| Fast forward / rewind (hold) | Space / Backspace | R2 / L2 |
| Sound; volume +/− | M; = / − | — |
| Fullscreen; screenshot; GIF; MP4 | F11; F12; F9; F10 | — |
| Open / close game / quit | Ctrl+O / Ctrl+W / Ctrl+Q | — |
| Refresh game list (reads the ROM folder again) | F5 | — |

Everything can be remapped in **Emulation › Controls and shortcuts…**.

---

## 6. Code architecture  [GENERIC + SYSTEM]

A Cargo *workspace* with the console core separate from the interface. Details and
decisions in [`docs/architecture.md`](docs/architecture.md).

```
crates/api/              core ↔ interface contract (no dependencies)
crates/{{SLUG}}-core/    the console; depends only on emu-api (and the emulation libraries)
crates/frontend/         the reusable interface; knows no console
crates/{{SLUG}}/         the executable: core + interface + app identity
crates/{{SLUG}}-libretro/  the core behind the libretro C API ({{ID}}_libretro.so)
assets/                  embedded data: icons, bezels, fonts, translations, game database
docs/                    architecture, game database, libretro core, accuracy, translating
```

<!-- TODO(system): the dependency diagram with the real core's libraries; then the three
  sub-sections of Mil Capivaras: `emu-api` (the contract), the core crate (the console) and
  `emu-frontend` (the interface); and Tests (what is tested and what needs ROMs that are not
  in the repository). docs/kit/architecture.model.md has the text for emu-api and emu-frontend. -->

## 7. How the emulation works  [SYSTEM]

<!-- TODO(system): the heart of the emulation, as the code does it: time base, one frame, the
  CPU (which core/crate and what was fixed), video pixel by pixel, sound, save states.
  Link to the code (`file:line`) where it helps. -->

## 8. The reasoning behind each decision  [SYSTEM]

<!-- TODO(system): a table "Decision | Why". Every choice that someone would question:
  the reference chosen, language/libraries, resolution, frame rate, aspect ratio,
  identification database, what is embedded. Say what each choice costs. -->

## 9. Accuracy and validation  [SYSTEM]

<!-- TODO(system): how the comparison is done (the reference, the same input, what is compared
  and when), the methodology, the results, the classification, validation against the real
  console, and what is still missing. Detail in docs/accuracy.md (docs/kit/accuracy.model.md). -->

## 10. Compatibility  [SYSTEM]

<!-- TODO(system): how it was tested; a table per group of games (commercial, homebrew…) with
  the status ("Perfect" = what exactly) and the origin of the homebrews. -->

## 11. libretro core (RetroArch)  [GENERIC]

{{NAME}} ships as `{{ID}}_libretro` (`.so` / `.dll`) plus its `.info`: copy both to
RetroArch's cores folder. RetroArch then provides shaders, run-ahead, netplay, remapping and
RetroAchievements. Everything about it, including the options, is in
[`docs/libretro.md`](docs/libretro.md).

## 12. Project history  [SYSTEM]

| Date | Stage |
|---|---|
| {{DATE}} | <!-- TODO(system): one row per day or stage: what was done and what was decided --> |

## 13. Lessons learned  [SYSTEM]

<!-- TODO(system): what you would tell someone doing the same: approach, accuracy, tools,
  interface, recording, hardware model. Honest, specific, short. -->

## 14. References  [SYSTEM]

<!-- TODO(system): numbered sources used above: the console ([W1]), implementations ([M1]),
  the CPU, games and data, emulation. URL and what each one was used for. -->

## 15. Licenses

| Part | License |
|---|---|
| {{NAME}} | {{LICENSE}} (`LICENSE`) |
| Noto Sans CJK JP subset | SIL Open Font License 1.1 |
| <!-- TODO(system): the core's origin, vendored crates, game database --> | |

The ROMs are not part of the project.
