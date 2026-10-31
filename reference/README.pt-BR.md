# Referência: o core MiSTer simulado com o Verilator

[English](README.md) · **Português** · [Español](README.es.md)

A ferramenta que compara o V-Bear com uma segunda implementação do mesmo hardware. Ela simula, ciclo
a ciclo, o (System)Verilog do
[core MiSTer do Super A'Can](https://github.com/MiSTer-devel/SuperACan_MiSTer), roda um jogo e
salva a imagem e a work RAM, para que possam ser comparadas com o que o V-Bear produz no mesmo quadro.
O método e os resultados estão em [Precisão e validação](../docs/accuracy.pt-BR.md).

Não faz parte do emulador: nada aqui é compilado pelo `cargo build`.

## Antes de usar: o que esta referência é e o que não é

**É uma segunda reconstrução, não o console.** O core MiSTer foi escrito a partir de engenharia reversa
(o README dele, "Development and QA"): vídeos de hardware real mostrando as telas de demonstração dos
jogos, gravados por Diskman, disassemblies de toda a biblioteca e documentação anterior. O autor dele não
o testou contra um console real: o console custa milhares de dólares e não existe flash cart. O próprio
README dele diz que a biblioteca roda "apparently bug free" (aparentemente sem bugs).

O que decorre disso para quem lê os resultados:

- **Idêntico ao core MiSTer não é idêntico ao console.** Duas reconstruções concordarem é evidência de
  que ambas estão certas, não prova. Se concordarem num erro, a comparação não o vê.
- **Uma discordância não diz quem está errado.** Uma diferença pode estar no V-Bear, no core MiSTer ou
  nesta simulação. Foi preciso um terceiro lado (o MAME 0.289) para resolver o caso da janela 0.
- **O repositório do core é novo:** um único commit (2026-09-11, `59ba275`), sem histórico de correções
  que mostrasse o que já foi encontrado e corrigido.

Limitações da simulação em si (nossas, não do core):

- **O cartucho é lido com um atraso fixo** (`CART_LATENCY`, 12 ciclos de `clk_sys` em `sim_top.sv`). Num
  MiSTer real a ROM vem da SDRAM, com atraso variável. É um palpite, e muda o tempo de boot de cada
  jogo: por isso os dois lados não estão no mesmo quadro (veja "Alinhamento" abaixo).
- **Ainda sem entrada:** a simulação não aperta botões. Só o início de cada jogo é comparado (a abertura
  da BIOS e o começo do jogo), não menus nem partidas.
- **Lenta:** cerca de 1 quadro por segundo (medido: 120 quadros em cerca de 2 min 20 s), então 300
  quadros levam ~5 min por jogo e 3.000 quadros ~50 min.
- **Sem som, sem save de bateria, sem save state.** Apenas a imagem e os 64 KB de work RAM.
- **A RAM só faz sentido no mesmo ponto do quadro.** Os dois lados param em pontos ligeiramente
  diferentes do quadro e deixam sobras diferentes no topo da pilha do 68000 e em alguns contadores;
  isso não é um erro de emulação.

## Uso

Requisitos: Verilator 5 e um compilador C++17; Python 3. O core MiSTer não está neste repositório: o
`Makefile` o procura em `_referencias/mister-59ba275` (uma cópia local, fora do git, registrada em
[Referências](../docs/references.pt-BR.md)); `make MISTER=/other/path` aponta para outra cópia.

```bash
make -C reference                          # builds reference/acan-ref (~10 s)
cargo build --release                      # builds V-Bear
FRAMES=300 reference/compare.sh target/release/v-bear "roms/Game.zip" ...
```

`compare.sh` roda cada jogo nos dois e imprime, por jogo e quadro: o **lag** (a defasagem em quadros
que foi encontrada), os **pixels** diferentes e os **bytes de RAM** diferentes (0 = idêntico). Opções:
`--jobs N` (jogos em paralelo; cada simulação usa um núcleo), `--bios DIR` (a pasta com os quatro
arquivos, padrão `bios/`), `--window W` (quantos quadros em volta procurar, padrão 4) e `--offset K`.
Quadros por variável: `FRAMES="300 600"`.

A referência sozinha:

```bash
reference/acan-ref GAME.bin --bios bios --shot 300 out.ppm --dump-ram ram.bin
reference/acan-ref GAME.bin --bios bios --frames-out 290 315 prefix   # several frames as PPM
```

O V-Bear grava o equivalente com `V_BEAR_SHOT=<file>` e `V_BEAR_RAM=<file>` (modo sem janela,
`--frames N`).

## Alinhamento

A simulação e o V-Bear não chegam ao mesmo quadro ao mesmo tempo. Na abertura da BIOS a defasagem é de 1
quadro, constante. No início de cada jogo ela varia de 0 a cerca de 13 quadros, dependendo do jogo.
Assim, a comparação procura, numa janela de quadros do V-Bear, aquele que melhor corresponde ao quadro N
da simulação, e imprime a defasagem (`LAG`). Numa imagem parada muitos quadros dão 0 pixels e o `LAG`
não é confiável; numa animação é.

Dois formatos de imagem diferentes são normalizados antes de comparar: o V-Bear sempre gera 320×240
(linhas H256 esticadas; o modo de 224 linhas centralizado, com 8 linhas pretas em cima e embaixo), e a
simulação gera o tamanho real (256×224, 320×224, 320×240...). A comparação aplica a mesma conversão de
`crates/v-bear-core/src/video.rs` e compara cores exatas (8 bits por canal).

## O que esperar

A tabela de resultados e a lista de achados estão em [Precisão e validação](../docs/accuracy.pt-BR.md).
Aparecem dois padrões de diferença que não foram atribuídos a um erro de emulação (são
interpretações, não provas):

- **Um fade cujos canais de cor sobem em quadros diferentes** (Sango Fighter, Gambling Lord): a imagem é
  idêntica a cada quarto quadro e, nos quadros entre eles, uma região difere em 1 nível de um ou mais
  canais (123 contra 132), porque o V-Bear e a simulação atualizam os três canais em quadros diferentes do
  ciclo de 4 quadros. A causa não foi encontrada ([Precisão](../docs/accuracy.pt-BR.md)).
- **Sobras da pilha e contadores na RAM** (poucos bytes): os dois lados param em pontos ligeiramente
  diferentes do quadro. No Boom Zoo, 9 bytes: 2 contadores com um passo de diferença e 7 bytes no topo da
  pilha. Isto também é uma hipótese: a RAM só foi comparada no mesmo quadro, não em vários pontos do
  quadro.

## Arquivos

| Arquivo | O que é |
|---|---|
| `sim_top.sv` | o topo da simulação: o console do core MiSTer sem o framework MiSTer; a ROM do cartucho é um array, lido pelo mesmo protocolo request/busy/done da ponte SDRAM |
| `sim.h` | a simulação: clock, carga da BIOS e da ROM, captura de vídeo, o laço de quadros, leitura da RAM |
| `ref_main.cpp` | a linha de comando do `acan-ref` |
| `Makefile` | compila com o Verilator e copia `microrom.mem` e `nanorom.mem` (o `fx68k` os lê do diretório atual) |
| `compare.py`, `compare.sh` | rodam a referência e o V-Bear e comparam |

Como o Verilator trata o core: o código do core é lido sem alterações. Para compilá-lo foram necessários
`-Wno-BLKANDNBLK` (uma limitação conhecida do `fx68k` no Verilator) e a ordem dos arquivos
(`mos6502_pkg.sv` primeiro), e `--public-flat-rw` **não** deve ser usado (ele expõe todos os sinais e
faz o GCC gastar mais de 25 minutos num único arquivo; o array da ROM é público por um comentário no
próprio `sim_top.sv`, e a RAM é lida pela porta `ss_wram` do core).

## Licenças

| Parte | Licença |
|---|---|
| O core MiSTer (`_referencias/mister-59ba275`, não incluído aqui) | GPL-2.0 (o arquivo `LICENSE` dele) |
| O resto desta pasta | GPL-2.0 ou posterior (como no Mil Capivaras: é compilado junto com o core GPL-2.0, e o V-Bear é GPL-3.0) |

A referência é um programa separado: não é ligada ao emulador.
