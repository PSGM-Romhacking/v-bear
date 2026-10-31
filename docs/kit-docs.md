# Kit de documentação (para cada emulador novo)

Documento interno. Os docs de um emulador seguem a estrutura do **Mil Capivaras** (README de 15 seções que é ao mesmo tempo manual e referência técnica do console, mais cinco guias em `docs/`, tudo em três idiomas). Este kit transforma essa estrutura em modelos reaproveitáveis em `docs/kit/`, para cada emulador novo ter a mesma documentação sem recomeçar do zero. Base lida em 20/10/2026: `README.md` e `docs/{architecture,accuracy,libretro,sheets,translating}.md` do Mil Capivaras 0.9.0.

## Mapa: Mil Capivaras → kit

| Mil Capivaras | Modelo do kit | O que é igual / o que muda |
|---|---|---|
| `README.md` (15 seções) | `README.model.md` | Seções **4 (funcionalidades)**, **5 (instalação e uso)**, **11 (libretro)** e **15 (licenças)** já vêm escritas, porque são da interface; a **6** vem em parte. As seções **1, 2, 3, 7, 8, 9, 10, 12, 13, 14** são do sistema: vêm como esboço, com o que o Mil Capivaras documentou em cada uma |
| `docs/architecture.md` | `architecture.model.md` | Estrutura, contrato `emu-api` e frontend iguais; o núcleo do sistema é a parte a escrever |
| `docs/libretro.md` | `libretro.model.md` | O core libretro é **gerado** (`new-emulator.sh --libretro`), então o texto é quase todo igual; muda opções, `.info`, RetroAchievements |
| `docs/translating.md` | `translating.model.md` | Igual para todos (a interface é a mesma); só nome, pasta de config, extensão e URL do repo |
| `docs/sheets.md` | `sheets.model.md` | Parte A (textos dos jogos) igual; parte B (`games.json`) é do núcleo, com o formato do Mil Capivaras como sugestão. Apague se o sistema não tem banco de jogos |
| `docs/accuracy.md` (+ seção 9 do README) | `accuracy.model.md` | Só o esqueleto: tudo depende do sistema e de como ele foi validado |
| `CHANGELOG.md` | `CHANGELOG.model.md` | Mesmo formato (as notas de cada release saem da seção da versão) |

## Como usar

No clone do emulador (depois do `init-project.sh`, com o `new-emulator.sh` já rodado):

```
tools/new-docs.sh --name "Mil Nintendos" --system "Nintendo Entertainment System" \
  --slug nes --ext nes --repo https://github.com/Org/mil_nintendos --force
```

- Preenche os `{{PLACEHOLDERS}}` (`NAME`, `SYSTEM`, `SLUG`, `ID`, `EXT`, `REPO`, `VERSION`, `LICENSE`, `CORE_CRATE`, `DATE`...), tira o comentário de cabeçalho do kit e as marcas `[GENERIC]`/`[SYSTEM]` dos títulos, e escreve `README.md`, `CHANGELOG.md` e `docs/{architecture,accuracy,libretro,sheets,translating}.md`. `--no-game-db` pula o `sheets.md`.
- `--force` é necessário porque o `init-project.sh` já criou um `README.md` e um `CHANGELOG.md` provisórios. Sem ele, o script pula o que já existe.
- O que depende do sistema fica como `<!-- TODO(system): ... -->`, e dentro de cada bloco está o roteiro do que escrever. `TODO(translate)` marca onde entra a linha de troca de idioma.
- Lista do que falta: `grep -rn 'TODO(system)\|TODO(translate)' README.md CHANGELOG.md docs/`.

**Ordem sugerida:** escreva enquanto constrói, não no fim. Seção 2 (hardware) antes do núcleo, 8 (decisões) e 12 (histórico) à medida que decide, 9 e `accuracy.md` à medida que valida, 10 quando tiver a lista de jogos testados.

## Convenções herdadas do Mil Capivaras

- O README é **manual e referência técnica**: "tudo foi tirado do código, de medições ou das fontes listadas em Referências; onde as fontes divergem, a divergência é mostrada".
- **Fontes numeradas** no texto ([W1] artigo, [M2] lista do MAME, [I1] site do autor dos homebrews, [N1] NESdev...) e listadas na seção 14. Cada número do texto tem que ter fonte ou medição.
- **Idiomas:** inglês é o texto-fonte; `X.pt-BR.md` e `X.es.md` ao lado, e no topo de cada arquivo a linha `**English** · [Português](X.pt-BR.md) · [Español](X.es.md)`. Os modelos vêm só em inglês: depois que a versão em inglês estiver pronta, peça à IA para traduzir e mantenha as três em sincronia (não há checagem automática disso).
- **Datas** em AAAA-MM-DD nos docs públicos; tabelas de status que nunca perdem linha (a resolvida diz como).
- `CHANGELOG.md`: a seção de cada versão é a nota da release.
- Nomes de arquivo, comandos e opções citados no texto precisam existir no app: o app gerado só tem `--scale`; `--couch`, `--identify`, `--info`, `--controls`, `--memory-map`, `--bench`, `--shot` e `--dump-ram` são do app do Mil Capivaras e só entram no README se o app do novo emulador os implementar.

## Screenshots

O README cita imagens em `screenshots/`. O frontend gera todas sozinho, com variáveis de ambiente (veja `crates/frontend/src/app/docshots.rs`):

```
EMU_DOCSHOTS=screenshots EMU_DOCSHOTS_GAME=/caminho/jogo.rom \
  XDG_CONFIG_HOME=$(mktemp -d) cargo run --release -p <slug>
```

Use um `XDG_CONFIG_HOME` descartável: o roteiro muda configurações. Ele percorre uma lista fixa de telas, salva um PNG de cada e fecha: `library-covers`, `library-screens`, `couch-library`, `game-tv`, `game-console`, `couch-menu`, `game-info`, `folders`, `debugger-memory`, `debugger-tiles`, `debugger-cpu`, `debugger-video`, `cheats`, `cheats-search`. As que dependem do sistema (molduras `game-tv`/`game-console`, abas do debugger) só aparecem se o app registrar bezels e o núcleo der `debug_views()`. O modelo do README usa só as sempre disponíveis (`library-covers`, `library-screens`, `game-info`, `debugger-memory`, `cheats`); ajuste ao que existir.

## O que o Mil Capivaras tem e o kit não traz

Conteúdo próprio do PV-1000, que cada emulador reescreve do zero para o seu sistema: o hardware (seção 2), as comparações com outras implementações (3), o funcionamento do núcleo (7), a validação contra a referência e as medições no console (9 e `accuracy.md`), a tabela de compatibilidade (10), as lições (13) e as referências (14). Servem de **exemplo de profundidade**: consulte o README do Mil Capivaras antes de escrever a seção correspondente.

## Antes de dar o doc por pronto

- [ ] Nenhum `TODO(system)` nem `{{...}}` sobrando.
- [ ] Todo número e afirmação sobre o hardware tem fonte ([Xn]) ou medição citada.
- [ ] Os links do índice e entre os docs funcionam (âncoras da seção 1 seguem o nome do sistema).
- [ ] Comandos, opções e atalhos citados existem no app.
- [ ] Capturas de tela geradas com o app de verdade.
- [ ] Versões em português e espanhol atualizadas (ou removida a linha de troca de idioma).
