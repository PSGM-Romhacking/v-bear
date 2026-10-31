# Super A'Can — hardware

[English](hardware.md) · **Português** · [Español](hardware.es.md)

Referência técnica para o V-Bear. As citações [n] apontam para `docs/references.pt-BR.md`.
Primeira versão: 2026-10-20 (MAME `60dbfc6`, MiSTer `59ba275`).

Legenda: **[C]** confirmado no código · **[D]** documentação/wiki · **[?]** incerto ou desconhecido ·
**⚠** MAME e MiSTer divergem.

**Regra de desempate do V-Bear:** quando MAME e MiSTer divergem, seguir o MiSTer (modela a placa
inteira e roda toda a biblioteca), registrando a divergência aqui.

---

## 1. Visão geral e história

- Console de 16 bits lançado só em Taiwan e na China em 25/10/1995, descontinuado em 04/04/1996;
  NT$2.900; codinome "F-16" [31].
- Funtech (Dunhuang Technology, subsidiária da UMC) em Taiwan e Sino Wealth na China. A UMC fez os
  chips UM6618 (vídeo) e UM6619 (sistema + som). Panda Entertainment foi a principal terceirizada [31][39].
- Fracassou pelo preço e pela concorrência 3D. Diz-se que a Funtech perdeu ~US$6 milhões e destruiu
  o equipamento de produção. CD-ROM e upgrade de CPU/gráficos planejados e nunca lançados [31].
- **12 jogos lançados, todos dumpados**; há inéditos conhecidos (Dinosaur War, City Escape, Quick
  Fighting Attack, Journey to the Center of the Earth…) [5][31].
- 2026: a Piko Interactive comprou IPs do console (só snippet de busca; página deu 403) [40][?].

## 2. Especificações

| Item | Valor | Fonte |
|---|---|---|
| Cristal U13 | 53,693175 MHz (MCLK); só vai ao UM6618, que gera o CPU_CLK [C] | [1][7][15][30] |
| CPU principal | MC68000P10. ⚠ **MiSTer: MCLK/5 = 10,738635 MHz** [C][10], igual à Wikipedia [31]. MAME: MCLK/6 = 8,949 MHz [C][1] | |
| CPU de som | 65C02 **embutido no UM6619** (não é chip separado). ⚠ **MiSTer: CPU_CLK/3 = 3,579545 MHz** [12] (bate com os "3,58 MHz" do Giant Bomb [39]). MAME: MCLK/12 ≈ 4,47 MHz, com comentário "/4 faz speedyd falhar" [1] | |
| Work RAM | 64 KiB: 2× 32K×8, um por byte lane [C][7][30] | |
| Sound RAM | U11, **64 KiB** como o 65C02 e o 68000 a enxergam: o MiSTer a instancia com 16 bits de endereço (`sram_async #(.ADDR_WIDTH(16))` em `mainboard.sv`) [7] e o MAME modela 64 KiB planos [1]. **Correção (2026-10-20):** a primeira versão deste documento dizia 32 KiB; os drivers de som ficam em `$E800-$FFFF` e os dados em `$8400+`, que colidiriam com 32 KiB | |
| VRAM | 128 KiB: 2× UM61512 64K×8 [7][30] | |
| Paleta/DAC | UM70C188: 256 entradas xBGR555 (32.768 cores), compatível com UM70C171 [27][1] | |
| Encoder | KA2195D, RGB→NTSC; saídas RF e composto [7][31] | |
| Resolução | 256 ou 320 × 240 (ou 224 com o bit de overscan); 262 linhas no total; ~59,92 Hz [17][1] | |
| Vídeo (UM6618) | 4 tilemaps, 1 camada ROZ, sprites, 2 janelas de clip. Mixagem em espaço de índice de paleta; translucidez = soma de índices [15][21] | |
| Sprites | Até 128 entradas no MiSTer (SPRITE_MAX), com escala e mosaico [20]. hw_docs: 32/linha e 320/quadro [29][?]; Wikipedia: 40/linha, até 256×256 px [31][?] | |
| Som (UM6619) | 16 canais PCM 8 bits sem sinal, estéreo, saída 44.744 Hz (clock de som /80) [2][13] | |
| Cartucho | Até 4 MB (2 mask ROMs de 16 Mbit) + SRAM de bateria 32 KiB + lockout UM6650 [24] | |

## 3. Mapas de memória

### 68000

MAME [1] e MiSTer [8][15] concordam, salvo onde indicado.

| Faixa | Conteúdo |
|---|---|
| `$000000-$3FFFFF` | ROM do cartucho. Os primeiros 4 KiB ficam sobrepostos pelo **IPL interno** até o bit 1 de `$E9001C` (latch de mão única) [1][11] |
| `$E80000-$E8FFFF` | Janela da sound RAM (bytes do 6502, big-endian). ⚠ No MAME, `$E80200/$E80202` leem os controles direto [1]; não existe no MiSTer |
| `$E90000-$E9001F` | Host port do UM6619 (seção 6) |
| `$E90020-$E9002F` / `$E90030-$E9003F` | Canais de DMA 0 e 1 |
| `$E90B3C` | "Easter egg": o IPL escreve 9 constantes (nome chinês da UMC) e lê uma vez [8] |
| `$EB0D01` (dados) / `$EB0D03` (endereço) | Lockout UM6650, byte baixo [3][24] |
| `$EC0000-$ECFFFF` | SRAM do cartucho, só no byte ímpar (32 KiB numa janela de 64 KiB) [24][4] |
| `$F00000-$F001FF` | Registradores do UM6618: 256 palavras, todas legíveis, **só acesso por palavra** (o chip não tem UDS/LDS) [16] |
| `$F00200-$F003FF` | Paleta: 256 × xBGR555 [1][15] |
| `$F40000-$F5FFFF` | VRAM 128 KiB |
| `$F80000-$FBFFFF` | Espelho de 256 KiB da ROM; primeiros 4 KiB sobrepostos pelo IPL até o bit 3 de `$E9001C` [1][11][24] |
| `$FC0000-$FFFFFF` | Work RAM 64 KiB, espelhada a cada 64 KiB [1][8] |

- Acessos do 68000 à janela da sound RAM (`$E80000`) passam pelo árbitro do UM6619, onde o 65C02 tem
  prioridade: são dois ciclos de byte e geram espera [8][12]. O V-Bear cobra 2 ciclos de CPU por acesso;
  com o 65C02 a CPU_CLK/3 exato, 1 ciclo já basta (o V-Bear usa 2 de margem); sem espera nenhuma, o Speedy Dragon perde a corrida entre o seu timeout (0xC000 voltas de 44 ciclos ≈ 0,201 s) e
  a inicialização do driver (≈ 0,207 s, com a detecção de controle por timeout) e recarrega o driver em laço.
- Sem bus error: `!BERR` amarrado em alto. Acesso sem resposta trava a máquina, por isso o UM6619
  responde a todo ciclo [8].
- Sango Fighter escreve 32 palavras além do fim da paleta [8].
- IACK por autovetor (`!VPA`) [8].

### 65C02

Sound RAM (64 KiB), com a página de I/O em `$0400-$04FF` decodificada nos 16 bits [1][12].

| Endereço | Função |
|---|---|
| `$0000-$3FFF` | No reset, recebe a cópia da **imagem interna de 16 KiB** (duas ROMs de 8 KiB): vetores + driver [1][12] |
| `$0402/$0403` | Shift registers dos pads 1 e 2 |
| `$0404/$0405` | Latches de bytes enviados pelo 68k. Leem `$CD` quando vazios; limpam IRQ bit 3/bit 2 [12] |
| `$0406` | Lê 0 (possível presença de pad) |
| `$0407` | Controle dos pads (borda de descida): bits 1-0 latch, 3-2 shift, 5-4 clear [1][12] |
| `$0409` | Ack do IRQ bit 4 |
| `$040A` | Escrita: IRQ6 no 68k (mailbox). Leitura: ack do IRQ bit 5, ativado pelo 68k em `$E9000A` [1][12]. ⚠ Comentário em `um6619_irq.sv` diz "$0300" [9]; o código usa `$040A` |
| `$040C/$040D` | Bytes que o 68k lê em `$E90004/5` (pedido de DMA de som) |
| `$0410` | Habilitação de IRQ |
| `$0411` | Status de IRQ. ⚠ MAME: leitura limpa tudo; MiSTer: não limpa nada [12] |
| `$0412` | Ack de NMI |
| `$0420` / `$0422` | Endereço / dado dos registradores do sampler; leitura de `$0420` = status [1][12] |

- Bits de IRQ do 6502: `$40` DMA de som, `$04`/`$08` latches, `$10`, `$20` pedido do 68k, `$80` timer de som [1].
- NMI vem do vblank, condicionada ao bit 7 de `$E90010` [1][12].
- hw_docs lista IRQs de mouse nas portas DE-9 [29][?].

## 4. Vídeo (UM6618)

### Registradores globais [1][16][29]

- `$00`: status. Bit 15 = vblank (MAME usa o bit 1 como quadro ímpar).
- `$02`: scanline. `$04`: ponto na linha (MiSTer implementa; Boom Zoo e Baseball leem).
- `$08`: flags de vídeo — bit 8 H320 · bit 9 overscan/224 linhas · bit 10 entrelaçado [?] ·
  bits 7-4 habilitação dos tilemaps 0-2 (e 3) [?] · bit 3 sprites · bit 2 ROZ · bit 1 janela 0 ·
  bit 0 janela 1 (MiSTer [22]).
- `$0A` / `$0C`: comparadores de raster "line on"/"line off" (bit 15 habilita, bits 7-0 linha) → IRQ5.
- `$10-$1E`: DMA de sprite — contagem, destino+incremento, origem+incremento, disparo no bit 15.
  Bit 8 = preenche com zero; bits 13/14 forçam destino na VRAM. MiSTer devolve os ponteiros finais [15].
- `$20`: base da tabela de sprites (<<2) · `$22`: quantidade − 1 · `$24`: cor mono ·
  `$26`: bit 0 = 8bpp (1) / 4bpp (0).
- `$1F0`: modo de pixel (bits 4-3) e modo GFX (bits 2-0), que define a profundidade das camadas 0 e 1.
  ⚠ hw_docs: 000 = 2bpp em todas; 001 = 4/4/2; 010 = 8/4; 011 = 4+linescroll; 100 = 8+linescroll [29].
  LUTs do MAME: `{2,1,0,1,0…}` [1]. No MiSTer as camadas 2 e 3 são sempre 2bpp [15][18].

### Tilemaps

Blocos de 8 palavras em `$100/$120/$140/$160` [1][18][29]. ⚠ MiSTer implementa os **4**; MAME só 3
(TODO em `$F00160`).

- `+0` flags: bits 15-13 prioridade · bits 11-8 tamanho do mapa (2 = 16×16, 4 = 32×32, 6 = 64×32,
  A = 128×32, C = 64×64 tiles) · bit 8 tiles 16×16 [?] · bit 6 **mistura aditiva** (MiSTer [18]) ·
  bit 5 wrap · bits 4-2 mosaico · bits 1/0 flip global X/Y (XOR no scroll).
- `+2` modo: bit 14 tabela de linescroll X · bit 11 tabela de line-select · bit 9 prioridade por tile ·
  bit 8 uma entrada por linha, senão a mesma para todas (MiSTer).
- `+4` / `+6` scroll X/Y. ⚠ MiSTer: X 11 bits com sinal (por causa do Magical Pool), Y 12 bits; MAME 12/12.
- `+8` base do mapa · `+A` modo/banco de tiles (bits 14-12) · `+C` tabela de linescroll ·
  `+E` tabela de line-select.
- Entrada de tile: bits 15-12 paleta · 11 flip X · 10 flip Y · 9-0 tile. Em 2bpp a paleta anda de 4 em 4.
- Tiles 8×8 lineares empacotados: 8bpp = 64 B, 4bpp = 32 B, 2bpp = 16 B [1][18].

### ROZ (`$180-$19E`) [19][1][29]

- `$180` modo: bits 1-0 profundidade (0 = logo 1bpp do BIOS, 1 = 2bpp, 2 = 4bpp, 3 = 8bpp ou bitmap
  linear via `$1F0`) · bit 5 wrap · bits 15-13 prioridade · bit 6 aditivo.
- `$182` controle: bit 15 tabela Y · bit 6 tabela de escala `$198` · bit 8 duas palavras X por linha.
- `$184-$18A`: origem X/Y, 24 bits com sinal, 8 bits fracionários.
- `$18C-$192`: coeficientes A/B/C/D em 8.8. MiSTer:
  `srcX = (scrollX + y·B + x·A) >> 8`, `srcY = (scrollY + y·D + x·C) >> 8`.
- `$194` base do mapa · `$196` banco (16 KiB por banco).
- Tabelas por linha: `$198`, `$19A`, `$19C` (lida e descartada), `$19E`. Zero na 1ª tabela desliga a linha.
- MAME trata essas tabelas como "unk_base", com HACK [1].

### Sprites (4 palavras por entrada) [1][20]

- W0: bits 12-9 altura em tiles (LUT `{1..10,12,16,20,22,24,26}`) · bits 8-0 Y.
  ⚠ MAME: bit 14 = enable. MiSTer: **bits 15-13 = escala vertical**.
- W1: bits 15-12 banco · 11 flip X · 10 flip Y · bits 9-8 máscara (0 normal, 1 mascarado/backup,
  2 só máscara, 3 máscara frontal) · bits 2-0 log2 da largura.
- W2: bits 10-9 prioridade · bits 8-0 X. ⚠ MAME: bits 15-13 escala X (não implementada).
  MiSTer: **bits 15-11 escala horizontal**, com divisor + Bresenham.
- W3: bit 15 "direct" (tile único na própria palavra), senão ponteiro para lista de tiles na VRAM.
  1×1 também é tratado como direct.

### Mixer [21][1]

- Prioridade menor fica na frente.
- MiSTer desempata por bits de "tie"; ROZ ganha de camada normal com a mesma prioridade.
- Sprites com blend somam só os pens, sem carry.

### Janelas (`$1D0` e `$1D8`) [22][29]

- Preenchem uma faixa horizontal com uma cor; bordas vêm de tabela na VRAM, um par por quadro ou
  por linha (bit 8). Bit 11 inverte · bits 7-0 cor · bits 14-13 prioridade.
- Efeito do scroll vertical da janela: desconhecido [?].
- Com um par por linha, a linha L usa a entrada L da tabela: o MiSTer a lê durante a linha L−1, mas o seu
  `render_y` já é a linha seguinte [22], e o MAME indexa pela própria linha [1]. O V-Bear usou L−1 até
  2026-10-30 e desenhava essas formas uma linha abaixo (veja [Precisão e validação](accuracy.pt-BR.md)).

### Timing [17]

- 3420 MCLK por linha nos dois modos, 2560 ativos. H256 = 342 pontos de MCLK/10; H320 = 420 pontos
  (70 a /8 e 30 a /10). ⚠ MAME usa htotal 455 a /8 no H320 (= 3640 MCLK/linha) [1].
- VBL na linha 240.
- Arbitragem da VRAM (MiSTer): 8 slots por ponto [23].

## 5. Áudio (sampler do UM6619) [2][13]

- 256 bytes de registradores atrás de `$0420/$0422`.
- `$11/$12`: período do timer, alterna a cada 5·(0x10000−n) clocks de som (MiSTer). ⚠ MAME: 10·(…) [2].
- `$14`: controle do timer (bit 7 inicia; bits 7-6 = 00 param). Ler faz ack e devolve a fase no bit 6.
- `$16`: ler faz ack do IRQ de DMA/stream. `$17`: key on/off (bits 7-4 ≠ 0 = on, bits 3-0 = voz).
- Por voz *v*:
  - `$2v/$3v` pitch — incremento = pitch<<6 num acumulador 16.16.
  - `$4v` controle — bit 1 habilita envelope (só MiSTer).
  - `$5v` comprimento = 0x40 << ((d>>1)&7); bit 0 = one-shot.
  - `$6v/$7v` início ÷ 0x40.
  - `$9v` ≠ 0 = stream por DMA; no fim gera IRQ e reinicia.
  - `$Av-$Dv` envelope (ataque, decay, sustain, release, nível). **MiSTer implementa; MAME só armazena.**
  - `$Ev` volume em nibbles E/D.
- Amostras: 8 bits sem sinal, centro 0x80. No MiSTer, stream termina num byte zero [13].
- MAME: `MACHINE_IMPERFECT_SOUND` (canais ruidosos, loops presos) [1][5]. MiSTer: envelope medido no
  console real (boot chime) [13].

## 6. DMA, IRQ, timers e controles

### Host port `$E90000-$E9001F` [1][8][9][10][11]

- `$04/$05`: lê `$040C/$040D` do 6502.
- `$0A`: escrita ativa o IRQ bit 5 no 6502.
- `$0C`: lê `$040A` e faz ack do IRQ6 (MiSTer).
- `$10`: máscara de IRQ — bit 7 VBL (IRQ7 + NMI do 6502), bit 4 linha visível (IRQ4).
- Os registradores do host port gravam a palavra inteira do barramento, sem olhar UDS/LDS [8]; como o
  68000 repete o byte nas duas metades, uma escrita de byte vale para os dois lados. Os jogos usam isso:
  o Sango Fighter grava `$B8` como byte em `$E90010` para ligar o IRQ4, cujo tratador copia as bordas da
  janela 0 de `$FC0070/$FC0072` para a VRAM linha a linha (a cortina das transições e da luta). Só os
  registradores de DMA (`$E90020-$3F`) respeitam as metades [14].
- `$14` controle do FRC · `$16` recarga · `$18` valor atual (MAME chama de "RNG"; leitura faz ack do IRQ3).
- `$1C`: bit 0 solta/segura o 6502 · bit 1 desliga overlay baixo · bit 3 desliga overlay alto ·
  bit 2 desconhecido.

### IRQs do 68k (autovetor) [9][29]

1 expansão · 2 cartucho · 3 FRC · 4 linha visível · 5 raster on/off · 6 mailbox do 6502 · 7 VBL.

### FRC ⚠

- MiSTer: `$A2xx`; modo 0 = 1 Hz, modo 1 = 1024·(n+1) ticks, modo F = **9040**·(n+1). `$A300` conta quadros [10].
- MAME: modo F = **8192**·período, com hacks por jogo [1].

### DMA do host (2 canais) [1][14]

- Registradores: origem, destino, contagem, controle. Disparo com `$8800`/`$A800`.
- Bit 12 palavra (senão repete a origem = preenchimento) · bit 8 recua o destino a cada 16 bytes
  (registros de sprite) · bits 10/9 decremento.
- MiSTer: **bus master real** (BR/BG/BGACK, ciclos de 4 clocks, CPU parada).
  MAME: instantâneo, com casos especiais para staiwbbl e rebelst.

### Controles [26][1]

- Pad serial de 16 bits no esquema do SNES, ativo em baixo, MSB primeiro.
- Bits: 15 A · 14 B · 13 Start · 12 Select · 11 Up · 10 Down · 9 Left · 8 Right · 7 X · 6 Y · 5 L ·
  4 R · 3-0 em 1.
- **Quem lê os controles é o 6502**, via `$0407/$0402`.
- Multitap existe no MiSTer [26]; o design real é desconhecido [?].

### Boot e BIOS

- O IPL do 68k (4 KiB) verifica o UM6650: chaves em `$20-$2F` comparadas com "UMC 1994 (C)" +
  checksums; testa RAM em `$40-$5F` e escreve `$FF` em `$09` [24][3]. Depois libera overlays e o 6502 [11].
- MiSTer tem `BIOS_BYPASS`, que pula o IPL [11].
- **O BIOS é indispensável para jogar:** todo jogo, depois da abertura, carrega o seu driver de som (por DMA,
  em `$E800-$FFFF` da sound RAM) e conversa com ele por `$0300` (fila de comandos; o driver responde `$FF`),
  `$E9000A` (pedido) e `$040A` (resposta/IRQ6). Quem lê os controles é esse driver, que escreve o estado dos
  pads em `$0200/$0202` para o 68000. Sem o BIOS, o V-Bear simula só o "pronto" em `$0300`.

| Arquivo | Tamanho | CRC32 | Observação |
|---|---|---|---|
| `internal_68k.bin` | 4 KiB | `8d575662` | palavras com bytes trocados |
| `internal_6502_1.bin` | 8 KiB | `fc9fb05f` | copiado para a sound RAM em `$0000` |
| `internal_6502_2.bin` | 8 KiB | `bf950ab7` | copiado para `$2000` |
| `umc6650.bin` | 16 B | `0ba78597` | chave do lockout |

MiSTer aceita também a concatenação dos quatro (20.496 B) como `boot.rom` [25]. Os BIOS têm
copyright da UMC/Funtech: o V-Bear não os distribui — carrega do usuário ou usa HLE/bypass.

### Formato das ROMs

- `.bin` cru, palavras de 16 bits. MAME lê em big-endian nativo [4]; MiSTer diz que "a maioria dos
  dumps tem as palavras com bytes trocados" e detecta pela tabela de vetores [24] → **autodetectar**.
- Mais de 2 MB usa a 2ª mask ROM (A21); ROMs menores se espelham [24].
- Saves de 32 KiB [24].

## 7. Jogos

MAME: atributo `supported` do softlist [5]. MiSTer: README [6].

| Série | ID MAME | Título | Ano / editora | Tamanho | SRAM | MAME |
|---|---|---|---|---|---|---|
| F001 | formduel | Formosa Duel | 1995 AV Artisan | 1 MB | – | **não** |
| F002 | sangofgt | Sango Fighter | 1995 Panda | 3 MB | – | parcial |
| F003 | sonevil | The Son of Evil | 1995 Funtech | 2 MB | 32K? | parcial |
| F004 | speedyd | Speedy Dragon ("Sonic Dragon") | 1995 AV Artisan | 2 MB | – | **não** |
| F005 | staiwbbl | Super Taiwanese Baseball League | 1995 C&E | 2 MB | 32K? | **não** (som) |
| F006 | jttlaugh | Journey to the Laugh (嘻遊記), provavelmente o "C.U.G." da Wikipedia [?] | 1995 Funtech | 2 MB | – | parcial |
| F007 | slghtsag | Super Light Saga / Dragon Force | 1996 Kingformation | 3 MB | 32K? | parcial |
| F008 | monopoly | Monopoly / African Adventure | 1995 Panda | 1 MB | 32K? | parcial |
| F009 | gamblord | Gambling Lord | 1996 Funtech | 2 MB | 32K? | parcial |
| F010 | magipool | Magical Pool | 1996 Funtech | 2 MB | – | parcial |
| F011 | boomzoo | Boom Zoo | 1996 Funtech | 512 KB | – | parcial |
| F012 | rebelst | Rebel Star (REBEL) | 1996 Horng Shen | 2 MB | 32K? | **não** |

- SRAM com "?" = "unconfirmed" no próprio softlist [5].
- MiSTer: roda "the entire game library (apparently) bug free"; feito a partir de vídeos de attract
  mode do hardware real (Diskman) e desmontagem de todos os jogos [6].
- Existem patches de tradução para inglês e pacotes de cheats [33].

## 8. Emuladores existentes e lacunas

- **MAME** (driver `supracan` desde a 0.269, 2024 [37]): `IMPERFECT_SOUND | IMPERFECT_GRAPHICS`;
  4 jogos sem suporte, 8 parciais [1][5]. Lacunas: só 3 tilemaps; ROZ e tabelas por linha com hacks;
  sem escala de sprite; prioridades aproximadas; sem envelope de som; clocks possivelmente errados;
  DMA instantâneo; FRC com hacks. Roda também no libretro-mame e no Batocera (pede `supracan.zip` e
  `umc6650.zip`) [32]. Relato de 2023: 5 de 11 jogos funcionando [38] (desatualizado).
- **MiSTer SuperACan_MiSTer**: commit único "Initial commit", 2026-09-11, Jamie Blanks (kitrinx) [6].
  Modela a placa PCGAM 16000-2A como netlist, com UM6618, UM6619, UM6650, UM70C188, KA2195D, além de
  savestates, cheats e multitap. **A referência mais completa hoje**; os comentários do RTL documentam
  o hardware melhor que qualquer outra fonte.
- **Outros:** nenhum emulador standalone encontrado.

**Desconhecido em todas as fontes:** datasheets da UMC (não existem, segundo o MiSTer [8][15]) ·
bit 2 de `$E9001C` · scroll Y das janelas · tabela `$19C` do ROZ · IRQs de expansão e cartucho ·
entrelaçado · formato real do multitap e do mouse.

### Onde o V-Bear segue cada um

Os ⚠ acima marcam os pontos em que o MAME e o MiSTer discordam. O V-Bear segue o MiSTer em todos (a regra de
desempate acima), exceto onde uma comparação mostrou o contrário:

| Ponto | MAME | MiSTer | V-Bear |
|---|---|---|---|
| Clock do 68000 | MCLK/6 | MCLK/5 | MiSTer |
| Clock do 65C02 | MCLK/12 | CPU_CLK/3 | MiSTer |
| Tilemaps | 3 | 4 | MiSTer (4) |
| Tabela de janelas, linha L | entrada L | par lido na linha L−1, cujo `render_y` já é `vpos + 1`, ou seja, a entrada L | entrada L (corrigido em 30/10/2026; ver [Acurácia](accuracy.pt-BR.md)) |
| `$0411` (status de IRQ) | a leitura limpa tudo | não limpa nada | MiSTer |
| Escala de sprites | não implementada | bits 15-13 da primeira palavra | MiSTer |

## 9. Licenças

- **MAME:** `supracan.cpp`, `umc6619_sound.cpp`, `umc6650.cpp` e `bus/supracan/*` são **BSD-3-Clause**
  [C][1][2][3][4] (copyright Angelo Salese, Ryan Holtz, superctr). Softlist `supracan.xml` é **CC0** [5].
- **MiSTer:** `LICENSE` do repositório é **GPL v2**; `SuperAcan.sv` diz "v2 or later" [28]. Os arquivos
  `um6618*`/`um6619*`/`board`/`cart` trazem só "Copyright (c) 2026 Jamie Blanks" → valem pelo LICENSE
  GPLv2 [?: sem SPDX por arquivo]. `rtl/fx68k` é GPL v3; `rtl/6502/mos6502*` é MIT [28].
- Consequência: traduzir a lógica do RTL para o V-Bear gera obra derivada **GPL**. Para outra licença,
  usar o RTL só como documentação de comportamento (clean-room) ou portar só o código do MAME.

## 10. NESdev

- Wiki: buscas por "Super A'Can", "Funtech" e "UM6619" sem resultado [36].
- Fórum: só menções de passagem, nada técnico:
  - 2011, etabeta (dev do MESS) cita o Super A'Can entre sistemas que o MESS "emula totalmente" [34] —
    **contradiz** o estado real (o MAME ainda marca o driver como imperfeito em 2026).
  - 2010, "Top 10 Failed Gaming Consoles": specs "comparáveis para 2D", difícil de programar [35].
- A busca do phpBB não indexa "A'Can" (o apóstrofo quebra o termo); buscados também "acan",
  "supracan", "super acan", "UM6619", "UMC 6650", "Funtech", "Sango Fighter".
- **Conclusão:** o NESdev não complementa nem corrige o MAME/MiSTer.
