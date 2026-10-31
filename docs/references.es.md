# Referencias

[English](references.md) · [Português](references.pt-BR.md) · **Español**

Registro de las fuentes consultadas para V-Bear. **Solo añadir** — las fuentes antiguas no se
borran ni se reemplazan; una versión nueva de una fuente se convierte en una entrada nueva, con su fecha.
`hardware.es.md` cita estas entradas como [n].

Leyenda de tipo: código · doc · wiki · foro · noticia.

## 2026-10-20 — investigación inicial

Commits consultados:

| Repositorio | Commit |
|---|---|
| mamedev/mame | `60dbfc6f256dc56c5985fae05c29682c3f24e7b6` |
| MiSTer-devel/SuperACan_MiSTer | `59ba2750592f825aee73003142209c9f0f4d1cf6` |
| angelosa/hw_docs | `a1bc727331c74d561552fe6846c8b6a04f7aa974` |
| kitrinx/ACan | `c67a883a84ad47090751f1404e9a1fdc684ae7e9` |

Prefijos: **M** = `https://github.com/mamedev/mame/blob/60dbfc6f256dc56c5985fae05c29682c3f24e7b6/` ·
**S** = `https://github.com/MiSTer-devel/SuperACan_MiSTer/blob/59ba2750592f825aee73003142209c9f0f4d1cf6/`

| # | Título | URL | Tipo | Wayback |
|---|---|---|---|---|
| 1 | MAME supracan.cpp | M`src/mame/umc/supracan.cpp` | código | – |
| 2 | MAME umc6619_sound.cpp/.h | M`src/mame/umc/umc6619_sound.cpp` | código | – |
| 3 | MAME umc6650.cpp | M`src/mame/umc/umc6650.cpp` | código | – |
| 4 | MAME bus supracan rom/slot | M`src/devices/bus/supracan/rom.cpp` (+ `slot.cpp`) | código | – |
| 5 | MAME softlist | M`hash/supracan.xml` | código/datos | – |
| 6 | MiSTer README | S`Readme.md` | doc | – |
| 7 | mainboard.sv | S`rtl/board/mainboard.sv` | código | – |
| 8 | um6619.sv | S`rtl/um6619/um6619.sv` | código | – |
| 9 | um6619_irq.sv | S`rtl/um6619/um6619_irq.sv` | código | – |
| 10 | um6619_frc.sv | S`rtl/um6619/um6619_frc.sv` | código | – |
| 11 | um6619_overlay.sv | S`rtl/um6619/um6619_overlay.sv` | código | – |
| 12 | um6619_cpu.sv | S`rtl/um6619/um6619_cpu.sv` | código | – |
| 13 | um6619_snd.sv | S`rtl/um6619/um6619_snd.sv` | código | – |
| 14 | um6619_dma.sv | S`rtl/um6619/um6619_dma.sv` | código | – |
| 15 | um6618.sv | S`rtl/um6618/um6618.sv` | código | – |
| 16 | um6618_regs.sv | S`rtl/um6618/um6618_regs.sv` | código | – |
| 17 | um6618_vtiming.sv | S`rtl/um6618/um6618_vtiming.sv` | código | – |
| 18 | um6618_tilemap.sv | S`rtl/um6618/um6618_tilemap.sv` | código | – |
| 19 | um6618_roz.sv | S`rtl/um6618/um6618_roz.sv` | código | – |
| 20 | um6618_sprite.sv | S`rtl/um6618/um6618_sprite.sv` | código | – |
| 21 | um6618_mixer.sv | S`rtl/um6618/um6618_mixer.sv` | código | – |
| 22 | um6618_window.sv | S`rtl/um6618/um6618_window.sv` | código | – |
| 23 | um6618_vram_arb.sv | S`rtl/um6618/um6618_vram_arb.sv` | código | – |
| 24 | Cartucho/lockout | S`rtl/cart/um6650.sv`, `cart_16003_1b.sv`, `cart_slot.sv`, S`rtl/board/cart_loader.sv` | código | – |
| 25 | bios_loader.sv | S`rtl/board/bios_loader.sv` | código | – |
| 26 | joypad/multitap | S`rtl/board/joypad.sv`, S`rtl/board/multitap.sv` | código | – |
| 27 | um70c188.sv | S`rtl/board/um70c188.sv` | código | – |
| 28 | Licencias MiSTer | S`LICENSE`, S`rtl/fx68k/LICENSE`, S`rtl/6502/mos6502.sv` | código | – |
| 29 | angelosa hw_docs — pergame.md | https://github.com/angelosa/hw_docs/blob/a1bc727331c74d561552fe6846c8b6a04f7aa974/funtech_superacan/pergame.md | doc | no encontrado |
| 30 | evadot — pinout/lista de chips (gist) | https://gist.github.com/evadot/66cfdb8891544b41b4c9 (revisión no fijada) | doc | https://web.archive.org/web/20240908064156/https://gist.github.com/evadot/66cfdb8891544b41b4c9 |
| 31 | Wikipedia — Super A'Can | https://en.wikipedia.org/wiki/Super_A%27Can | wiki | https://web.archive.org/web/20260427233007/https://en.wikipedia.org/wiki/Super_A%27can |
| 32 | Batocera wiki — supracan | https://wiki.batocera.org/systems:supracan | wiki | https://web.archive.org/web/20260607050332/https://wiki.batocera.org/systems:supracan |
| 33 | kitrinx/ACan (traducciones y trucos) | https://github.com/kitrinx/ACan/tree/c67a883a84ad47090751f1404e9a1fdc684ae7e9 | código | – |
| 34 | NESdev — hilo ZapFC, mensaje de etabeta (2011) | https://forums.nesdev.org/viewtopic.php?p=74760 (t=7494) | foro | https://web.archive.org/web/20230826065022/https://forums.nesdev.org/viewtopic.php?t=7494 |
| 35 | NESdev — "Top 10 Failed Gaming Consoles" (2010) | https://forums.nesdev.org/viewtopic.php?p=69563 (t=7128) | foro | https://web.archive.org/web/20230826083326/https://forums.nesdev.org/viewtopic.php?t=7128 |
| 36 | NESdev wiki — búsquedas sin resultados | https://www.nesdev.org/w/index.php?search=%22Super+A%27Can%22&fulltext=1 | wiki | – |
| 37 | Multiplayer.it — MAME 0.269 | https://multiplayer.it/notizie/la-versione-0269-1-del-mame-consente-di-emulare-una-console-rarissima.html | noticia | https://web.archive.org/web/20250712072456/https://multiplayer.it/notizie/la-versione-0269-1-del-mame-consente-di-emulare-una-console-rarissima.html |
| 38 | Foro de LaunchBox (2023) | https://forums.launchbox-app.com/topic/74179-funtech-super-acan/ | foro | https://web.archive.org/web/20250911083710/https://forums.launchbox-app.com/topic/74179-funtech-super-acan/ |
| 39 | Giant Bomb — Super A'Can | https://www.giantbomb.com/super-acan/3045-151/ | wiki | https://web.archive.org/web/20250605112025/https://www.giantbomb.com/super-acan/3045-151/ |
| 40 | Time Extension — Piko compra IPs (2026-05) | https://www.timeextension.com/news/2026/05/piko-interactive-acquires-two-more-super-acan-ips-teases-something-new-is-on-the-cards | noticia | no verificado (solo fragmento de búsqueda; la página dio 403) |

No consultados (búsquedas sin página técnica): Sega-16, AssemblerGames, vídeos de Diskman.
Inaccesibles: gametechwiki (403).

Punto de partida del proyecto: `Referencias.txt` (core de MiSTer + driver de MAME).

## 2026-10-20 — ROMs proporcionadas por el usuario

Carpeta `roms - Funtech - Super Acan/` (fuera de git): 12 zips, nombres No-Intro. Los CRC32 de los 13 archivos
coinciden con el softlist de MAME [5]. 11 volcados tienen los bytes intercambiados por palabra (`load16_word_swap`
en el softlist); solo **Rebel Star (En)** está en big-endian. **Super Dragon Force** viene en dos partes:
`16007` (2 MiB, en `$000000`) + `08002` (1 MiB, en `$200000`; `08007.1` en MAME).
Ningún archivo de BIOS vino incluido.

## 2026-10-20 — BIOS y correcciones durante la implementación

- **BIOS** de la colección local del usuario (`~/Projetos/Bios/retrobios/bios/Funtech/Super A'Can/`):
  `supracan.zip` (`internal_68k.bin` `8d575662`, `umc6650.bin` `0ba78597`) y `.variants/supracan.zip`
  (`internal_6502_1.bin` `fc9fb05f`, `internal_6502_2.bin` `bf950ab7`, `internal_68k.bin`). CRCs iguales
  a los de MAME [3] y MiSTer [6]. Copia local en `bios/` (fuera de git).
- **Corrección — la sound RAM tiene 64 KiB:** `rtl/board/mainboard.sv` (U11, `sram_async #(.ADDR_WIDTH(16))`) [7].
  El informe inicial decía 32 KiB.
- **Archivos de MiSTer leídos en la implementación** (mismo commit `59ba275`, ya listados arriba):
  `um6619.sv` [8], `um6619_irq.sv` [9], `um6619_frc.sv` [10], `um6619_overlay.sv` [11], `um6619_cpu.sv`
  [12], `um6619_snd.sv` [13], `um6619_dma.sv` [14], `um6618.sv` [15], `um6618_tilemap.sv` [18],
  `um6618_roz.sv` [19], `um6618_sprite.sv` [20], `um6618_mixer.sv` [21], `um6618_window.sv` [22],
  `cart/um6650.sv` [24], `board/joypad.sv` [26].
- **Bibliotecas Rust usadas:** `m68k` 0.14.6 (MIT, https://github.com/benletchford/m68k-rs) para el 68000;
  `mos6502` 0.10.1 (BSD-3-Clause, https://crates.io/crates/mos6502/0.10.1) para el 65C02.

## 2026-10-21 — copias locales

- Las copias del 2026-10-20 (en /tmp) se perdieron; se descargaron de nuevo en los mismos commits a
  `_referencias/` (fuera de git): MAME `60dbfc6` (7 archivos) y MiSTer `59ba275` (clon completo).
- Usado en esta etapa: MAME `supracan.cpp` [1] (ventana 0, raster IRQ, `$E90010` "smearing mirror").

## 2026-10-21 — MAME como referencia de ejecución

- **MAME 0.289** (Flatpak `org.mamedev.MAME`, Flathub, commit de Flatpak `41ebdb5c`), driver `supracan`, ejecutado
  sin ventana con un script Lua (`-autoboot_script`) que pulsa los mismos botones y captura los mismos
  fotogramas que el modo sin ventana de V-Bear. Audio mediante `-wavwrite`. Herramientas en `tools/mame/` (fuera de git).
  MAME exige los cuatro archivos de la BIOS; el `supracan.zip` principal solo tiene dos.
- Hallazgo: en esa versión, sin ninguna entrada, MAME avanza solo por títulos y menús, aunque los puertos
  `P1`/`P2` queden en reposo (`FFFF`, activo-bajo, misma polaridad que el `joypad.sv` de MiSTer [26] y
  V-Bear). Causa no investigada. Por eso MAME solo sirve para comparar escenas iguales, no la secuencia del juego.

## 2026-10-21 — bibliotecas del savestate

- `serde` 1.0.229 (MIT/Apache-2.0, https://crates.io/crates/serde/1.0.229) y `bincode` 1.3.3
  (MIT, https://crates.io/crates/bincode/1.3.3).
- `mos6502` 0.10.1 leído en `src/cpu.rs`: `wait_state` y `last_nmi_state` son privados (solo existe
  `wait_state()` para lectura), por eso V-Bear los recompone al cargar un estado.

## 2026-10-21 — capturas en PNG

- `flate2` 1.1.10 (MIT/Apache-2.0, https://crates.io/crates/flate2/1.1.10), ya presente por `zip`.
- Formato PNG: W3C, *Portable Network Graphics (PNG) Specification (Third Edition)*,
  https://www.w3.org/TR/png-3/ (IHDR/IDAT/IEND, CRC-32, filtro 0 por fila).

## 2026-10-21 — base de la interfaz y licencia

- **estrutura-novos-emuladores-rust** (privado, PSGM-Romhacking), `git@github-psgm:PSGM-Romhacking/_estrutura-novos-emuladores-rust.git`,
  commit `5b61218` (2026-10-20), más cambios locales sin commitear del checkout
  `~/Projetos/_estrutura-novos-emuladores-rust` (2026-10-21 12:29). Instrucciones leídas: `README.md`,
  `CLAUDE.md`, `SISTEMA.md`, `docs/guia.md`, `crates/api/src/lib.rs`, `tools/init-project.sh`,
  `tools/new-emulator.sh`. Licencia GPL-3.0-or-later.
- **Licencia de V-Bear** pasa a ser GPL-3.0-or-later (decisión del dueño). Salvedad registrada: en el core
  de MiSTer [6], `SuperAcan.sv` dice "version 2 of the License, or (at your option) any later version";
  los archivos de los chips (`rtl/um6618/*`, `rtl/um6619/*`) solo tienen "Copyright (c) 2026 Jamie Blanks" y el
  `LICENSE` del repositorio es el texto de la GPLv2. V-Bear reimplementa el comportamiento en Rust, sin
  copiar el HDL.
- Libretro: `libretro.h` (libretro-common), constantes `RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY` (9) y
  `RETRO_MEMORY_SAVE_RAM` (0); formato del `.info` con `firmwareN_*` como en los cores de RetroArch.

## 2026-10-21 — DAT de No-Intro y carátulas

- **No-Intro, Funtech - Super Acan** (copia en libretro-database), https://raw.githubusercontent.com/libretro/libretro-database/master/metadat/no-intro/Funtech%20-%20Super%20Acan.dat,
  commit `9a899f5` (2026-10-08). Nombre oficial `Rebel Star (Taiwan) (En)`, CRC32 `9e243c91` (2 MiB). El
  archivo del usuario con ese nombre tiene CRC32 `a3469172`: es el mismo volcado con las palabras de 16 bits
  intercambiadas (el otro orden de bytes da exactamente `9e243c91`). Los otros 11 coinciden con el DAT.
- **libretro-thumbnails, Funtech_-_Super_Acan**, https://github.com/libretro-thumbnails/Funtech_-_Super_Acan,
  commit `42814d9` (2025-01-13): Named_Boxarts, Named_Titles y Named_Snaps con 11 juegos cada uno; no hay
  ninguna imagen de Rebel Star.
- El 2026-10-21, a petición del usuario, la copia local se renombró a `Rebel Star (Taiwan).zip` (archivo
  interno `Rebel Star (Taiwan).bin`, contenido igual, CRC32 `a3469172`): el juego está en chino tradicional;
  solo el logo y el menú tienen palabras en inglés. La base de datos de juegos sigue usando el nombre de No-Intro para
  las carátulas.

## 2026-10-21 — descripciones de los juegos

- Las descripciones en `assets/games/games.{en,pt,es}.json` se escribieron para V-Bear, a partir de lo que
  aparece en los propios juegos (capturas de las pruebas, menús, pantallas de título) y en las cajas (por ejemplo
  "S.RPG" en Super Dragon Force). El softlist de MAME [5] solo tiene notas sobre fallos de emulación. La
  mención de los cartuchos de muestra de Rebel Star viene de 百度百科 (A'can), https://baike.baidu.com/item/A'can
  (acceso el 2026-10-21). Formato copiado de Mil Capivaras (`docs/sheets.md`, commit `fecf86c`).

## 2026-10-21 — investigación sobre los juegos (género, jugadores, descripciones)

Hechos usados en las descripciones, géneros y número de jugadores de `assets/games/` y `games.rs`, reescritos
con nuestras palabras (ningún texto copiado). Acceso el 2026-10-21.

- **Wikipedia (en), Super A'Can**, https://en.wikipedia.org/wiki/Super_A%27Can — código, desarrolladora y
  género de cada juego; nombres alternativos (C.U.G., Sonic Dragon, African Adventures).
- **Wikipedia (zh), Super A'Can**, https://zh.wikipedia.org/wiki/Super_A%27Can — Formosa Duel parodia
  a figuras públicas taiwanesas, con voces en taiwanés (hokkien); Rebel Star solo en cartuchos de muestra.
- **Wikipedia (en), Sango Fighter**, https://en.wikipedia.org/wiki/Sango_Fighter — original de 1993 para
  MS-DOS (Panda Entertainment); conversión para A'Can hecha a toda prisa por un solo empleado.
- **LaunchBox Games DB**, https://gamesdb.launchbox-app.com/ (base de datos comunitaria, sin citas; tratada como
  "según LaunchBox"): número de jugadores de los 12; tipo Bomberman (Boom Zoo); estilo Puyo Puyo y doce
  adversarios (Formosa Duel); mahjong taiwanés de 16 fichas (Gambling Lord); fotos digitalizadas (Magical
  Pool); granjas, canteras, hoteles y tesoro, hasta 6 jugadores (Monopoly); seis fases, taco, bolas,
  fase de bonus en 3D (Speedy Dragon); estilo Shining Force y ciudades vistas desde arriba (Super Dragon Force);
  equipos de la CPBL con licencia, animaciones y público digitalizado (béisbol); elecciones que hacen al héroe bueno
  o malo (The Son of Evil); modos arcade y versus (Sango Fighter).
- **PPE.pl, Historia konsol Funtech Super A'Can**, https://www.ppe.pl/publicystyka/202436/Historia-konsol-Funtech-Super-A-Can.html
  — Speedy Dragon como el "Sonic taiwanés"; Journey to the Laugh como el juego más popular.
- **RF Generation, Funtech Super A'Can**, https://www.rfgeneration.com/features/system-overview/237/ —
  Journey to the Laugh entre los mejores juegos de la consola.
- De los propios juegos (capturas de V-Bear): el "21.º escuadrón" y el "imperio Titán" de Rebel Star (diálogo e
  introducción); modos del billar y del béisbol (menús); temporada del béisbol guardada en la SRAM.
- Conflictos anotados: el género de Rebel Star es estrategia en LaunchBox y RPG en las Wikipedias (usado:
  estrategia, que coincide con lo que se ve en el juego); Formosa Duel "celebridades chinas" (LaunchBox) ×
  "políticos taiwaneses" (WP-zh), resumido como "figuras públicas de Taiwán". Bloqueados: MobyGames,
  Bahamut, avid.wiki, Vogons (403/Cloudflare).
- Desarrolladora × editora: las cajas (carátulas de libretro-thumbnails y la foto de Rebel Star) traen "…製作 ◎
  敦煌發行" — hecho por …, publicado por 敦煌 (Funtech). El campo `publisher` del softlist de MAME [5] es, en
  realidad, la desarrolladora. Desarrolladoras según la lista de la Wikipedia (en); Super Taiwanese Baseball
  League: "全崴製作" en la caja = Quan Wei Technology (la Wikipedia da "Quan Wei Technology/C&E").

## 2026-10-21 — SRAM de batería

- MAME `hash/supracan.xml` [5] (`dataarea name="nvram" size="0x8000"` en sonevil, staiwbbl, slghtsag,
  monopoly, gamblord, rebelst; gamblord con "TODO: unconfirmed size and actual presence") y
  `bus/supracan/rom.cpp` (`battery_load(..., 0xff)`: MAME inicia la SRAM con `0xFF`).
- MiSTer [6] `rtl/cart/cart_16003_1b.sv` (U2 62256, `A[15:1]`, solo el byte bajo; "a save file is 32 KiB
  over a 64 KiB window"), `rtl/mem/save_slot.sv` y `SuperAcan.sv` (el `.sav`; la SRAM no se borra al
  cambiar de cartucho).

## 2026-10-23 — homebrew

Se buscaron juegos homebrew, demos o ROMs de prueba libres para la Super A'Can (para `Brand::free_games`
y para pruebas): ninguno encontrado. Las búsquedas en la web (en inglés; "homebrew", "SDK", "test rom", "prototype",
"supracan" + GitHub) solo traen la biblioteca comercial. El softlist de MAME [5] tiene solo los 12 juegos
comerciales; el repositorio de MiSTer [6] no trae ninguna ROM de prueba.
- Games That Weren't, "Quick Fighting Attack" y "City Escape" (marzo de 2026):
  https://www.gamesthatwerent.com/2026/03/quick-fighting-attack/ ,
  https://www.gamesthatwerent.com/2026/03/city-escape/ — juegos cancelados, solo imágenes (vídeo "Let's
  visit Funtech!" de Brandon Cobb), "No download available"; se dice que muchas ROMs de Funtech fueron
  destruidas.

## 2026-10-30 — el core MiSTer simulado

- **Verilator** 5.046 (2026-02-28, paquete de Fedora), https://www.veripool.org/verilator/ — simula el
  SystemVerilog del core MiSTer ciclo a ciclo (`reference/`). Necesitó `-Wno-BLKANDNBLK` para `fx68k`.
- El README del core MiSTer [6], "Development and QA": cómo se validó el core (vídeos de hardware real que
  muestran pantallas de demostración, de Diskman, desensamblados, sin prueba en consola). El repositorio tiene un
  único commit, de 2026-09-11 (`59ba275`).
- **MAME 0.289** (Flatpak, el mismo de 2026-10-21), como tercera fuente: capturas de Magical Pool cerca del cuadro
  300 (el círculo creciente de la ventana 0) con `tools/mame/drive.lua`.
- `sevenz-rust` 0.6 (Apache-2.0, https://crates.io/crates/sevenz-rust/0.6.1), ya en el workspace: lee los archivos
  `.7z` (juegos con varios chips, BIOS dentro de un `.7z`).
