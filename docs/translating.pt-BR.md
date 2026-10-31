# Traduzindo o V-Bear

[English](translating.md) · **Português** · [Español](translating.es.md)

O V-Bear existe em inglês, português e espanhol. (O inglês é o texto-fonte; português e espanhol já vêm embutidos.) Qualquer outro idioma pode ser adicionado
**sem programar e sem compilar nada**: a tradução é um único arquivo JSON
que o emulador lê ao iniciar. Você pode guardá-lo só para você ou enviá-lo para ser
incluído na próxima versão, para que todos o tenham.

## Conteúdo

1. [O que você precisa](#1-o-que-você-precisa)
2. [Passo a passo](#2-passo-a-passo)
3. [O formato do arquivo](#3-o-formato-do-arquivo)
4. [Regras que mantêm a interface funcionando](#4-regras-que-mantêm-a-interface-funcionando)
5. [Testando sua tradução](#5-testando-sua-tradução)
6. [Corrigindo um idioma existente](#6-corrigindo-um-idioma-existente)
7. [Quando uma nova versão acrescenta textos](#7-quando-uma-nova-versão-acrescenta-textos)
8. [Enviando para inclusão](#8-enviando-para-inclusão)
9. [Fichas dos jogos (opcional)](#9-fichas-dos-jogos-opcional)
10. [Dúvidas](#10-dúvidas)

## 1. O que você precisa

- V-Bear **0.5.0 ou mais novo**.
- Um editor de texto que salve em **UTF-8** (qualquer um moderno: VS Code, Notepad++, Kate, gedit,
  micro…). Evite processadores de texto como Word ou LibreOffice Writer.
- Cerca de duas a quatro horas: são algumas centenas de textos, a maioria curtos (itens de menu,
  botões).

## 2. Passo a passo

1. **Exporte o modelo.** No emulador, abra **Ajuda › Language / Idioma › Exportar modelo de tradução…** e salve o arquivo. Ele já é oferecido na pasta de idiomas.
2. **Dê ao arquivo o nome do código do idioma**, em minúsculas:
   `fr.json` (francês), `de.json` (alemão), `it.json` (italiano), `ja.json` (japonês),
   `pt-PT.json` (português europeu), `zh-CN.json` (chinês simplificado)… O nome é o que
   diz ao emulador qual é o idioma. Use apenas letras, dígitos e `-`.
3. **Traduza.** Abra o arquivo no editor de texto e troque cada valor (o texto à
   direita) pela sua tradução. Mantenha os textos da esquerda como estão. Detalhes nas duas
   próximas seções.
4. **Escreva o nome do idioma** em `"@name"`, do jeito que seus falantes o escrevem: `"Français"`,
   `"Deutsch"`, `"日本語"`. É isso que aparece no menu de idiomas.
5. **Coloque o arquivo na pasta de idiomas** e reinicie o emulador. Para achar a pasta:
   **Emulação › Pastas…**, linha **Idiomas**, botão **Abrir**. Por padrão ela é:

   | Sistema | Pasta de idiomas |
   |---|---|
   | Linux, FreeBSD | `~/.config/v-bear/languages/` |
   | Windows | `%APPDATA%\v-bear\languages\` |
   | Modo portátil | `v-bear_data/languages/`, ao lado do programa |

6. **Escolha-o** em **Ajuda › Language / Idioma**. Seu idioma aparece depois dos
   embutidos.

Isso é tudo para usá-lo você mesmo. Para compartilhar, veja a [seção 8](#8-enviando-para-inclusão).

## 3. O formato do arquivo

O modelo é uma lista de pares `"texto original": "tradução"`. Recém-exportado, cada
tradução é uma cópia do texto em inglês:

```json
{
  "@name": "Language name",
  "Open game…": "Open game…",
  "Search {n} games…": "Search {n} games…",
  "Screenshot saved: {file}": "Screenshot saved: {file}"
}
```

Traduzido para o francês, fica assim:

```json
{
  "@name": "Français",
  "Open game…": "Ouvrir un jeu…",
  "Search {n} games…": "Rechercher parmi {n} jeux…",
  "Screenshot saved: {file}": "Capture enregistrée : {file}"
}
```

- **O lado esquerdo é a chave**: o texto em inglês exatamente como está no programa. Nunca
  o altere, ou o emulador não achará sua tradução e mostrará o inglês.
- **O lado direito é o que aparece.** Pode ficar como está (idêntico ao inglês) ou
  **esvaziado** (`""`), o que significa "ainda não traduzido": o inglês é mostrado no lugar. Assim, um
  arquivo incompleto já funciona, e você pode traduzir em várias sessões.
- **A ordem das linhas não importa**, e linhas podem ser removidas (mostram o inglês).

## 4. Regras que mantêm a interface funcionando

1. **Mantenha os campos entre chaves** exatamente como estão: `{n}`, `{name}`, `{file}`,
   `{path}`, `{error}`, `{system}`… O programa os substitui por um número, um nome ou um
   caminho. Eles podem mudar de lugar dentro da frase (`"{n} jeux"`), mas não ser traduzidos nem removidos.
2. **Mantenha o "…"** no fim dos itens de menu que abrem uma janela (`"Folders…"` →
   `"Dossiers…"`). É uma convenção: "…" significa "abre algo onde você vai escolher".
3. **Mantenha o "›"** nos caminhos de menu (`Emulação › Configurar controles e atalhos…`) e traduza os
   nomes dos dois lados do mesmo jeito que você traduziu esses menus, para que as pessoas consigam acompanhar.
4. **Mantenha o JSON válido:** cada linha termina com vírgula, exceto a última; aspas dentro de um
   texto se escrevem `\"`; uma quebra de linha dentro de um texto é `\n`. Se o arquivo tiver um erro, o
   emulador mostra uma mensagem ao iniciar com o nome do arquivo e o motivo, e ignora esse
   arquivo (os outros continuam carregando).
5. **Textos curtos continuam curtos.** Itens de menu e botões têm espaço limitado. Se a
   tradução for bem mais longa que o inglês, procure uma palavra mais curta.
6. **Não traduza** nomes de coisas: V-Bear, Funtech Super A'Can, RetroArch, libretro,
   RetroAchievements, os formatos de arquivo (`.bin`, `.cht`, PNG, MP4,
   GIF), nomes de teclas (`Ctrl`, `Esc`, `F2`) e os nomes de pasta `Named_Boxarts`,
   `Named_Titles` e `Named_Snaps`.
7. **Termos técnicos:** use o que os jogadores do seu idioma realmente dizem. "Save state",
   "ROM", "cheat" e "rewind" costumam ficar em inglês; na dúvida, veja como
   o RetroArch é traduzido para o seu idioma e siga-o.
8. **Os rótulos de teclado e controle** (`KEYBOARD`, `CONTROLLER`, `EMULATOR`…) são cabeçalhos
   de coluna mostrados em maiúsculas: mantenha-os em maiúsculas.

## 5. Testando sua tradução

Depois de reiniciar com seu idioma escolhido, percorra estas telas; juntas elas mostram
quase todos os textos:

- [ ] todos os menus da barra de menus, incluindo os submenus;
- [ ] a biblioteca (busca, as abas de imagem, clique com o botão direito em um jogo, a tela de boas-vindas quando
      a pasta está vazia);
- [ ] um jogo em execução: as mensagens no alto (salvar estado, captura de tela, avanço rápido);
- [ ] **Emulação › Configurar controles e atalhos…** e **Ajuda › Controles**;
- [ ] **Emulação › Pastas…**;
- [ ] **Ferramentas › Depurador…** e **Ferramentas › Trapaças…** (as duas abas);
- [ ] **Ajuda › Ficha do jogo…** e **Ajuda › Sobre**;
- [ ] o menu rápido (botão Home/Guia de um controle, ou **Vídeo › Modo sofá** e Esc).

Procure palavras cortadas, textos que transbordam do botão e inglês deixado para trás (uma chave que
foi alterada por acidente, ou um valor ainda vazio).

## 6. Corrigindo um idioma existente

Para corrigir alguns textos de um idioma embutido (português ou espanhol), você não precisa do
modelo inteiro: crie `pt.json` ou `es.json` na pasta de idiomas com **apenas as linhas que você
quer mudar**. Seus textos tomam o lugar dos embutidos; todo o resto fica.
`"@name"` é ignorado nos idiomas embutidos. Se a correção serve para todos, envie-a também
([seção 8](#8-enviando-para-inclusão)).

## 7. Quando uma nova versão acrescenta textos

As novas versões acrescentam e alteram textos. Para atualizar uma tradução:

1. Exporte o modelo de novo, a partir da nova versão.
2. Compare-o com o seu arquivo. Uma ferramenta de comparação de texto ajuda: a visão "comparar arquivos" do
   VS Code, Meld, WinMerge ou `diff`. As linhas que existem só no modelo novo são os
   textos novos.
3. Copie as linhas novas para o seu arquivo e traduza-as. As linhas que existem só no seu arquivo
   são textos que o programa não usa mais: são inofensivas, mas podem ser apagadas.

Até você fazer isso, os textos novos simplesmente aparecem em inglês.

## 8. Enviando para inclusão

As traduções enviadas são adicionadas à próxima versão, embutidas no programa, com crédito ao
tradutor no README e nas notas da versão.

**Como enviar**, em ordem de preferência:

1. **Uma issue no GitHub:** abra
   <https://github.com/PSGM-Romhacking/v-bear/issues/new>, com o título
   `Translation: <idioma>` (por exemplo `Translation: Français`), e anexe o arquivo `.json`
   (se o GitHub recusar o anexo, compacte-o antes como `.zip`). Diga como quer
   ser creditado: nome ou apelido e, opcionalmente, um link.
2. **Um pull request**, se você usa Git: coloque o arquivo em `translations/<código>.json` e abra
   o pull request. O mantenedor o divide nos catálogos do programa.

**Antes de enviar**, confira:

- [ ] o nome do arquivo é o código do idioma (`fr.json`);
- [ ] `"@name"` está preenchido com o nome do próprio idioma;
- [ ] o emulador o carrega sem mensagem de erro, e as telas da
      [seção 5](#5-testando-sua-tradução) estão corretas;
- [ ] ele foi exportado da **última versão** (ou atualizado como na
      [seção 7](#7-quando-uma-nova-versão-acrescenta-textos)).

Uma tradução parcial também é bem-vinda: diga isso na issue, e os textos que faltam aparecerão
em inglês até que alguém os complete.

**Licença:** o V-Bear é GPL-3.0 ou posterior. Ao enviar uma tradução, você concorda que ela seja
distribuída sob a mesma licença, como o resto do programa.

## 9. Fichas dos jogos (opcional)

As descrições em **Ajuda › Ficha do jogo…** (do que o jogo trata, observações) não estão no
modelo: pertencem ao banco de dados de jogos, um arquivo por idioma, no repositório, em
`assets/games/` (`games.en.json`, `games.pt.json`, `games.es.json`). Para um idioma
sem elas, o emulador as mostra em inglês, então traduzi-las não é necessário para que a
interface fique completa. (O número de jogadores não é um texto: sua frase, "1 ou {n}
jogadores", está no modelo da interface.)

Para traduzi-las também, copie `games.en.json` para `games.<código>.json`, traduza apenas os
valores de `"description"` e `"notes"` (mantenha as chaves dos jogos, como `"game-id"`, e os
nomes dos campos) e envie na mesma issue. Elas são embutidas no programa, então só
aparecem depois da próxima versão.

## 10. Dúvidas

Abra uma issue no GitHub com o título `Translation question: <idioma>`. Perguntas sobre
o significado de um texto são bem-vindas: alguns são curtos e dependem da tela em que estão.
