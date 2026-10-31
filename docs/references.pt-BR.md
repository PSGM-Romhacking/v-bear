# Referências

[English](references.md) · **Português** · [Español](references.es.md)

Registro das fontes consultadas para o V-Bear. **Só acrescentar** — fontes antigas não são
apagadas nem substituídas; uma versão nova de uma fonte vira uma entrada nova, com a data.
O `docs/hardware.pt-BR.md` cita estas entradas como [n].

Legenda de tipo: código · doc · wiki · fórum · notícia.

## 2026-10-20 — pesquisa inicial

Commits consultados:

| Repositório | Commit |
|---|---|
| mamedev/mame | `60dbfc6f256dc56c5985fae05c29682c3f24e7b6` |
| MiSTer-devel/SuperACan_MiSTer | `59ba2750592f825aee73003142209c9f0f4d1cf6` |
| angelosa/hw_docs | `a1bc727331c74d561552fe6846c8b6a04f7aa974` |
| kitrinx/ACan | `c67a883a84ad47090751f1404e9a1fdc684ae7e9` |

Prefixos: **M** = `https://github.com/mamedev/mame/blob/60dbfc6f256dc56c5985fae05c29682c3f24e7b6/` ·
**S** = `https://github.com/MiSTer-devel/SuperACan_MiSTer/blob/59ba2750592f825aee73003142209c9f0f4d1cf6/`

| # | Título | URL | Tipo | Wayback |
|---|---|---|---|---|
| 1 | MAME supracan.cpp | M`src/mame/umc/supracan.cpp` | código | – |
| 2 | MAME umc6619_sound.cpp/.h | M`src/mame/umc/umc6619_sound.cpp` | código | – |
| 3 | MAME umc6650.cpp | M`src/mame/umc/umc6650.cpp` | código | – |
| 4 | MAME bus supracan rom/slot | M`src/devices/bus/supracan/rom.cpp` (+ `slot.cpp`) | código | – |
| 5 | MAME softlist | M`hash/supracan.xml` | código/dados | – |
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
| 28 | Licenças MiSTer | S`LICENSE`, S`rtl/fx68k/LICENSE`, S`rtl/6502/mos6502.sv` | código | – |
| 29 | angelosa hw_docs — pergame.md | https://github.com/angelosa/hw_docs/blob/a1bc727331c74d561552fe6846c8b6a04f7aa974/funtech_superacan/pergame.md | doc | não encontrado |
| 30 | evadot — pinout/lista de chips (gist) | https://gist.github.com/evadot/66cfdb8891544b41b4c9 (revisão não fixada) | doc | https://web.archive.org/web/20240908064156/https://gist.github.com/evadot/66cfdb8891544b41b4c9 |
| 31 | Wikipedia — Super A'Can | https://en.wikipedia.org/wiki/Super_A%27Can | wiki | https://web.archive.org/web/20260427233007/https://en.wikipedia.org/wiki/Super_A%27can |
| 32 | Batocera wiki — supracan | https://wiki.batocera.org/systems:supracan | wiki | https://web.archive.org/web/20260607050332/https://wiki.batocera.org/systems:supracan |
| 33 | kitrinx/ACan (traduções e cheats) | https://github.com/kitrinx/ACan/tree/c67a883a84ad47090751f1404e9a1fdc684ae7e9 | código | – |
| 34 | NESdev — thread ZapFC, post de etabeta (2011) | https://forums.nesdev.org/viewtopic.php?p=74760 (t=7494) | fórum | https://web.archive.org/web/20230826065022/https://forums.nesdev.org/viewtopic.php?t=7494 |
| 35 | NESdev — "Top 10 Failed Gaming Consoles" (2010) | https://forums.nesdev.org/viewtopic.php?p=69563 (t=7128) | fórum | https://web.archive.org/web/20230826083326/https://forums.nesdev.org/viewtopic.php?t=7128 |
| 36 | NESdev wiki — buscas sem resultado | https://www.nesdev.org/w/index.php?search=%22Super+A%27Can%22&fulltext=1 | wiki | – |
| 37 | Multiplayer.it — MAME 0.269 | https://multiplayer.it/notizie/la-versione-0269-1-del-mame-consente-di-emulare-una-console-rarissima.html | notícia | https://web.archive.org/web/20250712072456/https://multiplayer.it/notizie/la-versione-0269-1-del-mame-consente-di-emulare-una-console-rarissima.html |
| 38 | LaunchBox forum (2023) | https://forums.launchbox-app.com/topic/74179-funtech-super-acan/ | fórum | https://web.archive.org/web/20250911083710/https://forums.launchbox-app.com/topic/74179-funtech-super-acan/ |
| 39 | Giant Bomb — Super A'Can | https://www.giantbomb.com/super-acan/3045-151/ | wiki | https://web.archive.org/web/20250605112025/https://www.giantbomb.com/super-acan/3045-151/ |
| 40 | Time Extension — Piko compra IPs (2026-05) | https://www.timeextension.com/news/2026/05/piko-interactive-acquires-two-more-super-acan-ips-teases-something-new-is-on-the-cards | notícia | não verificado (só snippet de busca; página deu 403) |

Não consultados (buscas sem página técnica): Sega-16, AssemblerGames, vídeos do Diskman.
Inacessíveis: gametechwiki (403).

Ponto de partida do projeto: `Referencias.txt` (core MiSTer + driver do MAME).

## 2026-10-20 — ROMs fornecidas pelo usuário

Pasta `roms - Funtech - Super Acan/` (fora do git): 12 zips, nomes No-Intro. Os CRC32 dos 13 arquivos
batem com o softlist do MAME [5]. 11 dumps estão com bytes trocados por palavra (`load16_word_swap`
no softlist); só o **Rebel Star (En)** está em big-endian. **Super Dragon Force** vem em duas partes:
`16007` (2 MiB, em `$000000`) + `08002` (1 MiB, em `$200000`; `08007.1` no MAME).
Nenhum arquivo de BIOS veio junto.

## 2026-10-20 — BIOS e correções durante a implementação

- **BIOS** da coleção local do usuário (`~/Projetos/Bios/retrobios/bios/Funtech/Super A'Can/`):
  `supracan.zip` (`internal_68k.bin` `8d575662`, `umc6650.bin` `0ba78597`) e `.variants/supracan.zip`
  (`internal_6502_1.bin` `fc9fb05f`, `internal_6502_2.bin` `bf950ab7`, `internal_68k.bin`). CRCs iguais
  aos do MAME [3] e do MiSTer [6]. Cópia local em `bios/` (fora do git).
- **Correção — sound RAM tem 64 KiB:** `rtl/board/mainboard.sv` (U11, `sram_async #(.ADDR_WIDTH(16))`) [7].
  O relatório inicial dizia 32 KiB.
- **Arquivos do MiSTer lidos na implementação** (mesmo commit `59ba275`, já listados acima):
  `um6619.sv` [8], `um6619_irq.sv` [9], `um6619_frc.sv` [10], `um6619_overlay.sv` [11], `um6619_cpu.sv`
  [12], `um6619_snd.sv` [13], `um6619_dma.sv` [14], `um6618.sv` [15], `um6618_tilemap.sv` [18],
  `um6618_roz.sv` [19], `um6618_sprite.sv` [20], `um6618_mixer.sv` [21], `um6618_window.sv` [22],
  `cart/um6650.sv` [24], `board/joypad.sv` [26].
- **Bibliotecas Rust usadas:** `m68k` 0.14.6 (MIT, https://github.com/benletchford/m68k-rs) para o 68000;
  `mos6502` 0.10.1 (BSD-3-Clause, https://crates.io/crates/mos6502/0.10.1) para o 65C02.

## 2026-10-21 — cópias locais

- As cópias de 2026-10-20 (em /tmp) se perderam; baixadas de novo nos mesmos commits para
  `_referencias/` (fora do git): MAME `60dbfc6` (7 arquivos) e MiSTer `59ba275` (clone completo).
- Usado nesta etapa: MAME `supracan.cpp` [1] (janela 0, raster IRQ, `$E90010` "smearing mirror").

## 2026-10-21 — MAME como referência de execução

- **MAME 0.289** (Flatpak `org.mamedev.MAME`, Flathub, commit Flatpak `41ebdb5c`), driver `supracan`, rodado
  sem janela com um script Lua (`-autoboot_script`) que aperta os mesmos botões e captura os mesmos
  quadros que o modo sem janela do V-Bear. Áudio por `-wavwrite`. Ferramentas em `tools/mame/` (fora do git).
  O MAME exige os quatro arquivos do BIOS; o `supracan.zip` principal só tem dois.
- Achado: nessa versão, sem entrada nenhuma, o MAME avança sozinho por títulos e menus, embora as portas
  `P1`/`P2` fiquem em repouso (`FFFF`, ativo-baixo, mesma polaridade do `joypad.sv` do MiSTer [26] e do
  V-Bear). Causa não investigada. Por isso o MAME só serve para comparar cenas iguais, não a sequência do jogo.

## 2026-10-21 — bibliotecas do savestate

- `serde` 1.0.229 (MIT/Apache-2.0, https://crates.io/crates/serde/1.0.229) e `bincode` 1.3.3
  (MIT, https://crates.io/crates/bincode/1.3.3).
- `mos6502` 0.10.1 lido em `src/cpu.rs`: `wait_state` e `last_nmi_state` são privados (só há
  `wait_state()` para leitura), por isso o V-Bear os recompõe ao carregar um estado.

## 2026-10-21 — capturas em PNG

- `flate2` 1.1.10 (MIT/Apache-2.0, https://crates.io/crates/flate2/1.1.10), já presente pelo `zip`.
- Formato PNG: W3C, *Portable Network Graphics (PNG) Specification (Third Edition)*,
  https://www.w3.org/TR/png-3/ (IHDR/IDAT/IEND, CRC-32, filtro 0 por linha).

## 2026-10-21 — base da interface e licença

- **estrutura-novos-emuladores-rust** (privado, PSGM-Romhacking), `git@github-psgm:PSGM-Romhacking/_estrutura-novos-emuladores-rust.git`,
  commit `5b61218` (2026-10-20), mais alterações locais não commitadas do checkout
  `~/Projetos/_estrutura-novos-emuladores-rust` (2026-10-21 12:29). Instruções lidas: `README.md`,
  `CLAUDE.md`, `SISTEMA.md`, `docs/guia.md`, `crates/api/src/lib.rs`, `tools/init-project.sh`,
  `tools/new-emulator.sh`. Licença GPL-3.0-or-later.
- **Licença do V-Bear** passa a GPL-3.0-or-later (decisão do dono). Ressalva registrada: no core
  MiSTer [6] o `SuperAcan.sv` diz "version 2 of the License, or (at your option) any later version";
  os arquivos dos chips (`rtl/um6618/*`, `rtl/um6619/*`) só têm "Copyright (c) 2026 Jamie Blanks" e o
  `LICENSE` do repositório é o texto da GPLv2. O V-Bear reimplementa o comportamento em Rust, sem
  copiar o HDL.
- Libretro: `libretro.h` (libretro-common), constantes `RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY` (9) e
  `RETRO_MEMORY_SAVE_RAM` (0); formato do `.info` com `firmwareN_*` como nos cores do RetroArch.

## 2026-10-21 — DAT do No-Intro e capas

- **No-Intro, Funtech - Super Acan** (cópia no libretro-database), https://raw.githubusercontent.com/libretro/libretro-database/master/metadat/no-intro/Funtech%20-%20Super%20Acan.dat,
  commit `9a899f5` (2026-10-08). Nome oficial `Rebel Star (Taiwan) (En)`, CRC32 `9e243c91` (2 MiB). O
  arquivo do usuário com esse nome tem CRC32 `a3469172`: é o mesmo dump com as palavras de 16 bits
  trocadas (a outra ordem de bytes dá exatamente `9e243c91`). Os outros 11 batem com o DAT.
- **libretro-thumbnails, Funtech_-_Super_Acan**, https://github.com/libretro-thumbnails/Funtech_-_Super_Acan,
  commit `42814d9` (2025-01-13): Named_Boxarts, Named_Titles e Named_Snaps com 11 jogos cada; não há
  nenhuma imagem do Rebel Star.
- Em 2026-10-21, a pedido do usuário, a cópia local foi renomeada para `Rebel Star (Taiwan).zip` (arquivo
  interno `Rebel Star (Taiwan).bin`, conteúdo igual, CRC32 `a3469172`): o jogo é em chinês tradicional;
  só o logo e o menu têm palavras em inglês. O banco de jogos continua usando o nome do No-Intro para
  as capas.

## 2026-10-21 — descrições dos jogos

- As descrições em `assets/games/games.{en,pt,es}.json` foram escritas para o V-Bear, a partir do que
  aparece nos próprios jogos (capturas dos testes, menus, telas-título) e nas caixas (por exemplo
  "S.RPG" no Super Dragon Force). O softlist do MAME [5] só tem notas sobre falhas de emulação. A
  menção aos cartuchos de amostra do Rebel Star vem do 百度百科 (A'can), https://baike.baidu.com/item/A'can
  (acesso em 2026-10-21). Formato copiado do Mil Capivaras (`docs/sheets.md`, commit `fecf86c`).

## 2026-10-21 — pesquisa sobre os jogos (gênero, jogadores, descrições)

Fatos usados nas descrições, gêneros e número de jogadores de `assets/games/` e `games.rs`, reescritos
com as nossas palavras (nenhum texto copiado). Acesso em 2026-10-21.

- **Wikipedia (en), Super A'Can**, https://en.wikipedia.org/wiki/Super_A%27Can — código, desenvolvedora e
  gênero de cada jogo; nomes alternativos (C.U.G., Sonic Dragon, African Adventures).
- **Wikipedia (zh), Super A'Can**, https://zh.wikipedia.org/wiki/Super_A%27Can — Formosa Duel parodia
  figuras públicas taiwanesas, com vozes em taiwanês (hokkien); Rebel Star só em cartuchos de amostra.
- **Wikipedia (en), Sango Fighter**, https://en.wikipedia.org/wiki/Sango_Fighter — original de 1993 para
  MS-DOS (Panda Entertainment); conversão do A'Can feita às pressas por um só funcionário.
- **LaunchBox Games DB**, https://gamesdb.launchbox-app.com/ (base comunitária, sem citações; tratada como
  "segundo o LaunchBox"): número de jogadores dos 12; Bomberman-like (Boom Zoo); estilo Puyo Puyo e doze
  adversários (Formosa Duel); mahjong taiwanês de 16 peças (Gambling Lord); fotos digitalizadas (Magical
  Pool); fazendas, pedreiras, hotéis e tesouro, até 6 jogadores (Monopoly); seis fases, taco, bolas,
  fase bônus em 3D (Speedy Dragon); estilo Shining Force e cidades vistas de cima (Super Dragon Force);
  times da CPBL licenciados, animações e torcida digitalizada (beisebol); escolhas que tornam o herói bom
  ou mau (The Son of Evil); modos arcade e versus (Sango Fighter).
- **PPE.pl, Historia konsol Funtech Super A'Can**, https://www.ppe.pl/publicystyka/202436/Historia-konsol-Funtech-Super-A-Can.html
  — Speedy Dragon como "Sonic taiwanês"; Journey to the Laugh como o jogo mais popular.
- **RF Generation, Funtech Super A'Can**, https://www.rfgeneration.com/features/system-overview/237/ —
  Journey to the Laugh entre os melhores jogos do console.
- Dos próprios jogos (capturas do V-Bear): o "21º esquadrão" e o "império Titã" do Rebel Star (diálogo e
  introdução); modos da sinuca e do beisebol (menus); temporada do beisebol salva na SRAM.
- Conflitos anotados: o gênero do Rebel Star é estratégia no LaunchBox e RPG nas Wikipédias (usado:
  estratégia, que bate com o que se vê no jogo); Formosa Duel "celebridades chinesas" (LaunchBox) ×
  "políticos taiwaneses" (WP-zh), resumido como "figuras públicas de Taiwan". Bloqueados: MobyGames,
  Bahamut, avid.wiki, Vogons (403/Cloudflare).
- Desenvolvedora × editora: as caixas (capas do libretro-thumbnails e a foto do Rebel Star) trazem "…製作 ◎
  敦煌發行" — feito por …, publicado pela 敦煌 (Funtech). O campo `publisher` do softlist do MAME [5] é, na
  verdade, a desenvolvedora. Desenvolvedoras conforme a lista da Wikipedia (en); Super Taiwanese Baseball
  League: "全崴製作" na caixa = Quan Wei Technology (a Wikipedia dá "Quan Wei Technology/C&E").

## 2026-10-21 — SRAM de bateria

- MAME `hash/supracan.xml` [5] (`dataarea name="nvram" size="0x8000"` em sonevil, staiwbbl, slghtsag,
  monopoly, gamblord, rebelst; gamblord com "TODO: unconfirmed size and actual presence") e
  `bus/supracan/rom.cpp` (`battery_load(..., 0xff)`: o MAME começa a SRAM com `0xFF`).
- MiSTer [6] `rtl/cart/cart_16003_1b.sv` (U2 62256, `A[15:1]`, só o byte baixo; "a save file is 32 KiB
  over a 64 KiB window"), `rtl/mem/save_slot.sv` e `SuperAcan.sv` (o `.sav`; a SRAM não é apagada ao
  trocar de cartucho).

## 2026-10-23 — homebrew

Procurados jogos homebrew, demos ou ROMs de teste livres para o Super A'Can (para `Brand::free_games`
e para testes): nenhum encontrado. Buscas na web (inglês; "homebrew", "SDK", "test rom", "prototype",
"supracan" + GitHub) só trazem a biblioteca comercial. O softlist do MAME [5] tem só os 12 jogos
comerciais; o repositório do MiSTer [6] não traz ROM de teste.
- Games That Weren't, "Quick Fighting Attack" e "City Escape" (março de 2026):
  https://www.gamesthatwerent.com/2026/03/quick-fighting-attack/ ,
  https://www.gamesthatwerent.com/2026/03/city-escape/ — jogos cancelados, só imagens (vídeo "Let's
  visit Funtech!" de Brandon Cobb), "No download available"; muitas ROMs da Funtech teriam sido
  destruídas.

## 2026-10-30 — o core MiSTer simulado

- **Verilator** 5.046 (2026-02-28, pacote do Fedora), https://www.veripool.org/verilator/ — simula o
  SystemVerilog do core MiSTer ciclo a ciclo (`reference/`). Precisou de `-Wno-BLKANDNBLK` para o `fx68k`.
- O README do core MiSTer [6], "Development and QA": como o core foi validado (vídeos de hardware real
  mostrando telas de demonstração, de Diskman, disassemblies, sem teste em console). O repositório tem um
  único commit, de 2026-09-11 (`59ba275`).
- **MAME 0.289** (Flatpak, o mesmo de 2026-10-21), como terceira fonte: capturas do Magical Pool perto do quadro
  300 (o círculo crescente da janela 0) com `tools/mame/drive.lua`.
- `sevenz-rust` 0.6 (Apache-2.0, https://crates.io/crates/sevenz-rust/0.6.1), já no workspace: lê os arquivos
  `.7z` (jogos com vários chips, BIOS dentro de um `.7z`).
