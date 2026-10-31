# Arquitetura e como funciona

[English](architecture.md) · **Português** · [Español](architecture.es.md)

Como o código é organizado, como a emulação funciona e a razão por trás de cada decisão.
O V-Bear é um workspace do Cargo com o console separado da
interface; eles só se conhecem por meio de um contrato pequeno (`emu-api`).

## Estrutura

```
crates/api/             contrato core ↔ frontend (sem dependências)
crates/v-bear-core/   Funtech Super A'Can: máquina, memória, banco de jogos, testes
crates/frontend/        frontend reutilizável (egui, menus, biblioteca, controles,
                        save states, rewind, gravação, capas, idiomas, configuração)
crates/v-bear/        executável: core + frontend + identidade visual (nome, ícone, Sobre)
crates/v-bear-libretro/   v_bear_libretro.so = core + API libretro
crates/libretro-host/, libretro-player/, toy*/   da base: um host para cores libretro prontos
                        e um exemplo de brinquedo; o V-Bear não os usa
assets/                 ícones, fontes, bezels, i18n/<código>.json, games/
docs/                   a documentação (lista no README)
```

- `v-bear-core` → só `emu-api` (+ as bibliotecas de que a emulação precisa).
- `emu-frontend` → só `emu-api`: não conhece nenhum console.
- `crates/v-bear` junta os dois; o core libretro usa o core e o `emu-api`,
  nunca o frontend.

Dependências do core (`crates/v-bear-core/Cargo.toml`): `m68k` 0.14 (MIT, o 68000, com sua feature
`serde`) e `mos6502` 0.10 (BSD-3-Clause, o 65C02), ambos do crates.io, sem nada vendorizado ou bifurcado;
`serde`, `serde_json` e `bincode` para os save states; `zip` para ler uma ROM ou a BIOS de um `.zip`.
As licenças dos crates estão listadas no README.

## Contrato (`emu-api`), seguindo as convenções do libretro

| Aspecto | Decisão |
|---|---|
| Controles | bitmask **RetroPad** por jogador (B, Y, Select, Start, ↑ ↓ ← →, A, X, L, R, L2, R2, L3, R3); o sistema declara quais usa e o nome de cada um |
| Vídeo | `&[u32]` **XRGB8888** + largura/altura/proporção de tela por quadro |
| Áudio | **estéreo intercalado f32**, taxa escolhida pelo frontend |
| Save state | bytes opacos (`retro_serialize`); o cabeçalho do arquivo (magic, hash da ROM, miniatura) pertence ao frontend |
| Memória | `system_ram()` (`RETRO_MEMORY_SYSTEM_RAM`) |
| Metadados | `SystemInfo`: nome, extensões, nome no libretro-thumbnails, formato da capa, botões, FPS, resolução, identificação por hash |
| Taxa de quadros | `Core::fps()` (padrão: a de `SystemInfo`) |
| Opções | `Core::options()` / `set_option()`, como as *core options* do libretro |
| Depuração | `debug_views()`: imagens (com grade opcional) e tabelas em grupos |

O que o Super A'Can faz com cada linha:

| Aspecto | No V-Bear |
|---|---|
| Controles | 2 jogadores. O controle é estilo SNES: Cima, Baixo, Esquerda, Direita, A, B, X, Y, L, R, Select e Start; cada bit do RetroPad vai para o bit do controle de mesmo nome (`PAD_MAP` em `core.rs`); L2, R2, L3 e R3 não são usados |
| Vídeo | sempre 320 × 240; os modos de 256 pontos são esticados até a mesma largura, como uma TV os mostra, e o modo de 224 linhas apaga 8 linhas no topo e na base. Os pixels são o xBGR555 da paleta expandido para XRGB8888; proporção 4:3 |
| Áudio | o fluxo estéreo de 44.744,3 Hz do sampler, multiplicado por 4 (deixa muita folga) e reamostrado para a taxa do frontend |
| Estado | um cabeçalho de arquivo (`VBEARST`, versão do formato, o CRC32 da ROM) seguido do estado completo da máquina; ~267 KB. A ROM, a BIOS e a SRAM da bateria ficam de fora |
| RAM | os 64 KB de work RAM, vistos a partir de `$FC0000`, para os cheats e o depurador |
| Bateria | `battery_ram()`: a SRAM de 32 KB do cartucho (`.sav`) |
| Opções | nenhuma |
| FPS | MCLK / (3420 × 262) = 53.693.175 / 896.040 ≈ 59,92 Hz (`FPS_NUM`/`FPS_DEN` em `core.rs`): o cristal de 53,693175 MHz, 3.420 clocks mestres por linha e 262 linhas |

## O crate do core

| Arquivo | Conteúdo |
|---|---|
| `lib.rs` | os `pub mod`s e `pub use core::{INFO, …Core}` |
| `core.rs` | o **adaptador**: `static INFO: SystemInfo` e `impl emu_api::Core` (RetroPad → entrada nativa, imagem → XRGB8888, áudio → estéreo f32, estado ↔ bytes) |
| `console.rs` | a máquina: liga as duas CPUs, o barramento, o vídeo, o som e os controles, e os executa linha a linha (`run_frame`); também `snapshot`/`restore` e os arquivos de save state e de SRAM |
| `bus.rs` | o barramento do 68000: decodificação de endereços, a porta host do UM6619, interrupções, o contador livre e os dois canais de DMA do host |
| `bios.rs` | os quatro arquivos da BIOS, identificados por CRC32 |
| `cart.rs` | o cartucho: detecção da ordem dos bytes, `.zip` com várias mask ROMs, a SRAM |
| `video.rs`, `render.rs` | o UM6618: registradores, paleta, VRAM e o renderizador (tilemaps, ROZ, janelas, sprites, mixer) |
| `sound.rs` | o lado de som do UM6619: o 65C02, sua RAM e página de E/S, as portas dos controles, as caixas de correio com o 68000 e o sampler de 16 canais |
| `audio.rs` | reamostra a saída do sampler para a taxa do dispositivo |
| `input.rs` | o registrador de deslocamento de 16 bits do controle |
| `state.rs` | o formato do arquivo de save state |
| `debug.rs` | chaves de depuração lidas uma vez de `V_BEAR_DEBUG` (`snd`, `t65`, `dma`, `irq`...), para o modo de teste sem janela |
| `games.rs` | banco de jogos (identificação por CRC32) e os textos dos jogos |

## O frontend

O `emu-frontend` é compartilhado como está. Cada arquivo em `app/` é um bloco `impl App` (laço de emulação,
entrada, navegação por controle, menus, tela, biblioteca, save states, gravação, controles,
diálogos). Ele lê tudo sobre o sistema de `SystemInfo`; as únicas partes específicas do sistema
são o que o app repassa: `brand::set(Brand { name, id, version, icon_png, about,
repo_url, bezels, free_games, thumbnails_url })` e os textos de idioma do próprio app
(`i18n::add_catalog`).

## Testes

`cargo test --workspace` roda tudo o que não precisa de jogo:

- `games.rs`: todo jogo tem seus textos em todos os idiomas, e um jogo é encontrado em qualquer ordem de bytes;
- `sound.rs`: o save state do 65C02 preserva o estado de espera e a borda do NMI, o controle é lido começando pelo bit
  mais significativo, a caixa de correio e os latches, e uma voz de sampler em loop;
- `state.rs`: o arquivo de save state faz o ciclo de ida e volta e rejeita outro jogo, outra versão de formato e lixo;
- `crates/v-bear/src/main.rs`: os textos dos jogos são válidos (verificado pelo frontend);
- `crates/v-bear-libretro`: a API C do libretro corresponde ao core e os estados fazem o ciclo de ida e volta.

`crates/v-bear-core/tests/contract.rs` executa um jogo real pelo contrato `emu-api` (900 quadros:
uma imagem na maior parte do tempo, a quantidade certa de som, e um save state que torna os 120 quadros seguintes
idênticos). Ele precisa de arquivos que não estão no repositório, então é ignorado a menos que `V_BEAR_TEST_ROM` (um
`.zip` de jogo) e `V_BEAR_TEST_BIOS` (uma pasta com os quatro arquivos da BIOS) estejam definidos.

A comparação com outras implementações não faz parte do `cargo test`: veja
[Precisão e validação](accuracy.pt-BR.md) e [Compatibilidade](compatibility.pt-BR.md).

## Como a emulação funciona

**Base de tempo.** O clock mestre é o cristal de 53.693.175 Hz (`MCLK`, `console.rs`). Uma linha tem 3.420
clocks mestres e um quadro 262 linhas, o que dá 59,92 quadros por segundo. O 68000 roda a MCLK / 5 (684
ciclos por linha) e o 65C02 a esse clock dividido por 3.

**Um quadro** (`Console::run_frame`). Para cada uma das 262 linhas:

1. `start_line` dispara as interrupções da linha: vertical blank na linha 240 (e o NMI da CPU de som, se
   habilitado), a interrupção de linha nas linhas 1 a 239 e a comparação de raster "line on" / "line off";
2. o UM6618 monta a próxima linha em um buffer de linha enquanto esta é exibida (no core do MiSTer,
   `render_y = vpos + 1`), então `render_line(next)` roda agora: a linha L enxerga os registradores e a VRAM como estão
   quando a linha L−1 começa;
3. a linha é executada em 4 fatias, alternando o 68000 (`run_cpu`, 171 ciclos) e o 65C02 com o sampler (57 ciclos deles):
   depois de cada lote de instruções, a posição horizontal é derivada dos ciclos usados.

As instruções nunca são divididas: os ciclos executados além do fim de uma fatia (`overshoot`) são descontados da
seguinte. Um DMA mantém o 68000 fora do barramento (`stall`) enquanto o contador continua rodando.

**CPU.** O 68000 é o crate `m68k`, executado em lotes de ciclos com um hook que atualiza o nível de interrupção
após cada instrução; instruções ilegais, A-line e F-line são contadas em `faults`, que é como uma falha
aparece nos testes. O 65C02 é o crate `mos6502` (`Mos65C02`), dentro de `sound.rs`, com a página de E/S do UM6619
decodificada ao redor dele.

**Vídeo** (`video.rs`, `render.rs`). Cada uma das 240 linhas visíveis é montada a partir das camadas na
ordem do mixer: quatro tilemaps, a camada ROZ, a janela 0 e a janela 1, e depois os sprites (até 128 entradas na
tabela, com escala, mosaico e máscaras). O mixer trabalha no espaço de índices da paleta; a paleta tem 256 entradas de
xBGR555. A largura é de 256 ou 320 pontos (`$F00008`, bit 8), e o bit 9 apaga 8 linhas no topo e na
base. Os detalhes do chip estão em [Hardware](hardware.pt-BR.md).

**Som.** O sampler de 16 canais produz uma amostra estéreo a cada 80 ciclos da CPU de som
(44.744,3 Hz). O `audio.rs` a multiplica por 4 e a reamostra para a taxa que o frontend pede.

**Save states.** `Console::snapshot` copia a máquina entre quadros: o 68000 passa pela sua forma `serde`
(JSON), o resto (`Bus`, que também guarda a CPU de som) por `bincode`. A ROM, a BIOS e a
SRAM da bateria não são salvas, então um estado carregado depois não desfaz um save feito no jogo. Um estado é exato:
carregá-lo e executar 120 quadros dá a mesma imagem de tê-los executado desde o início (verificado por
`tests/contract.rs`).

## A razão por trás de cada decisão

| Decisão | Por quê, e o que custa |
|---|---|
| Quando o MAME e o core do MiSTer discordam, seguir o core do MiSTer | ele modela a placa inteira e roda a biblioteca inteira; as divergências estão registradas em [Hardware](hardware.pt-BR.md). Custo: o core do MiSTer é ele próprio uma reconstrução, com um commit e sem teste em um console real ([Precisão](accuracy.pt-BR.md)) |
| O 68000 e o 65C02 são crates, não foram escritos aqui | crates existentes, MIT e BSD-3-Clause. Custo: o V-Bear depende de como eles contam ciclos; o 68000 não é `Clone`, então o estado passa pela sua forma `serde` |
| A máquina roda linha a linha, em 4 fatias por linha | suficiente para os efeitos que os jogos usam (interrupções de raster, scroll por linha e tabelas de janela) e barato. Custo: não é cycle-accurate dentro de uma linha, e a posição horizontal é derivada, não simulada |
| O renderizador monta uma linha inteira de uma vez, uma linha adiantado | é o que o core do MiSTer faz (`render_y = vpos + 1`). Custo: a linha L enxerga os registradores e a VRAM como estavam quando a linha L−1 começou |
| Saída fixa de 320 × 240 | o frontend trabalha com um tamanho fixo por sistema; as linhas de 256 pontos são esticadas, como uma TV as mostra, e o modo de 224 linhas é exibido com barras pretas. Custo: o tamanho real da imagem não fica visível; a comparação com o core do MiSTer o converte de volta |
| Proporção 4:3 | uma televisão; a proporção de pixel do console não foi medida |
| A BIOS é obrigatória e identificada por CRC32 | a boot ROM do 68000, as duas ROMs da CPU de som e a chave de lockout são necessárias para inicializar; o nome do arquivo não importa. Custo: o usuário precisa encontrar os quatro arquivos, que não são distribuídos |
| Os jogos são identificados por CRC32 em um banco de 12 jogos | o frontend mostra a capa e a ficha do jogo pelo conteúdo. Custo: uma ROM que não está no banco (um homebrew, um hack) não tem ficha |
| Os save states usam um cabeçalho com o CRC32 da ROM | um estado de outro jogo é recusado. Custo: o formato tem versão; mudar as estruturas salvas torna os estados antigos ilegíveis |
| Tudo de que a interface precisa está embutido | ícone, fontes, traduções e o banco de jogos estão no executável; não há nada a instalar. Custo: um binário maior |
