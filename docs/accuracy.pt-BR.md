# Precisão e validação

[English](accuracy.md) · **Português** · [Español](accuracy.es.md)

Como o V-Bear é comparado com outras implementações do Super A'Can, o resultado e o que ainda falta.
**O V-Bear não foi comparado com um console real**, e as referências com que foi comparado são, elas
mesmas, reconstruções: leia "Limites da referência" antes dos números.

As referências entre colchetes ([n]) estão em [Referências](references.pt-BR.md).

## Como a comparação é feita

**1. Os dois "consoles"**
- **A referência:** o core MiSTer do Super A'Can é a descrição do circuito da placa (em SystemVerilog). Um
  programa (Verilator) simula esse circuito no PC, ciclo de clock a ciclo de clock. É lento, mas é o
  próprio core MiSTer rodando.
- **V-Bear:** o emulador, escrito em Rust, rápido. Quando o MAME e o core MiSTer discordam, ele segue o
  core MiSTer ([Hardware](hardware.pt-BR.md)).

Se o V-Bear está certo, os dois têm de mostrar **a mesma coisa** no mesmo instante.

**2. O mesmo jogo**

Os dois rodam a mesma ROM com a mesma BIOS, sem ninguém jogando: nenhum botão é pressionado. Assim, o que
se compara é o início de cada jogo, até o quadro 300 (cerca de 5 segundos), não menus nem partidas.

**3. Pausa e captura no mesmo quadro**

Depois de N quadros, os dois param e salvam:
- **a imagem:** no tamanho real (256 ou 320 pontos de largura, 224 ou 240 linhas);
- **a work RAM:** os 64 KB que o 68000 usa.

**4. Comparar, ponto a ponto**

Um script compara os arquivos: quantos pixels têm cor diferente (0 = imagem idêntica) e quantos bytes da
RAM têm valor diferente.

**5. Os dois nunca iniciam na mesma velocidade**

O mesmo jogo chega à mesma imagem com alguns quadros de diferença (de 0 a 13 aqui, e varia de jogo para
jogo), porque a simulação lê o cartucho com um atraso fixo que não é o real. Por isso o quadro N da
simulação é comparado com uma janela de quadros do V-Bear, e se guarda o melhor: aquele com menos pixels
diferentes e, entre empatados, o com menos bytes de RAM diferentes. O deslocamento encontrado faz parte do
resultado (`LAG`). A consequência é que uma imagem parada prova pouco sobre o alinhamento, e é a RAM que
decide.

**6. Quando der diferente: olhar onde**

Uma diferença é medida, não adivinhada. Olha-se o retângulo que contém o que difere, os valores dos pixels
e, quando a causa não está clara, uma terceira fonte (MAME), antes de chamar de bug.

## Limites da referência

Esta é a parte que mais importa para saber como ler os números.

- **O core MiSTer é uma reconstrução, não o console.** O README dele ("Development and QA") diz que foi
  construído por engenharia reversa: vídeos de hardware real mostrando as telas de demonstração dos jogos
  (gravados por Diskman), desmontagens (disassemblies) de toda a biblioteca e documentação anterior. O autor
  não o testou contra um console: o console custa milhares de dólares e não existe flash cart. O mesmo
  README diz que a biblioteca roda "apparently bug free".
- **Portanto, "idêntico ao core MiSTer" significa que duas reconstruções independentes concordam.** Isso é
  indício de que as duas estão certas, não prova. Se concordam num erro, esta comparação não o enxerga. Se
  discordam, ela não diz qual está errada: foi preciso um terceiro lado (abaixo).
- **O repositório do core é novo:** um commit (2026-09-11, `59ba275`), sem histórico de correções.
- **A simulação é nossa e tem limites próprios:** o cartucho é lido com um atraso fixo (um palpite); nenhum
  botão é pressionado; não se compara som, save state nem save de bateria; roda a cerca de um quadro por
  segundo. Detalhes em [`reference/README.md`](../reference/README.pt-BR.md).

## Metodologia (detalhes técnicos)

1. **Referência:** o SystemVerilog do core MiSTer simulado ciclo a ciclo com o Verilator,
   em [`reference/`](../reference/README.pt-BR.md), sem alterar o código do core: `sim_top.sv` substitui o
   framework do MiSTer (clock, cartucho como um array, carga da BIOS) e `sim.h` captura a imagem da saída de
   vídeo do core e lê a work RAM pela porta de savestate dele.
2. **Mesma entrada:** nenhuma (nenhum botão pressionado), BIOS carregada dos mesmos quatro arquivos.
3. **Mesma saída:** a imagem e os 64 KB de work RAM no quadro N.
4. **Normalização da imagem:** o V-Bear sempre gera 320×240 (linhas do H256 esticadas; o modo de 224 linhas
   centralizado, com 8 linhas pretas no topo e embaixo) e a simulação gera o tamanho real; a comparação
   aplica a conversão de `video.rs` ao quadro da simulação e compara cores exatas (8 bits por canal).
5. **Alinhamento:** uma janela de ±12 quadros do V-Bear em torno do esperado, como descrito acima.
6. **Comandos:**

```bash
make -C reference
FRAMES=300 reference/compare.sh --jobs 6 --window 12 --cache /tmp/ref-cache target/release/v-bear roms/*.zip
```

7. **Terceira fonte:** MAME 0.289 (Flatpak `org.mamedev.MAME`), conduzido por `tools/mame/drive.lua`, com a
   captura de cada quadro. Foi usada para decidir a diferença da janela 0 abaixo.

## Resultado

No quadro 300, sem nenhum botão pressionado, 12 jogos, core MiSTer simulado (`LAG` é o deslocamento do
V-Bear em quadros, e não é confiável numa imagem parada):

| Jogo | LAG | Pixels diferentes | Bytes de RAM diferentes |
|---|---|---|---|
| Boom Zoo | 5 | **0** | 1 |
| Formosa Duel | 9 | **0** | 9 |
| Gambling Lord | 2 | 5.629 | 124 |
| Journey to the Laugh | 9 | **0** | 2 |
| Magical Pool | 3 | **0** | 451 |
| Monopoly: Adventure in Africa | 2 | **0** | 1 |
| Rebel Star | 0 | **0** | **0** |
| Sango Fighter | −2 | 10.643 | 52 |
| Speedy Dragon | 6 | **0** | 37 |
| Super Dragon Force | 16 | **0** | 868 |
| Super Taiwanese Baseball League | 2 | **0** | 13 |
| The Son of Evil | −4 | **0** | 6 |

- **Imagem idêntica, pixel a pixel e cor a cor, em 10 de 12 jogos.**
- **RAM:** idêntica no Rebel Star; no máximo 13 bytes diferentes de 65.536 em mais 6 (Boom Zoo, Formosa
  Duel, Journey, Monopoly, Baseball, The Son of Evil).
- **Também comparado, fora desta tabela:** a introdução da BIOS (o logo "A'can"), quadros 100 a 125 do Boom
  Zoo, é idêntica nos 26 quadros (0 pixels de 71.680 a 5 bits por canal de cor; conferido a 8 bits por
  canal só no quadro 120), com um deslocamento constante de 1 quadro.

**O que teve de ser corrigido para chegar aqui:**

| Problema | Onde | Efeito antes da correção |
|---|---|---|
| O recorte da janela usava a entrada L−1 da tabela para a linha L (`render.rs`, `window`) | vídeo | uma forma desenhada pela janela 0 (o círculo crescente do Magical Pool) uma linha abaixo do correto: 76 pixels num anel no quadro 300, 122 e 176 nos quadros 310 e 315. Depois da correção: 0 pixels nos quadros 290, 300 e 305 (3, 23 e 61 em 295, 310 e 315, não investigado) |

**Como o caso da janela foi decidido.** O V-Bear lia a entrada da tabela da linha anterior porque o código
do core MiSTer diz "the pair fetched during line L-1 clips line L". O `render_y` do core já é `vpos + 1`,
então o que ele busca durante a linha L−1 é a entrada L. O MAME indexa a tabela pela própria linha. Na
simulação o círculo ocupa as linhas 101 a 138 (centro 119,5) e o V-Bear o desenhava em 102 a 139 (centro
120,5); com o V-Bear deslocado uma linha para trás, a diferença passou de 76 pixels para 4. O círculo nas
capturas do MAME também está centrado na linha 119,5, nos quadros medidos (298 e 301). Duas referências
independentes contra o V-Bear.

## O que ainda é diferente

- **Sango Fighter (imagem).** A logo aparece em fade com os três canais de cor (R, G, B), cada um subindo
  um nível em um quadro diferente de um ciclo de 4 quadros. Medido nos quadros 290 a 305 (mesmo número de
  quadro dos dois lados, média de cada canal): na simulação os passos acontecem nos quadros 291 (G), 292 (R)
  e 294 (B), e a cada 4 quadros depois disso; no V-Bear, em 293 (G), 294 (R) e 292 (B). A cada quarto quadro
  (290, 294, 298, 302, 306, 310, 314) os três canais têm o mesmo valor dos dois lados e a imagem é
  **idêntica, 0 pixels, com lag 0**; nos outros três quadros de cada ciclo, de 9.000 a 12.600 pixels diferem,
  em 1 nível de um ou mais canais. Ou seja, os dois produzem o mesmo fade, e o que difere é **em que quadro do
  ciclo cada canal é atualizado**. A causa não foi encontrada: o próprio código do jogo, a fase de um timer ou
  de uma interrupção, ou o atraso fixo de leitura da cartucho na simulação poderiam produzi-la.
- **Gambling Lord (imagem).** Idêntica em 8 dos 26 quadros de 290 a 315 (292, 294, 298, 302, 306, 309, 310 e
  314) e de 1.900 a 5.600 pixels nos outros, com o mesmo tipo de diferença (1 ou 2 níveis de um canal de cor).
  Não analisada canal por canal.
- **Magical Pool, Speedy Dragon e Super Dragon Force (RAM).** 451, 37 e 868 bytes diferem com a imagem
  idêntica. Não investigado. O Super Dragon Force vem em dois arquivos de ROM (duas mask ROMs); uma rodada
  anterior da comparação usou só um deles (um erro do `compare.py`, corrigido), e o número dela foi
  descartado. Os 868 são com os 3 MB inteiros e janela de ±40 quadros (o melhor alinhamento, 16, está dentro).
- **Alguns bytes na RAM dos outros jogos** (1 a 13): no Boom Zoo no quadro 120, 9 bytes: 2 contadores com um
  passo de diferença e 7 bytes no topo da pilha do 68000. Provavelmente porque os dois lados param em pontos
  ligeiramente diferentes do quadro (não provado; a RAM foi comparada só em um ponto do quadro).
- **Quadros posteriores, menus e partidas.** Nada além do quadro 300 e sem botão pressionado foi comparado,
  exceto os testes contra o MAME em [Compatibilidade](compatibility.pt-BR.md).

## Classificação

O V-Bear **não** declara um nível na escala da Emulation General Wiki: não foi avaliado, e a referência com
que foi comparado não é o console. O que se pode dizer limita-se a "comparado com o core MiSTer simulado,
nos casos acima, e com o MAME 0.289 nas cenas listadas em [Compatibilidade](compatibility.pt-BR.md)".

## Validação contra o console

Nenhuma. Não há medições feitas num Super A'Can real, por nós ou por outros, nem ROMs de teste com fotos de
um console real. A validação do próprio core MiSTer (engenharia reversa a partir de vídeos) é o mais
próximo que existe, e está descrita acima.

## Status

| # | Item | Status |
|---|---|---|
| 1 | Janela 0 recortada pela entrada errada da tabela (uma forma uma linha abaixo) | **feito:** Magical Pool, 0 pixels nos quadros 290, 300 e 305 |
| 2 | Sango Fighter e Gambling Lord: os canais de cor de um fade são atualizados em quadros diferentes do ciclo | pendente: causa desconhecida; a imagem é idêntica a cada 4 quadros |
| 3 | A RAM difere no Magical Pool, Speedy Dragon e Super Dragon Force | pendente: não investigado |
| 4 | Comparar depois do quadro 300 e com botões pressionados | pendente: a simulação ainda não tem entrada |
| 5 | A simulação lê o cartucho com um atraso fixo (um palpite) | pendente: um atraso diferente mudaria o alinhamento, não o resultado |
| 6 | Comparar o som | pendente: a simulação não tem saída de som |
| 7 | Comparar com um console real | nenhuma mudança possível: não há nenhum para medir |

## Como cada parte foi validada

Cada parte da emulação, com a sua prova. Só o core MiSTer simulado e o MAME servem de referência; nenhum
dos dois é o console.

| Parte | Prova | Força |
|---|---|---|
| Imagem (tiles, sprites, fundos, prioridades, paleta) | 0 pixels diferentes contra o core MiSTer simulado em 10 de 12 jogos no quadro 300; intro da BIOS idêntica em 26 quadros | forte, nos primeiros 300 quadros |
| Janelas e tabelas por linha | círculo do Magical Pool: 0 pixels nos quadros 290, 300 e 305, decidido com a simulação e com o MAME | forte para a janela 0; as outras janelas não foram isoladas |
| 68000 e 65C02 (crates `m68k` e `mos6502`) | indireta: os jogos rodam e a RAM é idêntica no Rebel Star e difere em até 13 bytes em mais 6 | fraca: nenhuma suíte de testes de instruções foi rodada |
| Interrupções e temporização | mesma RAM e imagem da simulação; o fade do Sango Fighter e do Gambling Lord **não** é idêntico | parcial |
| Som | nenhuma contra referência: a simulação não tem saída de áudio | nenhuma |
| Controles | cenas jogadas contra o MAME 0.289, listadas em [Compatibilidade](compatibility.pt-BR.md) | parcial |
| Save de bateria | salvo e carregado à mão em 2 dos 6 jogos com bateria | parcial |
| Jogo em duas partes (Super Dragon Force) | arquivo unido com CRC32 `c4fe1ca1`, o que consta na base de jogos (`games.rs`) | forte |

## Cada item em detalhe

**2. Sango Fighter e Gambling Lord.** Para achar a causa, registrar as escritas do 68000 na paleta
(`$F00200` a `$F003FF`: quadro, linha, entrada, valor) no V-Bear e na simulação, e ver em que quadro cada
canal é escrito. Se o jogo os escreve no mesmo quadro nos dois lados e só a imagem difere, é uma questão de
renderização; se os escreve em quadros diferentes, a diferença está na temporização que conduz o jogo (uma
interrupção ou um timer), no V-Bear ou na simulação.

**3. A RAM.** A simulação para no fim de um quadro e o V-Bear no fim do seu; um jogo que mantém um contador
ou uma tabela que muda durante o quadro vai diferir em alguns bytes em qualquer alinhamento. Para separar
isso de uma diferença real, comparar a RAM em vários pontos do quadro e listar quais endereços diferem.

**4. Botões.** `--press-start` na comparação do Mil Capivaras pressiona Start em quadros fixos nos dois. A
simulação precisaria dos controles ligados em `sim_top.sv` (a entrada `pad` já existe) e de um roteiro de
pressionamentos em `sim.h`; o V-Bear já tem `V_BEAR_INPUT`.

## Ordem sugerida

O item 1 está feito. O que falta, do mais barato ao mais caro:

| # | Item | Esforço | Ganho |
|---|---|---|---|
| 3 | RAM no Magical Pool, Speedy Dragon e Super Dragon Force: comparar em vários pontos do quadro | pequeno | separa diferença real de efeito do ponto de parada |
| 2 | Sango Fighter e Gambling Lord: registrar as escritas na paleta dos dois lados | médio | acha (ou descarta) um erro de temporização no V-Bear |
| 4 | Botões na simulação | médio | estende a comparação além dos 300 primeiros quadros |
| 5 | Atraso do cartucho na simulação | pequeno | só muda o alinhamento |
| 6 | Som | grande | exige uma saída de áudio na simulação |
