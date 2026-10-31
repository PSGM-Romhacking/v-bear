# References

**English** · [Português](references.pt-BR.md) · [Español](references.es.md)

Register of the sources consulted for V-Bear. **Append only** — old sources are never
deleted or replaced; a new version of a source becomes a new entry, with its date.
`hardware.md` cites these entries as [n].

Type legend: code · doc · wiki · forum · news.

## 2026-10-20 — initial research

Commits consulted:

| Repository | Commit |
|---|---|
| mamedev/mame | `60dbfc6f256dc56c5985fae05c29682c3f24e7b6` |
| MiSTer-devel/SuperACan_MiSTer | `59ba2750592f825aee73003142209c9f0f4d1cf6` |
| angelosa/hw_docs | `a1bc727331c74d561552fe6846c8b6a04f7aa974` |
| kitrinx/ACan | `c67a883a84ad47090751f1404e9a1fdc684ae7e9` |

Prefixes: **M** = `https://github.com/mamedev/mame/blob/60dbfc6f256dc56c5985fae05c29682c3f24e7b6/` ·
**S** = `https://github.com/MiSTer-devel/SuperACan_MiSTer/blob/59ba2750592f825aee73003142209c9f0f4d1cf6/`

| # | Title | URL | Type | Wayback |
|---|---|---|---|---|
| 1 | MAME supracan.cpp | M`src/mame/umc/supracan.cpp` | code | – |
| 2 | MAME umc6619_sound.cpp/.h | M`src/mame/umc/umc6619_sound.cpp` | code | – |
| 3 | MAME umc6650.cpp | M`src/mame/umc/umc6650.cpp` | code | – |
| 4 | MAME bus supracan rom/slot | M`src/devices/bus/supracan/rom.cpp` (+ `slot.cpp`) | code | – |
| 5 | MAME softlist | M`hash/supracan.xml` | code/data | – |
| 6 | MiSTer README | S`Readme.md` | doc | – |
| 7 | mainboard.sv | S`rtl/board/mainboard.sv` | code | – |
| 8 | um6619.sv | S`rtl/um6619/um6619.sv` | code | – |
| 9 | um6619_irq.sv | S`rtl/um6619/um6619_irq.sv` | code | – |
| 10 | um6619_frc.sv | S`rtl/um6619/um6619_frc.sv` | code | – |
| 11 | um6619_overlay.sv | S`rtl/um6619/um6619_overlay.sv` | code | – |
| 12 | um6619_cpu.sv | S`rtl/um6619/um6619_cpu.sv` | code | – |
| 13 | um6619_snd.sv | S`rtl/um6619/um6619_snd.sv` | code | – |
| 14 | um6619_dma.sv | S`rtl/um6619/um6619_dma.sv` | code | – |
| 15 | um6618.sv | S`rtl/um6618/um6618.sv` | code | – |
| 16 | um6618_regs.sv | S`rtl/um6618/um6618_regs.sv` | code | – |
| 17 | um6618_vtiming.sv | S`rtl/um6618/um6618_vtiming.sv` | code | – |
| 18 | um6618_tilemap.sv | S`rtl/um6618/um6618_tilemap.sv` | code | – |
| 19 | um6618_roz.sv | S`rtl/um6618/um6618_roz.sv` | code | – |
| 20 | um6618_sprite.sv | S`rtl/um6618/um6618_sprite.sv` | code | – |
| 21 | um6618_mixer.sv | S`rtl/um6618/um6618_mixer.sv` | code | – |
| 22 | um6618_window.sv | S`rtl/um6618/um6618_window.sv` | code | – |
| 23 | um6618_vram_arb.sv | S`rtl/um6618/um6618_vram_arb.sv` | code | – |
| 24 | Cartridge/lockout | S`rtl/cart/um6650.sv`, `cart_16003_1b.sv`, `cart_slot.sv`, S`rtl/board/cart_loader.sv` | code | – |
| 25 | bios_loader.sv | S`rtl/board/bios_loader.sv` | code | – |
| 26 | joypad/multitap | S`rtl/board/joypad.sv`, S`rtl/board/multitap.sv` | code | – |
| 27 | um70c188.sv | S`rtl/board/um70c188.sv` | code | – |
| 28 | MiSTer licenses | S`LICENSE`, S`rtl/fx68k/LICENSE`, S`rtl/6502/mos6502.sv` | code | – |
| 29 | angelosa hw_docs — pergame.md | https://github.com/angelosa/hw_docs/blob/a1bc727331c74d561552fe6846c8b6a04f7aa974/funtech_superacan/pergame.md | doc | not found |
| 30 | evadot — pinout/chip list (gist) | https://gist.github.com/evadot/66cfdb8891544b41b4c9 (revision not pinned) | doc | https://web.archive.org/web/20240908064156/https://gist.github.com/evadot/66cfdb8891544b41b4c9 |
| 31 | Wikipedia — Super A'Can | https://en.wikipedia.org/wiki/Super_A%27Can | wiki | https://web.archive.org/web/20260427233007/https://en.wikipedia.org/wiki/Super_A%27can |
| 32 | Batocera wiki — supracan | https://wiki.batocera.org/systems:supracan | wiki | https://web.archive.org/web/20260607050332/https://wiki.batocera.org/systems:supracan |
| 33 | kitrinx/ACan (translations and cheats) | https://github.com/kitrinx/ACan/tree/c67a883a84ad47090751f1404e9a1fdc684ae7e9 | code | – |
| 34 | NESdev — ZapFC thread, post by etabeta (2011) | https://forums.nesdev.org/viewtopic.php?p=74760 (t=7494) | forum | https://web.archive.org/web/20230826065022/https://forums.nesdev.org/viewtopic.php?t=7494 |
| 35 | NESdev — "Top 10 Failed Gaming Consoles" (2010) | https://forums.nesdev.org/viewtopic.php?p=69563 (t=7128) | forum | https://web.archive.org/web/20230826083326/https://forums.nesdev.org/viewtopic.php?t=7128 |
| 36 | NESdev wiki — searches with no results | https://www.nesdev.org/w/index.php?search=%22Super+A%27Can%22&fulltext=1 | wiki | – |
| 37 | Multiplayer.it — MAME 0.269 | https://multiplayer.it/notizie/la-versione-0269-1-del-mame-consente-di-emulare-una-console-rarissima.html | news | https://web.archive.org/web/20250712072456/https://multiplayer.it/notizie/la-versione-0269-1-del-mame-consente-di-emulare-una-console-rarissima.html |
| 38 | LaunchBox forum (2023) | https://forums.launchbox-app.com/topic/74179-funtech-super-acan/ | forum | https://web.archive.org/web/20250911083710/https://forums.launchbox-app.com/topic/74179-funtech-super-acan/ |
| 39 | Giant Bomb — Super A'Can | https://www.giantbomb.com/super-acan/3045-151/ | wiki | https://web.archive.org/web/20250605112025/https://www.giantbomb.com/super-acan/3045-151/ |
| 40 | Time Extension — Piko buys IPs (2026-05) | https://www.timeextension.com/news/2026/05/piko-interactive-acquires-two-more-super-acan-ips-teases-something-new-is-on-the-cards | news | not verified (search snippet only; page returned 403) |

Not consulted (searches with no technical page): Sega-16, AssemblerGames, Diskman videos.
Inaccessible: gametechwiki (403).

Project starting point: `Referencias.txt` (MiSTer core + MAME driver).

## 2026-10-20 — ROMs provided by the user

Folder `roms - Funtech - Super Acan/` (outside git): 12 zips, No-Intro names. The CRC32s of the 13 files
match the MAME softlist [5]. 11 dumps have bytes swapped per word (`load16_word_swap`
in the softlist); only **Rebel Star (En)** is big-endian. **Super Dragon Force** comes in two parts:
`16007` (2 MiB, at `$000000`) + `08002` (1 MiB, at `$200000`; `08007.1` in MAME).
No BIOS file came with them.

## 2026-10-20 — BIOS and corrections during implementation

- **BIOS** from the user's local collection (`~/Projetos/Bios/retrobios/bios/Funtech/Super A'Can/`):
  `supracan.zip` (`internal_68k.bin` `8d575662`, `umc6650.bin` `0ba78597`) and `.variants/supracan.zip`
  (`internal_6502_1.bin` `fc9fb05f`, `internal_6502_2.bin` `bf950ab7`, `internal_68k.bin`). CRCs identical
  to those of MAME [3] and MiSTer [6]. Local copy in `bios/` (outside git).
- **Correction — sound RAM is 64 KiB:** `rtl/board/mainboard.sv` (U11, `sram_async #(.ADDR_WIDTH(16))`) [7].
  The initial report said 32 KiB.
- **MiSTer files read during implementation** (same commit `59ba275`, already listed above):
  `um6619.sv` [8], `um6619_irq.sv` [9], `um6619_frc.sv` [10], `um6619_overlay.sv` [11], `um6619_cpu.sv`
  [12], `um6619_snd.sv` [13], `um6619_dma.sv` [14], `um6618.sv` [15], `um6618_tilemap.sv` [18],
  `um6618_roz.sv` [19], `um6618_sprite.sv` [20], `um6618_mixer.sv` [21], `um6618_window.sv` [22],
  `cart/um6650.sv` [24], `board/joypad.sv` [26].
- **Rust libraries used:** `m68k` 0.14.6 (MIT, https://github.com/benletchford/m68k-rs) for the 68000;
  `mos6502` 0.10.1 (BSD-3-Clause, https://crates.io/crates/mos6502/0.10.1) for the 65C02.

## 2026-10-21 — local copies

- The 2026-10-20 copies (in /tmp) were lost; downloaded again at the same commits into
  `_referencias/` (outside git): MAME `60dbfc6` (7 files) and MiSTer `59ba275` (full clone).
- Used in this step: MAME `supracan.cpp` [1] (window 0, raster IRQ, `$E90010` "smearing mirror").

## 2026-10-21 — MAME as an execution reference

- **MAME 0.289** (Flatpak `org.mamedev.MAME`, Flathub, Flatpak commit `41ebdb5c`), driver `supracan`, run
  headless with a Lua script (`-autoboot_script`) that presses the same buttons and captures the same
  frames as V-Bear's headless mode. Audio via `-wavwrite`. Tools in `tools/mame/` (outside git).
  MAME requires all four BIOS files; the main `supracan.zip` has only two.
- Finding: in this version, with no input at all, MAME advances through titles and menus on its own, even though the
  `P1`/`P2` ports stay at rest (`FFFF`, active-low, same polarity as MiSTer's `joypad.sv` [26] and
  V-Bear). Cause not investigated. For this reason MAME is only useful for comparing identical scenes, not the game sequence.

## 2026-10-21 — savestate libraries

- `serde` 1.0.229 (MIT/Apache-2.0, https://crates.io/crates/serde/1.0.229) and `bincode` 1.3.3
  (MIT, https://crates.io/crates/bincode/1.3.3).
- `mos6502` 0.10.1 read in `src/cpu.rs`: `wait_state` and `last_nmi_state` are private (there is only
  `wait_state()` for reading), so V-Bear recomposes them when loading a state.

## 2026-10-21 — PNG screenshots

- `flate2` 1.1.10 (MIT/Apache-2.0, https://crates.io/crates/flate2/1.1.10), already present through `zip`.
- PNG format: W3C, *Portable Network Graphics (PNG) Specification (Third Edition)*,
  https://www.w3.org/TR/png-3/ (IHDR/IDAT/IEND, CRC-32, filter 0 per row).

## 2026-10-21 — interface base and license

- **estrutura-novos-emuladores-rust** (private, PSGM-Romhacking), `git@github-psgm:PSGM-Romhacking/_estrutura-novos-emuladores-rust.git`,
  commit `5b61218` (2026-10-20), plus uncommitted local changes in the checkout
  `~/Projetos/_estrutura-novos-emuladores-rust` (2026-10-21 12:29). Instructions read: `README.md`,
  `CLAUDE.md`, `SISTEMA.md`, `docs/guia.md`, `crates/api/src/lib.rs`, `tools/init-project.sh`,
  `tools/new-emulator.sh`. License GPL-3.0-or-later.
- **V-Bear license** becomes GPL-3.0-or-later (owner's decision). Caveat recorded: in the MiSTer core
  [6], `SuperAcan.sv` says "version 2 of the License, or (at your option) any later version";
  the chip files (`rtl/um6618/*`, `rtl/um6619/*`) only have "Copyright (c) 2026 Jamie Blanks" and the
  repository's `LICENSE` is the GPLv2 text. V-Bear reimplements the behavior in Rust, without
  copying the HDL.
- Libretro: `libretro.h` (libretro-common), constants `RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY` (9) and
  `RETRO_MEMORY_SAVE_RAM` (0); `.info` format with `firmwareN_*` as in the RetroArch cores.

## 2026-10-21 — No-Intro DAT and covers

- **No-Intro, Funtech - Super Acan** (copy in libretro-database), https://raw.githubusercontent.com/libretro/libretro-database/master/metadat/no-intro/Funtech%20-%20Super%20Acan.dat,
  commit `9a899f5` (2026-10-08). Official name `Rebel Star (Taiwan) (En)`, CRC32 `9e243c91` (2 MiB). The
  user's file with that name has CRC32 `a3469172`: it is the same dump with the 16-bit words
  swapped (the other byte order gives exactly `9e243c91`). The other 11 match the DAT.
- **libretro-thumbnails, Funtech_-_Super_Acan**, https://github.com/libretro-thumbnails/Funtech_-_Super_Acan,
  commit `42814d9` (2025-01-13): Named_Boxarts, Named_Titles and Named_Snaps with 11 games each; there is
  no image of Rebel Star.
- On 2026-10-21, at the user's request, the local copy was renamed to `Rebel Star (Taiwan).zip` (inner
  file `Rebel Star (Taiwan).bin`, same content, CRC32 `a3469172`): the game is in Traditional Chinese;
  only the logo and the menu have English words. The game database keeps using the No-Intro name for
  the covers.

## 2026-10-21 — game descriptions

- The descriptions in `assets/games/games.{en,pt,es}.json` were written for V-Bear, from what
  appears in the games themselves (test captures, menus, title screens) and on the boxes (for example
  "S.RPG" on Super Dragon Force). The MAME softlist [5] only has notes about emulation failures. The
  mention of Rebel Star's sample cartridges comes from 百度百科 (A'can), https://baike.baidu.com/item/A'can
  (accessed 2026-10-21). Format copied from Mil Capivaras (`docs/sheets.md`, commit `fecf86c`).

## 2026-10-21 — research on the games (genre, players, descriptions)

Facts used in the descriptions, genres and number of players in `assets/games/` and `games.rs`, rewritten
in our own words (no text copied). Accessed 2026-10-21.

- **Wikipedia (en), Super A'Can**, https://en.wikipedia.org/wiki/Super_A%27Can — code, developer and
  genre of each game; alternative names (C.U.G., Sonic Dragon, African Adventures).
- **Wikipedia (zh), Super A'Can**, https://zh.wikipedia.org/wiki/Super_A%27Can — Formosa Duel parodies
  Taiwanese public figures, with voices in Taiwanese (Hokkien); Rebel Star only on sample cartridges.
- **Wikipedia (en), Sango Fighter**, https://en.wikipedia.org/wiki/Sango_Fighter — original from 1993 for
  MS-DOS (Panda Entertainment); A'Can conversion done in a rush by a single employee.
- **LaunchBox Games DB**, https://gamesdb.launchbox-app.com/ (community database, without citations; treated as
  "according to LaunchBox"): number of players of the 12; Bomberman-like (Boom Zoo); Puyo Puyo style and twelve
  opponents (Formosa Duel); 16-tile Taiwanese mahjong (Gambling Lord); digitized photos (Magical
  Pool); farms, quarries, hotels and treasure, up to 6 players (Monopoly); six stages, cue, balls,
  3D bonus stage (Speedy Dragon); Shining Force style and top-down towns (Super Dragon Force);
  licensed CPBL teams, animations and digitized crowd (baseball); choices that make the hero good
  or evil (The Son of Evil); arcade and versus modes (Sango Fighter).
- **PPE.pl, Historia konsol Funtech Super A'Can**, https://www.ppe.pl/publicystyka/202436/Historia-konsol-Funtech-Super-A-Can.html
  — Speedy Dragon as the "Taiwanese Sonic"; Journey to the Laugh as the most popular game.
- **RF Generation, Funtech Super A'Can**, https://www.rfgeneration.com/features/system-overview/237/ —
  Journey to the Laugh among the best games on the console.
- From the games themselves (V-Bear captures): the "21st squadron" and the "Titan empire" of Rebel Star (dialogue and
  introduction); pool and baseball modes (menus); baseball season saved in SRAM.
- Conflicts noted: Rebel Star's genre is strategy on LaunchBox and RPG on the Wikipedias (used:
  strategy, which matches what is seen in the game); Formosa Duel "Chinese celebrities" (LaunchBox) ×
  "Taiwanese politicians" (WP-zh), summarized as "Taiwanese public figures". Blocked: MobyGames,
  Bahamut, avid.wiki, Vogons (403/Cloudflare).
- Developer × publisher: the boxes (libretro-thumbnails covers and the Rebel Star photo) read "…製作 ◎
  敦煌發行" — made by …, published by 敦煌 (Funtech). The `publisher` field of the MAME softlist [5] is, in
  fact, the developer. Developers according to the Wikipedia (en) list; Super Taiwanese Baseball
  League: "全崴製作" on the box = Quan Wei Technology (Wikipedia gives "Quan Wei Technology/C&E").

## 2026-10-21 — battery SRAM

- MAME `hash/supracan.xml` [5] (`dataarea name="nvram" size="0x8000"` in sonevil, staiwbbl, slghtsag,
  monopoly, gamblord, rebelst; gamblord with "TODO: unconfirmed size and actual presence") and
  `bus/supracan/rom.cpp` (`battery_load(..., 0xff)`: MAME starts the SRAM at `0xFF`).
- MiSTer [6] `rtl/cart/cart_16003_1b.sv` (U2 62256, `A[15:1]`, low byte only; "a save file is 32 KiB
  over a 64 KiB window"), `rtl/mem/save_slot.sv` and `SuperAcan.sv` (the `.sav`; the SRAM is not erased when
  switching cartridges).

## 2026-10-23 — homebrew

Searched for free homebrew games, demos or test ROMs for the Super A'Can (for `Brand::free_games`
and for tests): none found. Web searches (English; "homebrew", "SDK", "test rom", "prototype",
"supracan" + GitHub) only bring up the commercial library. The MAME softlist [5] has only the 12
commercial games; the MiSTer repository [6] ships no test ROM.
- Games That Weren't, "Quick Fighting Attack" and "City Escape" (March 2026):
  https://www.gamesthatwerent.com/2026/03/quick-fighting-attack/ ,
  https://www.gamesthatwerent.com/2026/03/city-escape/ — cancelled games, images only (video "Let's
  visit Funtech!" by Brandon Cobb), "No download available"; many Funtech ROMs are said to have been
  destroyed.

## 2026-10-30 — the MiSTer core simulated

- **Verilator** 5.046 (2026-02-28, Fedora package), https://www.veripool.org/verilator/ — simulates the
  SystemVerilog of the MiSTer core cycle by cycle (`reference/`). Needed `-Wno-BLKANDNBLK` for `fx68k`.
- The MiSTer core's README [6], "Development and QA": how the core was validated (videos of real hardware
  showing attract screens by Diskman, disassemblies, no test on a console). The repository has a single
  commit, 2026-09-11 (`59ba275`).
- **MAME 0.289** (Flatpak, same as in 2026-10-21), as a third source: snapshots of Magical Pool around frame 300
  (the growing circle on window 0) with `tools/mame/drive.lua`.
- `sevenz-rust` 0.6 (Apache-2.0, https://crates.io/crates/sevenz-rust/0.6.1), already in the workspace: reads
  the `.7z` files (games with several chips, BIOS inside a `.7z`).
