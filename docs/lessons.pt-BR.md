# Lições aprendidas

[English](lessons.md) · **Português** · [Español](lessons.es.md)

O que o projeto ensinou, para quem for fazer o mesmo em outro sistema. Cada lição vem de algo que
aconteceu aqui; os números estão em [Precisão e validação](accuracy.pt-BR.md) e
[Compatibilidade](compatibility.pt-BR.md).

## Abordagem

- **Tenha duas implementações para comparar, não uma.** O V-Bear foi escrito a partir do MiSTer core e do
  MAME. Nenhum dos dois é o console: o MiSTer core também é uma reconstrução (um commit, nenhum teste em um
  console real). Quando eles discordam, é preciso uma terceira opinião. O bug da janela (abaixo) só foi
  decidido porque o MAME concordou com o MiSTer core contra o V-Bear.
- **Um comentário no código-fonte é uma afirmação, não uma medição.** O V-Bear recortava cada linha com a
  entrada da tabela de janelas da linha *anterior* porque o MiSTer core diz "o par buscado durante a linha
  L-1 recorta a linha L". O que a frase deixa de dizer é que o `render_y` do core já é `vpos + 1`, então o
  par que ele busca é a entrada L. A imagem estava errada por uma linha até a comparação mostrar isso.
- **"Idêntico à referência" nunca significa "idêntico ao console".** Diga qual referência, quais quadros e
  quais jogos, e mantenha o que não foi comparado na lista do que falta.

## Comparação com um core FPGA simulado

- **Simule o Verilog do core com o Verilator e compare imagem e RAM.** É a única forma, aqui, de rodar o
  hardware do MiSTer sem um MiSTer. O custo é a velocidade: cerca de um quadro por segundo para este
  console, então 300 quadros levam cinco minutos por jogo.
- **Não compile com `--public-flat-rw`.** Ele faz o Verilator expor todos os sinais de um core com dezenas
  de milhares deles, e o compilador C++ gasta mais de 25 minutos em um único arquivo gerado. Marque apenas o
  que você lê com um comentário `public_flat_rw`, e leia o resto por portas que o core já tem (a work RAM
  aqui é lida pela porta de savestate).
- **Pequenos detalhes de build que custam uma tarde:** o pacote do 6502 tem que vir primeiro na lista de
  arquivos; `-Wno-BLKANDNBLK` para o core do 68000 (`fx68k`); e o `fx68k` lê dois arquivos `.mem` de
  microcódigo do diretório *atual*, então a simulação tem que rodar ao lado deles, senão o 68000 não
  funciona, com apenas um aviso na tela.
- **Duas implementações nunca iniciam na mesma velocidade.** O mesmo jogo chega à mesma imagem com alguns
  quadros de diferença (de 0 a 13 aqui, e varia por jogo), porque a leitura do cartucho leva um tempo
  diferente. Compare o quadro N de uma com uma janela de quadros da outra, e informe a defasagem.
- **Um empate não é um resultado.** Numa imagem parada, muitos quadros dão 0 pixels diferentes; pegar o
  primeiro comparou a RAM no quadro errado (15.000 bytes diferentes em um jogo). Desempate pelo quadro mais
  próximo do esperado.
- **Veja o que difere antes de decidir que é um bug.** Dois grupos de diferenças não foram
  atribuídos a um erro: um fade cujos três canais de cor sobem em quadros diferentes de um ciclo de 4 quadros
  (a imagem é idêntica a cada quarto quadro; a causa não foi encontrada), e alguns bytes da pilha e dos contadores do 68000,
  provavelmente porque os dois lados param em pontos ligeiramente diferentes do quadro (não provado). Uma
  diferença era real: uma forma recortada pela janela do chip de vídeo, desenhada uma linha abaixo do correto.
- **Confira se a referência rodou o mesmo jogo.** A primeira comparação deu à simulação só um dos dois
  arquivos do Super Dragon Force (um jogo em dois chips de ROM; o zip lista primeiro o chip de 1 MB), então o
  resultado de RAM dela (8.682 bytes) não significava nada: com as partes unidas deu 868. Quando um número
  parecer estranho, confira a entrada antes de culpar qualquer emulador.
- **Meça onde uma forma está, não só quantos pixels diferem.** O bug da janela parecia ruído (76 pixels em
  um anel); a caixa delimitadora do círculo mostrou que ele estava exatamente uma linha abaixo, e a contagem
  caiu de 76 para 4 quando foi deslocado de volta.

## Interface e RetroArch

- **Identifique a BIOS pelo CRC32, não pelo nome.** Os mesmos quatro arquivos vêm do MAME, do MiSTer e de
  dumps com muitos nomes, e os sets do MAME não contêm todos (a chave `umc6650.bin` está no próprio set).
- **O RetroArch guarda em cache as informações do core.** Depois de um novo `.info`, o core aparecia como
  `v_bear_libretro`, sem descrição nem licença, e o navegador de arquivos escondia `.bin` e `.can`, até o
  cache ser limpo. O sintoma parece um bug do core e não é.
- **Teste o core libretro primeiro sem o RetroArch.** Um programa pequeno que faz o papel do RetroArch pela
  API C (busca da BIOS, quadros, save state, RAM de bateria) acha as falhas óbvias em segundos; o que sobra é
  o que só o RetroArch mostra, como o cache acima.
- **Uma base de frontend economiza meses,** mas as necessidades próprias do sistema (uma BIOS obrigatória em
  vários arquivos, saves de bateria, jogos em dois arquivos) vão além do contrato da base e precisam ser
  acrescentadas com cuidado.

## Modelo de hardware

- **Siga a placa, não o manual.** Quando o MAME e o MiSTer core discordam sobre clocks ou temporização,
  vence o core que modela a placa inteira e roda a biblioteca inteira, e a discordância é registrada
  ([Hardware](hardware.pt-BR.md)).
- **Dê tempo ao 68000 na janela da RAM de som.** O Speedy Dragon perde uma corrida entre um timeout e a
  inicialização do driver de som se o acesso não custar nada, e recarrega o driver para sempre.
