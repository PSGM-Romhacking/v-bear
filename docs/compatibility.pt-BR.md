# Compatibilidade

[English](compatibility.md) · **Português** · [Español](compatibility.es.md)

Estado de cada jogo no V-Bear, com o BIOS. Atualizar a cada rodada de testes, sem apagar as
anteriores (o histórico fica nas seções por data).

ROMs: as 12 do set No-Intro, conferidas por CRC32 com o softlist do MAME (ver `references.pt-BR.md`).

## Como testar

Tudo sem janela, com `--frames N` (o BIOS é achado em `bios/`):

```sh
# capturas a cada 450 quadros, com um roteiro de botões (quadro:botão, segura 8 quadros)
V_BEAR_INPUT="1500:start,1560:a" V_BEAR_SHOT=/tmp/g V_BEAR_SHOT_EVERY=450 \
  ./target/release/v-bear "roms - Funtech - Super Acan/<jogo>.zip" --frames 7200
# som em WAV (44 744 Hz, estéreo)
V_BEAR_WAV=/tmp/jogo.wav ./target/release/v-bear <rom> --frames 1800
```

O resumo final mostra IRQs atendidos por nível, paleta/VRAM em uso, pico de áudio, bytes de SRAM,
`faults` (exceções ilegais, A-line, F-line do 68000) e `halted` (CPU parada por falha dupla).
Outras chaves: `V_BEAR_REGS`, `V_BEAR_VRAM=<palavra>`, `V_BEAR_SND=<end>`, `V_BEAR_LIVE=<quadro>`
(rastreia o 68000), `V_BEAR_PCS`, `V_BEAR_DEBUG=snd,t65,dma,irq,watch,vregs,sndr,pal`,
`V_BEAR_SNAPTEST=<quadro>` (savestate determinístico), `V_BEAR_RESET=<quadro>` (reinicia ali),
`V_BEAR_RAM=<arquivo>` (grava os 64 KB da RAM de trabalho no fim, para comparar com `reference/`).
`V_BEAR_SHOT` sem `_EVERY` grava só a última tela, em PNG se o nome terminar em `.png`.

## 2026-10-21

Dois roteiros: um fixo (Start/A/direções a cada 60 quadros, 7200 quadros) e um aleatório
(semente 7, um botão a cada 12 quadros, 20 000 quadros ≈ 5,5 min de jogo). Nos 12 jogos:
`faults=0`, `halted=false`.

| Jogo | Até onde chegou | Observações |
|---|---|---|
| Boom Zoo | partidas do quebra-cabeça contra vários adversários | — |
| Formosa Duel | título, partida | — |
| Gambling Lord | seleção de personagem, partidas de mahjong | — |
| Journey to the Laugh | quatro fases de plataforma, chefe aranha | — |
| Magical Pool | título, menu de modos, mapa | o roteiro aleatório não sai do título (precisa de Start no momento certo) |
| Monopoly – Adventure in Africa | tabuleiro, dias avançando, compra e venda | — |
| Rebel Star (En) | introdução, título, mapa com diálogo | — |
| Sango Fighter | seleção, lutas em dois cenários | corrigido nesta data: a luta ficava coberta pela janela 0 (IRQ4 não ligava) |
| Speedy Dragon | introdução, várias fases | corrigido em 2026-10-20: corrida driver × timeout (esperas na RAM de som) |
| Super Dragon Force | título, masmorra, mapa do mundo | — |
| Super Taiwanese Baseball League | menus, partida com placar | usa a SRAM de bateria (1849 bytes gravados) |
| The Son of Evil | título, Start leva à cena do lago | — |

**Não verificado ainda:** fidelidade do som (só conferido que há música e efeitos), jogos até o
fim, modo de 2 jogadores, comparação lado a lado com MiSTer/console real.

## 2026-10-21 (2ª rodada)

Depois de corrigir a velocidade do 65C02 (rodava ~3,5% rápido): mesmo roteiro aleatório de
20 000 quadros, `faults=0` e `halted=false` nos 12. Novidades vistas: Magical Pool chega à mesa
de sinuca; Gambling Lord à mesa de mahjong com os quatro jogadores. Speedy Dragon segue bem: com a
velocidade certa basta 1 ciclo de espera na RAM de som (o V-Bear usa 2; com 0 o driver é
recarregado sem fim). Savestate (F5/F9) testado: restaurar e rodar 600 quadros dá tela idêntica
nos 12 jogos.

## 2026-10-21 — comparação com o MAME 0.289

Ferramenta: `tools/mame/compare.sh <jogo.zip>` (fora do git; ver `references.pt-BR.md`). Captura os
mesmos quadros nos dois emuladores, monta as imagens lado a lado e conta pixels diferentes (o MAME
mostra 224 linhas; o V-Bear tem 8 a mais em cima e embaixo).

- **Imagem:** nas cenas em que os dois estão no mesmo ponto, as telas batem. Speedy Dragon, título
  no quadro 1800: 99,2% dos pixels iguais, e a diferença é só o quadro da animação do personagem.
  Formosa Duel, título: 95,6%, a diferença é a posição das nuvens. Logos (Funtech, Panda, AV
  artisan), títulos de Boom Zoo, Gambling Lord, Monopoly, Super Dragon Force, Journey to the Laugh e o
  estádio do Super Taiwanese Baseball conferidos a olho, iguais.
- **Som:** Speedy Dragon, 6 s de música do título em espectrograma: mesmas notas, timbres e andamento
  (o padrão se repete a cada 1,6 s nos dois); volume da mesma ordem. A música começa ~0,5 s antes no
  V-Bear.
- **Sequência:** diverge, porque o MAME avança sozinho por títulos e menus sem entrada nenhuma (as
  portas dele ficam em repouso; causa não investigada, ver `references.pt-BR.md`). Exemplos sem entrada:
  no MAME o Sango Fighter corta a introdução no quadro ~1100 e entra numa luta; o Formosa Duel vai
  para a escolha de adversários; o Rebel Star e o The Son of Evil ficam com a tela preta ou com
  lixo. No V-Bear os jogos ficam no título e nas demonstrações, alternando, sem ninguém jogar.

## 2026-10-21 — savestate em disco

`V_BEAR_SNAPTEST=900 … --frames 2400`: grava o estado no formato do arquivo no quadro 900, roda até
2400, carrega numa máquina nova, roda de novo e compara a imagem e o estado inteiro (byte a byte).
Idêntico nos 12 jogos; cada estado tem ~260 KiB. Teste de unidade cobre o 65C02 parado em WAI e a
borda do NMI. As teclas F5/F6/F7/F9 na janela ainda não foram testadas à mão.

## 2026-10-21 — teclas de uso

Reset (F1) conferido sem janela: Sango Fighter reiniciado no quadro 600 e rodado mais 1800 dá a
mesma tela (pixel a pixel) e os mesmos contadores de IRQ que 1800 quadros desde o boot. PNG da tela
idêntico à captura PPM. Pausa, avanço de quadro, Tab e F12 na janela ainda não testados à mão.

## 2026-10-21 — migração para a interface da base

O núcleo foi para `crates/v-bear-core` sem mudar o comportamento: com o mesmo roteiro de botões,
a tela no quadro 2400 é idêntica, byte a byte, à da versão minifb nos 12 jogos. Pelo contrato
(`cargo test -p v-bear-core --test contract`, com `V_BEAR_TEST_ROM` e `V_BEAR_TEST_BIOS`): Super
Dragon Force (duas partes), Sango Fighter e Rebel Star identificados no banco, com imagem, áudio na
taxa pedida e save state exato. Core libretro: carrega o Sango Fighter com a BIOS na pasta
*system* (teste `real_core` do `libretro-host`). Interface: capturas automáticas (`EMU_DOCSHOTS`)
com o Sango Fighter rodando, ficha do jogo e as pastas novas (BIOS, saves de bateria). Ainda não
testado à mão na interface: controles, rewind, gravação, save de bateria.

## 2026-10-21 — save de bateria (SRAM), versão 0.5.0

**Hardware:** uma SRAM 62256 de 32 KiB (U2) na placa PCCAR 16003-1B, em `$EC0000-$ECFFFF`, só na metade
baixa do barramento: o byte `n` fica em `$EC0000 + 2n + 1`. O MiSTer solda a SRAM em todos os
cartuchos; o softlist do MAME declara `nvram` de 32 KiB em 6 jogos (alguns com "tamanho e presença não
confirmados"). O `.sav` do V-Bear tem 32 768 bytes nessa ordem, a mesma do `nvram` do MAME e do `.sav`
do MiSTer.

**Acessos medidos** (contadores `reads`/`writes` no resumo do modo sem janela, 3600 quadros com Start/A):

| Jogo | SRAM no MAME | Leituras | Escritas | Observação |
|---|---|---|---|---|
| Super Taiwanese Baseball League | sim | 29 968 | 2 162 | formata ao ligar; com o `.sav` gravado, lê 10 bytes e não reformata |
| Super Dragon Force | sim | 24 240 | 0 | procura saves já no título |
| Rebel Star | sim | 1 (ao escolher LOAD GAME) | 0 | sem save, volta ao título |
| The Son of Evil | sim | 0 | 0 | salvar e carregar confirmados à mão em 2026-10-30 |
| Monopoly | sim | 0 | 0 | LOAD GAME abre 5 saves e começa um jogo sem tocar em `$EC` nem em endereço fora do mapa (`V_BEAR_DEBUG=unmapped`); onde ele guarda o progresso é uma pergunta aberta |
| Gambling Lord | sim (presença não confirmada) | 0 | 0 | |
| os outros 6 | não | 0 | 0 | |

**Ida e volta (beisebol):** `V_BEAR_SAV=x.sav` no modo sem janela lê o arquivo antes e grava depois.
Primeira execução: lê 10 bytes, formata (2 162 escritas), grava `x.sav`. Segunda: lê os mesmos 10
bytes, 0 escritas, a mesma tela no quadro 3000.

**Corrigido:** a interface procurava o `.sav` só na pasta de saves (`~/.config/v-bear/saves/`); o que a
versão antiga deixou ao lado da ROM era ignorado. Agora, sem o arquivo na pasta de saves, ela usa o
`<jogo>.sav` da pasta do jogo e o copia para a pasta de saves na próxima gravação (conferido: cópia
idêntica).

**Para os testadores** (o que só se confere jogando):
1. Salvar e carregar em cada jogo com SRAM: Son of Evil, Super Dragon Force e Rebel Star (pontos de
   salvar), Monopoly (5 saves), Gambling Lord, beisebol (temporada).
2. Fechar o V-Bear logo depois de salvar e conferir que o save continua (o `.sav` é gravado até ~5 s
   depois da mudança, ao fechar o jogo e ao sair).
3. Trazer um `.sav` do MiSTer ou o `nvram` do MAME e ver se o jogo reconhece.

## 2026-10-30 — core do RetroArch e saves

Core compilado para Linux (glibc 2.27), Windows, FreeBSD e macOS (só compilou; FreeBSD e macOS não
foram executados). Testado o `.so` do Linux por um programa que simula o RetroArch (ctypes):

- BIOS achada solta em `system/`, em `system/supracan/`, num `.zip` de 4 arquivos e no `supracan.zip`
  do MAME com o `umc6650.bin` solto ao lado. O `supracan.zip` do MAME 0.289 sozinho tem só 3 arquivos
  (sem a chave `umc6650.bin`), então não basta. Segundo um relato no Reddit (não verificado aqui), o
  zip mais antigo trazia `internal_68k.bin` e `umc6650.bin`, e o mais novo trocou o segundo pelos
  dois `internal_6502`: nos dois casos faltam arquivos para o V-Bear.
- Save state: serializa 267 KB e restaurar dá o mesmo quadro nos jogos testados (Boom Zoo, beisebol).
- Save de bateria: o core entrega 32 KB; no beisebol o jogo grava 1849 bytes e a SRAM volta idêntica
  numa segunda sessão. Son of Evil, Super Dragon Force, Rebel Star, Monopoly e Gambling Lord ficam
  com a SRAM vazia em 3000 quadros sem botões (só gravam ao salvar de verdade).
- Confirmado à mão: **The Son of Evil** salva e carrega. Super Dragon Force, Rebel Star, Monopoly
  e Gambling Lord seguem por testar.
- Os 10 `.bin` conferidos têm o CRC32 do No-Intro.

## 2026-10-30 — comparação com o core MiSTer simulado

Os 12 jogos foram comparados com o core MiSTer do Super A'Can, simulado com o Verilator (pasta
`reference/`), no quadro 300, sem apertar botão nenhum, com BIOS igual nos dois lados. Método, números e
limites em [accuracy.pt-BR.md](accuracy.pt-BR.md).

- **Imagem idêntica, pixel a pixel e cor a cor, em 10 dos 12 jogos.** Gambling Lord e Sango Fighter
  diferem em alguns quadros: os canais de cor de um fade são atualizados em quadros diferentes de um ciclo de 4 quadros, e a imagem é idêntica a cada quarto quadro (causa não encontrada).
- **Achado e correção:** a janela 0 usava a entrada `L−1` da tabela para recortar a linha `L`, e o círculo
  crescente do Magical Pool saía uma linha abaixo. O MiSTer (`render_y = vpos + 1`) e o MAME 0.289
  concordam entre si contra o V-Bear; corrigido em `render.rs`. Antes: 76 pixels diferentes no quadro 300;
  depois: 0.
- **O core MiSTer não é o console.** Foi feito por engenharia reversa (vídeos de hardware real, disassemblies),
  sem teste em console, e o repositório tem um único commit. "Idêntico ao MiSTer" é a concordância de duas
  reconstruções, não uma prova.
- **RAM:** idêntica no Rebel Star, com no máximo 13 bytes diferentes em mais 6 jogos; Magical Pool,
  Speedy Dragon e Super Dragon Force têm diferenças ainda não investigadas.
