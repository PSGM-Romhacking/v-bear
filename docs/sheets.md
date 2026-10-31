# Game database

**English** · [Português](sheets.pt-BR.md) · [Español](sheets.es.md)

The info sheet (right-click the game › **Game info…**, or **Help › Game info…**) and the
game's name and box art in the library come from the **game database**, embedded in the
program. It is kept apart from the emulator's own texts (menus, buttons), which are the
interface translations in `assets/i18n/`.

| File | What it has |
|---|---|
| `crates/v-bear-core/src/games.rs` | the data of each game, the same in every language: a table in the code (part B) |
| `assets/games/games.en.json` | the texts of each game in English, the source text (part A) |
| `assets/games/games.<code>.json` | the same texts in another language (part A) |

Where it comes from: the 12 games, their CRC32 and their No-Intro names come from the No-Intro set
(copy in libretro-database) and MAME's software list (`hash/supracan.xml`, CC0); titles, years and
publishers from the list; developers as in Wikipedia's list; the number of players from the LaunchBox Games
DB. The descriptions were written for V-Bear from the games themselves, because MAME's list only has
emulation notes. Sources and dates are in [References](references.md). The set is complete (12 released
games) and does not change: a new game would be a new entry.

## A. The texts: `games.<code>.json`

Read by the interface. The core hands them over in `SystemInfo::game_texts`
(`&[("en", include_str!(…)), …]`), keyed by the game's id (`GameInfo::id`).

```json
{
  "game-id": {
    "genre": "Platform",
    "description": "What the game is about…",
    "notes": "…"
  }
}
```

| Field | Notes |
|---|---|
| key (`game-id`) | the game's id: lower case, no spaces; the same key in the table and in every text file |
| `genre` | a word or two, translated in each file; shown on the sheet and in the library's list view |
| `description` | shown on the sheet |
| `notes` | shown at the end of the sheet |

All fields are optional. Without the text in the current language, the sheet shows
English. An unknown field (a typo such as `descripton`) is an error. A new language is a new
`games.<code>.json` with the same games ([translating.md](translating.md)).

## B. The data: the `GAMES` table

Read by the core (`games.rs`), which answers `SystemInfo::game_info(rom)` with a `GameInfo` (`emu-api`):
`id`, `title`, `alt_title`, `year`, `publisher`, `developer`, `serial`, `players`, `thumbnail`. In V-Bear the
data is not a JSON file but a Rust table, one row per game:

```rust
(0x6099bb44, "boomzoo", "Boom Zoo", "Bao Bao Dong Wu Yuan", "1996", "Funtech", "Boom Zoo (Taiwan)", 2),
```

| Field | Notes |
|---|---|
| CRC32 | of the ROM as the frontend reads it (a zip's files joined, largest first): this is how the game is recognized, whatever the file name. A ROM is also looked up with its 16-bit words swapped, because dumps exist in both byte orders |
| id | lower case, no spaces; the same key as in the text files |
| title | the game's name |
| alternative title | the title in pinyin |
| year | |
| developer | as in Wikipedia's list of Super A'Can games |
| thumbnail | the ROM's No-Intro name, which is the name of its box art, title and gameplay images, as in RetroArch ([libretro-thumbnails](https://thumbnails.libretro.com/)) |
| players | most players at once, from the LaunchBox Games DB; only the number, the phrase ("1 or 2 players") comes from the interface translation |

The publisher is the same for every game (Funtech: the boxes say who made the game and that Funtech
(敦煌) published it), so it is a constant, and no serial number is recorded.

A ROM that is not in the table (a hack, a homebrew) gets no sheet. The CRC32 is `emu_api::crc32` over
the bytes of the ROM; V-Bear has no command-line option to print it.

## Adding a game

1. Find its CRC32 and its No-Intro name.
2. Add a row to `GAMES` (and raise the array's length).
3. Add its texts to `games.en.json`, `games.pt.json` and `games.es.json`.
4. Check: `cargo test --workspace` (the test `texts_cover_every_game` fails if a file has a different set of games)
   and open the game's info sheet.

## Rules (put them in the tests)

- Every text file has **the same games and the same fields**; only the text changes
  (`texts_cover_every_game`; the frontend also checks the fields: `game_texts_valid`).
- Only verifiable content goes in: developer with a source, description from the author
  or from the game.

**Non-Latin titles:** the interface embeds a subset of Noto Sans CJK JP
(`assets/fonts/`, SIL OFL): kana, punctuation and only the kanji already in use. A title
with new kanji requires regenerating the subset.
