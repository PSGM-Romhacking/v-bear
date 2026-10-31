# Base de datos de juegos

[English](sheets.md) · [Português](sheets.pt-BR.md) · **Español**

La ficha de información (clic derecho en el juego › **Ficha del juego…**, o **Ayuda › Ficha del juego…**) y el
nombre y la carátula del juego en la biblioteca provienen de la **base de datos de juegos**, incrustada en el
programa. Está separada de los textos propios del emulador (menús, botones), que son las
traducciones de la interfaz en `assets/i18n/`.

| Archivo | Qué contiene |
|---|---|
| `crates/v-bear-core/src/games.rs` | los datos de cada juego, iguales en todos los idiomas: una tabla en el código (parte B) |
| `assets/games/games.en.json` | los textos de cada juego en inglés, el texto fuente (parte A) |
| `assets/games/games.<code>.json` | los mismos textos en otro idioma (parte A) |

De dónde viene: los 12 juegos, sus CRC32 y sus nombres No-Intro provienen del conjunto No-Intro
(copia en libretro-database) y de la lista de software de MAME (`hash/supracan.xml`, CC0); los títulos, los años y
las distribuidoras, de la lista; los desarrolladores, como en la lista de Wikipedia; el número de jugadores, de LaunchBox Games
DB. Las descripciones se escribieron para V-Bear a partir de los propios juegos, porque la lista de MAME solo tiene
notas de emulación. Las fuentes y las fechas están en [Referencias](references.es.md). El conjunto está completo (12 juegos
lanzados) y no cambia: un juego nuevo sería una entrada nueva.

## A. Los textos: `games.<code>.json`

Los lee la interfaz. El núcleo los entrega en `SystemInfo::game_texts`
(`&[("en", include_str!(…)), …]`), indexados por el id del juego (`GameInfo::id`).

```json
{
  "game-id": {
    "genre": "Platform",
    "description": "What the game is about…",
    "notes": "…"
  }
}
```

| Campo | Notas |
|---|---|
| clave (`game-id`) | el id del juego: minúsculas, sin espacios; la misma clave en la tabla y en todos los archivos de texto |
| `genre` | una o dos palabras, traducidas en cada archivo; se muestra en la ficha y en la vista de lista de la biblioteca |
| `description` | se muestra en la ficha |
| `notes` | se muestran al final de la ficha |

Todos los campos son opcionales. Sin el texto en el idioma actual, la ficha muestra el
inglés. Un campo desconocido (un error de escritura como `descripton`) es un error. Un idioma nuevo es un nuevo
`games.<code>.json` con los mismos juegos ([translating.es.md](translating.es.md)).

## B. Los datos: la tabla `GAMES`

La lee el núcleo (`games.rs`), que responde a `SystemInfo::game_info(rom)` con un `GameInfo` (`emu-api`):
`id`, `title`, `alt_title`, `year`, `publisher`, `developer`, `serial`, `players`, `thumbnail`. En V-Bear los
datos no están en un archivo JSON, sino en una tabla de Rust, una fila por juego:

```rust
(0x6099bb44, "boomzoo", "Boom Zoo", "Bao Bao Dong Wu Yuan", "1996", "Funtech", "Boom Zoo (Taiwan)", 2),
```

| Campo | Notas |
|---|---|
| CRC32 | de la ROM tal como la lee el frontend (los archivos de un zip unidos, el mayor primero): así se reconoce el juego, sea cual sea el nombre del archivo. Una ROM también se busca con sus palabras de 16 bits intercambiadas, porque existen volcados en ambos órdenes de bytes |
| id | minúsculas, sin espacios; la misma clave que en los archivos de texto |
| title | el nombre del juego |
| alternative title | el título en pinyin |
| year | |
| developer | como en la lista de juegos de Super A'Can de Wikipedia |
| thumbnail | el nombre No-Intro de la ROM, que es el nombre de sus imágenes de carátula, de título y de juego, como en RetroArch ([libretro-thumbnails](https://thumbnails.libretro.com/)) |
| players | máximo de jugadores a la vez, de LaunchBox Games DB; solo el número, la frase ("1 or 2 players") viene de la traducción de la interfaz |

La distribuidora es la misma para todos los juegos (Funtech: las cajas dicen quién hizo el juego y que Funtech
(敦煌) lo publicó), por lo que es una constante, y no se registra ningún número de serie.

Una ROM que no está en la tabla (un hack, un homebrew) no obtiene ficha. El CRC32 es `emu_api::crc32` sobre
los bytes de la ROM; V-Bear no tiene ninguna opción de línea de comandos para mostrarlo.

## Añadir un juego

1. Busca su CRC32 y su nombre No-Intro.
2. Añade una fila a `GAMES` (y aumenta la longitud del array).
3. Añade sus textos a `games.en.json`, `games.pt.json` y `games.es.json`.
4. Comprueba: `cargo test --workspace` (la prueba `texts_cover_every_game` falla si un archivo tiene un conjunto de juegos distinto)
   y abre la ficha de información del juego.

## Reglas (ponlas en las pruebas)

- Todo archivo de texto tiene **los mismos juegos y los mismos campos**; solo cambia el texto
  (`texts_cover_every_game`; el frontend también comprueba los campos: `game_texts_valid`).
- Solo entra contenido verificable: desarrollador con fuente, descripción del autor
  o del juego.

**Títulos no latinos:** la interfaz incrusta un subconjunto de Noto Sans CJK JP
(`assets/fonts/`, SIL OFL): kana, puntuación y solo los kanji que ya se usan. Un título
con kanji nuevos requiere regenerar el subconjunto.
