# Changelog

Changes to the shared base (`emu-api`, `emu-frontend`, tools). An emulator that picks up a
newer interface (see "Evolução da interface" in the README) reads this file to know what the
contract changes mean for its own code. This repo has no version (it is internal and private); sections are dated. The version belongs to each emulator.

## 2026-10-21: library list view, play time and history, game genre (from V-Bear)

No change to `emu-api`. The game texts gain an optional field, so an emulator only has to
add genres to its `games.<code>.json` if it wants them.

**Library**

- A fourth view, **List**, after Box art, Title screen and Gameplay: a table of the games
  with title, genre, year, developer, publisher, players, play time and last played. Click a
  header to sort by it (again to reverse; times sort by value, empty cells last); click a row
  to play; right-click for the game menu. Arrow keys and controller move through the rows,
  and the view is remembered (`prefs.library_art = "list"`). Each column is as wide as its
  widest text; room left over is shared, and when there is not enough only the long columns
  shrink (`fit_columns`, tested).
- Game menu (right-click, cards and list, one function `game_menu`): Open, Game info…,
  History…, with the menu bar's spacing.

**Play time and history** (as in ORBE; new module `usage.rs`)

- Every session (over 10 s of the game actually running: pause, menus and an unfocused window
  don't count) is recorded in `<app folder>/usage.json`: total time and last played per game,
  and the latest 200 sessions with start and length.
- Shown in the list (Play time, Last played: "2 days ago"), on the info sheet ("Played:
  40min · 21/10/2026 17:05 (1 h ago)"), in the game's History… and in File › Play history…
  (every game's sessions).

**Game sheet**

- Game texts (`games.<code>.json`, read by `sheets.rs`) take an optional `genre`, translated
  like the description; the sheet shows Genre, Developer before Publisher, and Played.
- "1 to N players" for more than two players (it said "1 or N").
- The sheet's window grows with the cover (20% larger since the previous change), and the
  cover never gets taller than the room there is: no scrollbar because of it.
- `docs/kit/sheets.model.md` documents `genre`.
- New catalog texts in pt.json and es.json.

## 2026-10-21: system files (BIOS), battery saves, multi-file ROMs; interface fixes (from V-Bear)

Made while building V-Bear (Funtech Super A'Can), whose core needs a BIOS and a battery save.
All additions are generic: no system is named in the interface.

**Contract changes**, which anyone updating an emulator has to apply:

- `SystemInfo` has two new fields. Add them to the system's `INFO`:
  - `bios: &'static [BiosFile]`: the system files the core can use; `&[]` if none.
  - `multi_file_roms: bool`: a .zip holding several ROM files is one game spread over several
    chips (the frontend joins them, largest first); `false` keeps the old behaviour (first file).
- New `BiosFile { name, crc32, size, required }`: usual file name (shown when missing),
  CRC32, size in bytes (0 = not known) and whether no game starts without it.
- New `Core` methods, all with an empty default (existing cores compile unchanged):
  - `set_bios(&[Option<Vec<u8>>])`: the system files found, in `SystemInfo::bios` order,
    called before every `load`.
  - `battery_ram() -> Option<&[u8]>` and `battery_ram_mut()`: the cartridge's battery-backed
    save memory, if any.
  - `load_battery(&[u8])`: puts a `.sav` back, called right after `load`.
- `rom::read(path)` is now `rom::read(path, multi: bool)`.

**Interface (`emu-frontend`)**

- System files: found by CRC32, whatever their name, loose or inside a .zip (the zip's CRC is
  read from its directory, nothing is unpacked until it matches), in the new **BIOS** folder
  and in the game's folder (files up to 1 MiB). Missing required files stop the game with a
  dialog that names them and offers "Choose BIOS folder…" (it saves the folder and opens the
  game). For systems with `bios` only:
  - a **BIOS** menu in the menu bar: choose / open the folder, check again, and the list of
    files with size and CRC32 in a monospace column and a check mark on the ones found; a
    click copies one, "Copy list" copies all as a text table padded with spaces;
  - a "BIOS folder…" button in the library header and a "BIOS MISSING" card above the games
    while a required file is missing (it disappears once the BIOS is complete).
- Battery saves: `<game>.sav` in the new **Battery saves** folder, read after `load`, written
  when it changes (checked every 5 s), when the game closes and when the app quits
  (`on_exit`). Nothing is written for games that never touch it.
- `config::Folder` has `Saves` and `Bios` (`Folder::ALL` has 8 entries); both show in
  Emulation › Folders, BIOS only for systems with one.
- Covers: one rule everywhere (`App::game_art`): the library cards, the game info sheet and
  the "Game cover" frame. The user's own image is found by the name shown, the libretro
  (No-Intro) name or the ROM file's name; then the downloaded one. Before, the info sheet
  and the frame only looked for the libretro name and missed the user's own covers.
- `menu::item_mono`: a menu row whose right column is a monospace detail (sizes, hashes),
  aligned from row to row.
- Library: covers, title and in-game screens 20% larger (`LIB_SCALE`); the initial window grows
  with them (`app::library_window_size`), still 7 × 2 covers.
- Library grid: the width left over after the last column is shared among the gaps, so the grid
  spans the whole window instead of leaving an empty strip on the right (seen at 1366 × 768).
- Menu bar: a little room before and after each title.
- "Open last game on startup" is off by default.
- The clipboard works: eframe is built with its `clipboard` feature (copying did nothing).
- New catalog texts in pt.json and es.json for all of the above.

**Tools and examples**

- `tools/templates/libretro.rs.in` (and so `new-emulator.sh --libretro`): the system files
  are looked up by CRC32 among the loose files of RetroArch's *system* folder
  (`RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY`), and the battery save is exposed as
  `RETRO_MEMORY_SAVE_RAM`. RetroArch hands over a single file, so multi-file ROMs do not
  work there.
- `tools/new-emulator.sh`, `toy-core`, `libretro-host` and the test systems in the frontend
  fill in the two new `SystemInfo` fields. `libretro-host` does not pass BIOS files or the
  save RAM on to the cores it runs yet.
- Docs: CLAUDE.md, docs/guia.md and SISTEMA.md no longer list BIOS and battery saves as
  missing from the contract; SISTEMA.md asks for them where they turn into code.

## 2026-10-21: refresh game list, keys of newer actions, Windows child processes (71c9443)

Brought from Mil Capivaras. No contract change.

- F5 / File › Refresh game list (`Func::RefreshGames`): reads the ROM folder again.
- An action missing from a saved configuration (added in a newer version) gets its default
  key; one the user cleared stays empty.
- `platform::command`: child processes (curl, ffmpeg, the folder opener) open without a
  console window on Windows.
- Neutral comments and font names (no more "ORBE"); F5 in the README model.

## 2026-10-20: first state of the base (interface considered mature)

Copied from Mil Capivaras 0.9.0 (`emu-api`, `emu-frontend`, fonts, language catalogs) and
changed as below. **Contract changes against the Mil Capivaras copy**, which anyone updating
an emulator has to apply:

- `SystemInfo` has a new field, `cover_aspect: f32` (box art width / height; about 0.66 for
  portrait boxes). Add it to the system's `INFO`.
- `app::library_window_size()` is now `app::library_window_size(sys: &SystemInfo)`.
- `i18n::add_catalog(code, json)` is new: the app's own texts go there. The catalogs in
  `assets/i18n/` now hold only the generic interface texts (55 PV-1000, command-line and
  memory-map texts were removed from pt.json and es.json).
- Temporary folder names in tests and comments no longer say `mil_capivaras`.
- Fixed `theme::tests::japanese_has_glyphs` (texture deltas were dropped unapplied).

**New in the base**
- Documentation kit: `docs/kit/*.model.md` (README of 15 sections, architecture, accuracy, libretro, sheets, translating, CHANGELOG), built from Mil Capivaras' docs, and `tools/new-docs.sh` to fill it in for an emulator. Guide: `docs/kit-docs.md`.
- `.gitignore` blocks ROMs, BIOS and third-party cores (`roms/`, `bios/`, `cores/`, archives, `*.bin`, `*.rom`, `*.so`...); licence notes in README, CLAUDE.md and SISTEMA.md.
- `tools/init-project.sh` (start the clone as one emulator's project; `--finalizar` removes the example) and `tools/update-interface.sh` (bring the interface into an existing emulator, recording the base commit in `.interface-version`). `SISTEMA.md` is the system sheet to fill in per clone.
- `libretro-host`: runs any libretro core as an `emu_api::Core`; `libretro-player` plays one
  (`--core CORE.so [GAME]`).
- `tools/new-emulator.sh [--libretro] [--ffi-c]`: generates `<slug>-core` and the app, and
  optionally the libretro core and a C/C++ FFI skeleton.
- `toy-core`, `toy`, `toy-libretro`: a working example system.
- CI (`.github/workflows/ci.yml`), `rust-toolchain.toml`, `Cargo.lock`.
- Docs: `README.md` (overview, workflow), `docs/guia.md` (architecture), `CLAUDE.md` (AI instructions).
