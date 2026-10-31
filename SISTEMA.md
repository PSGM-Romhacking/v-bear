# Ficha do sistema

Preenchida em 2026-10-21 a partir do núcleo do V-Bear (que já existia, em minifb) e de
`docs/hardware.md`. Cada valor diz de onde vem; `?` = não sabido.

## 1. Identidade
| Campo | Valor | Vira |
|---|---|---|
| Nome do emulador | V-Bear | `brand::Brand.name`, título da janela |
| Slug (minúsculas, dígitos e `-`) | `v-bear` | `v-bear-core`, `v-bear` (app), `v-bear-libretro`; pasta de config `v-bear` |
| Nome do sistema | Funtech Super A'Can | `SystemInfo.name` |
| Nome curto | Super A'Can | `SystemInfo.short_name` |
| Versão inicial | `0.1.0` | `[workspace.package] version` |
| Ícone (PNG 256) | `assets/icons/v-bear-256.png` (definitivo, 2026-10-23: urso-negro-de-formosa com o V no peito; também `.svg` e `.ico`) | `Brand.icon_png` |
| Repositório | ? (ainda não criado) | `Brand.repo_url` |
| Onde conseguir jogos legais/gratuitos (rótulo + URL) | ? (não se conhece homebrew) | `Brand.free_games` |

## 2. O núcleo
| Campo | Valor |
|---|---|
| Tipo | crate Rust próprio (`crates/v-bear-core`), escrito para o V-Bear |
| Caminho ou origem (pasta, repo, arquivo) | `crates/v-bear-core/src` (vindo de `~/Projetos/V-Bear/src`, versão minifb) |
| Licença do núcleo | GPL-3.0-or-later (era GPL-2.0-or-later; mudada em 2026-10-21 com o dono). Dependências: `m68k` (MIT), `mos6502` (BSD-3-Clause). Segue o comportamento do core MiSTer (GPL v2 ou posterior no topo; os arquivos dos chips só trazem o texto da GPLv2) e do MAME (BSD-3-Clause): ver `docs/references.md` |
| É só o `.so` pronto? | não |
| Precisa de BIOS ou arquivos de sistema? Quais? | sim, obrigatório: `internal_68k.bin` (8d575662), `internal_6502_1.bin` (fc9fb05f), `internal_6502_2.bin` (bf950ab7), `umc6650.bin` (0ba78597) |
| O que o conteúdo é: arquivo único, vários arquivos, disco? | cartucho; um arquivo, ou dois (Super Dragon Force: duas mask ROMs, juntadas a maior primeiro) |
| Já foi validado contra alguma referência (outro emulador, FPGA)? | MAME 0.289: imagem e som batem nas cenas comparáveis (`docs/compatibility.pt-BR.md`). Core MiSTer simulado com o Verilator (`reference/`): no quadro 300, sem botões, imagem idêntica em 10 dos 12 jogos (`docs/accuracy.pt-BR.md`); o core MiSTer é uma reconstrução por engenharia reversa, não um console. Console real: não |

## 3. Conteúdo (ROM)
| Campo | Valor | Vira |
|---|---|---|
| Extensões aceitas (sem ponto) | `can` (a principal, a que a ROM unida recebe ao ser salva), `bin` | `SystemInfo.extensions` |
| Tamanho mínimo/máximo da ROM | até 4 MiB, tamanho par, vetores do 68000 válidos (em qualquer ordem de bytes) | `Cartridge::from_bytes` |
| Banco de jogos (DAT, No-Intro...) | 12 jogos, CRC32 do ROM juntado; títulos/anos/editoras do softlist do MAME, nomes No-Intro | `crates/v-bear-core/src/games.rs` |

## 4. Vídeo
| Campo | Valor | Vira |
|---|---|---|
| Largura × altura visíveis (px) | 320 × 240 | `width`, `height` |
| Aspecto na tela | 4:3 | `aspect` |
| A resolução muda durante o jogo? | sim no hardware (H256/H320, 224/240 linhas); o núcleo estica H256 para 320 e recorta o modo de 224 linhas dentro dos 240, então a saída é fixa | — |
| Formato do pixel do núcleo | paleta xBGR555, convertida para XRGB8888 no próprio núcleo | `video.frame` |
| FPS exato | 53 693 175 / (3420 × 262) ≈ 59,923 | `fps_num` / `fps_den` |
| Existem modelos/regiões com FPS diferente? | não conhecidos (só Taiwan, NTSC) | — |

## 5. Áudio
| Campo | Valor | Vira |
|---|---|---|
| Taxa nativa (Hz) | 10 738 635 / 3 / 80 ≈ 44 744 | reamostragem linear em `Audio::resampled` |
| Canais (mono/estéreo) | estéreo | — |
| Canais que valem mutar no debugger | os 16 do sampler (ainda não ligados ao debugger) | `sound_channels` |

## 6. Controles
| Campo | Valor | Vira |
|---|---|---|
| Número de jogadores | 2 | `players` |
| Botões reais do controle | direcional, A, B, X, Y, L, R, Start, Select | `buttons` |
| Qual botão do RetroPad cada um usa | o mesmo nome (o controle é estilo SNES: A direita, B baixo, X cima, Y esquerda) | `PAD_MAP` em `core.rs` |
| Periféricos fora do RetroPad | não | — |

## 7. Memória
| Campo | Valor | Vira |
|---|---|---|
| RAM principal (tamanho) | 64 KiB de work RAM | `system_ram()` |
| Endereço da CPU do primeiro byte da RAM | `$FC0000` (espelhada até `$FFFFFF`) | `ram_address` |
| Tem mais de uma RAM, ou save de bateria? | sim: 64 KiB de RAM de som (65C02), 128 KiB de VRAM e 32 KiB de SRAM de bateria no cartucho (usada pelo Super Taiwanese Baseball League). Só a work RAM é exposta; a SRAM vai por `battery_ram()` | extensão do contrato (abaixo) |

## 8. Capas e interface
| Campo | Valor | Vira |
|---|---|---|
| Pasta no libretro-thumbnails | `Funtech - Super Acan` | `thumbnails` |
| Formato da caixa | retrato (~0,66) (não conferido com as caixas) | `cover_aspect` |
| Moldura (bezel) de TV ou console? | não (por enquanto) | `Brand.bezels` |
| Opções do núcleo | nenhuma | — |
| Idiomas além de en, pt, es | não | — |

## 9. Saída esperada
- [x] App desktop (`cargo run -p v-bear`)
- [x] Core libretro (`make libretro`)
- [x] Pacotes de release: AppImage (`packaging/build_linux.sh`); `.exe` ainda não
- [x] Outro: modo sem janela para testes (`v-bear JOGO --frames N`)

## 10. Observações

**Extensões do contrato feitas para este sistema** (aprovadas pelo dono em 2026-10-21; genéricas,
com padrão vazio, sem `if sistema ==`):

- `SystemInfo::bios: &[BiosFile]` (nome, CRC32, obrigatório) e `Core::set_bios`: a interface acha os
  arquivos por CRC32 na pasta BIOS (nova, em Pastas) e na pasta do jogo (só arquivos ≤ 1 MiB), soltos
  ou dentro de `.zip` ou `.7z`; o core libretro os procura na pasta *system* do RetroArch (`supracan/`, depois
  `superacan/` e `SuperAcan/`, depois a própria `system/`), nos mesmos formatos.
- `Core::battery_ram` / `battery_ram_mut` / `load_battery`: a interface grava `<jogo>.sav` na pasta
  "Saves de bateria" (nova) quando muda (verificado a cada 5 s, ao trocar e ao fechar o jogo); o
  core libretro expõe `RETRO_MEMORY_SAVE_RAM`.
- `SystemInfo::multi_file_roms`: um `.zip` ou `.7z` com vários arquivos é um jogo em vários chips, juntados
  do maior para o menor (o core libretro lê o arquivo ele mesmo, com `need_fullpath`, porque o RetroArch
  entregaria só um). Ferramentas › Juntar partes do jogo… (e `v-bear --join`) grava o jogo como um arquivo só.

Particularidades: o 65C02 só roda o driver de som que cada jogo carrega por DMA; o BIOS é
indispensável. Detalhes de hardware em `docs/hardware.md`.
