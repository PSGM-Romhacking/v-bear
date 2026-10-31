# Banco de dados de jogos

[English](sheets.md) · **Português** · [Español](sheets.es.md)

A ficha de informações (clique com o botão direito no jogo › **Ficha do jogo…**, ou **Ajuda › Ficha do jogo…**) e o
nome e a capa do jogo na biblioteca vêm do **banco de dados de jogos**, embutido no
programa. Ele fica separado dos textos do próprio emulador (menus, botões), que são as
traduções da interface em `assets/i18n/`.

| Arquivo | O que contém |
|---|---|
| `crates/v-bear-core/src/games.rs` | os dados de cada jogo, iguais em todos os idiomas: uma tabela no código (parte B) |
| `assets/games/games.en.json` | os textos de cada jogo em inglês, o texto-fonte (parte A) |
| `assets/games/games.<code>.json` | os mesmos textos em outro idioma (parte A) |

De onde vem: os 12 jogos, seus CRC32 e seus nomes No-Intro vêm do conjunto No-Intro
(cópia no libretro-database) e da lista de software do MAME (`hash/supracan.xml`, CC0); títulos, anos e
publicadoras vêm da lista; as desenvolvedoras, como na lista da Wikipedia; o número de jogadores, do LaunchBox Games
DB. As descrições foram escritas para o V-Bear a partir dos próprios jogos, porque a lista do MAME só tem
notas de emulação. As fontes e as datas estão em [Referências](references.pt-BR.md). O conjunto está completo (12 jogos
lançados) e não muda: um jogo novo seria uma entrada nova.

## A. Os textos: `games.<code>.json`

Lidos pela interface. O núcleo os entrega em `SystemInfo::game_texts`
(`&[("en", include_str!(…)), …]`), indexados pelo id do jogo (`GameInfo::id`).

```json
{
  "game-id": {
    "genre": "Platform",
    "description": "What the game is about…",
    "notes": "…"
  }
}
```

| Campo | Notas |
|---|---|
| chave (`game-id`) | o id do jogo: minúsculas, sem espaços; a mesma chave na tabela e em todos os arquivos de texto |
| `genre` | uma ou duas palavras, traduzidas em cada arquivo; exibido na ficha e na visualização em lista da biblioteca |
| `description` | exibida na ficha |
| `notes` | exibidas no final da ficha |

Todos os campos são opcionais. Sem o texto no idioma atual, a ficha mostra o
inglês. Um campo desconhecido (um erro de digitação como `descripton`) é um erro. Um idioma novo é um novo
`games.<code>.json` com os mesmos jogos ([translating.pt-BR.md](translating.pt-BR.md)).

## B. Os dados: a tabela `GAMES`

Lida pelo núcleo (`games.rs`), que responde a `SystemInfo::game_info(rom)` com um `GameInfo` (`emu-api`):
`id`, `title`, `alt_title`, `year`, `publisher`, `developer`, `serial`, `players`, `thumbnail`. No V-Bear os
dados não ficam em um arquivo JSON, e sim em uma tabela Rust, uma linha por jogo:

```rust
(0x6099bb44, "boomzoo", "Boom Zoo", "Bao Bao Dong Wu Yuan", "1996", "Funtech", "Boom Zoo (Taiwan)", 2),
```

| Campo | Notas |
|---|---|
| CRC32 | da ROM como o frontend a lê (os arquivos de um zip unidos, o maior primeiro): é assim que o jogo é reconhecido, qualquer que seja o nome do arquivo. Uma ROM também é procurada com suas palavras de 16 bits trocadas, porque existem dumps nas duas ordens de bytes |
| id | minúsculas, sem espaços; a mesma chave dos arquivos de texto |
| title | o nome do jogo |
| alternative title | o título em pinyin |
| year | |
| developer | como na lista de jogos do Super A'Can da Wikipedia |
| thumbnail | o nome No-Intro da ROM, que é o nome de suas imagens de capa, de título e de jogabilidade, como no RetroArch ([libretro-thumbnails](https://thumbnails.libretro.com/)) |
| players | máximo de jogadores ao mesmo tempo, do LaunchBox Games DB; só o número, a frase ("1 or 2 players") vem da tradução da interface |

A publicadora é a mesma para todos os jogos (Funtech: as caixas dizem quem fez o jogo e que a Funtech
(敦煌) o publicou), por isso é uma constante, e nenhum número de série é registrado.

Uma ROM que não está na tabela (um hack, um homebrew) não ganha ficha. O CRC32 é `emu_api::crc32` sobre
os bytes da ROM; o V-Bear não tem opção de linha de comando para exibi-lo.

## Adicionar um jogo

1. Encontre o CRC32 e o nome No-Intro dele.
2. Adicione uma linha a `GAMES` (e aumente o tamanho do array).
3. Adicione os textos dele a `games.en.json`, `games.pt.json` e `games.es.json`.
4. Confira: `cargo test --workspace` (o teste `texts_cover_every_game` falha se um arquivo tiver um conjunto de jogos diferente)
   e abra a ficha de informações do jogo.

## Regras (coloque-as nos testes)

- Todo arquivo de texto tem **os mesmos jogos e os mesmos campos**; só o texto muda
  (`texts_cover_every_game`; o frontend também confere os campos: `game_texts_valid`).
- Só entra conteúdo verificável: desenvolvedora com fonte, descrição do autor
  ou do jogo.

**Títulos em alfabeto não latino:** a interface embute um subconjunto da Noto Sans CJK JP
(`assets/fonts/`, SIL OFL): kana, pontuação e apenas os kanji já em uso. Um título
com kanji novo exige regenerar o subconjunto.
