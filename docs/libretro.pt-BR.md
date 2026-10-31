# V-Bear como core do RetroArch (libretro)

[English](libretro.md) · **Português** · [Español](libretro.es.md)

O V-Bear também é distribuído como um core `v_bear_libretro` (`.so` no Linux e no FreeBSD, `.dll` no
Windows) que roda dentro do RetroArch. O RetroArch cuida então de janela, vídeo, shaders,
áudio, controles, remapeamento, save states, rewind, run-ahead, netplay e RetroAchievements.

O que foi testado:

- **Windows:** executado manualmente no RetroArch. O core carrega, a BIOS é encontrada em `system/supracan/`, e
  jogos `.bin`, `.can`, `.zip` e `.7z` abrem pelo carregador de conteúdo.
- **Linux:** o `.so` foi conduzido por um programa que faz o papel do RetroArch por meio da API C (busca da BIOS,
  `retro_run`, uma ida e volta de save state de ~267 KB, a RAM de bateria de 32 KB). Não foi executado dentro do RetroArch.
O mesmo programa confere que a BIOS é achada em
  `system/supracan/`, `system/superacan/` e `system/SuperAcan/`, solta, num `.zip` e num `.7z`, e que o Super
  Dragon Force (duas partes, num `.zip` ou `.7z`) carrega e mostra a mesma imagem do arquivo unido.
- **FreeBSD e macOS:** o core compila para ambos, mas nunca foi executado, e não há release para macOS.
- A imagem, o som e a taxa de quadros não foram comparados entre o core e o aplicativo independente;
  o core informa a mesma imagem de 320 × 240, aspecto 4:3 e 59,92 fps do aplicativo (`INFO`).

## O que é um core libretro

Uma biblioteca dinâmica que exporta funções C com nomes fixos (`retro_*`). O RetroArch a carrega,
entrega a ROM e chama `retro_run()` uma vez por quadro; o core devolve o quadro
e o áudio, e lê os botões por meio de *callbacks* registrados antes. O core
**não** abre janela, não toca som, não lê o teclado nem cria threads: ele apenas
emula. Referência: `libretro.h` em
<https://github.com/libretro/libretro-common/blob/master/include/libretro.h> e
<https://docs.libretro.com/development/cores/developing-cores/>.

## Como é construído aqui

`crates/v-bear-libretro` (`crate-type = ["cdylib"]`) envolve o `v-bear-core` por meio da
trait `emu_api::Core`; não usa egui nem `emu-frontend`. É gerado por
`tools/new-emulator.sh --libretro` a partir de `tools/templates/libretro.rs.in` e não tem nada
específico do sistema: tudo vem de `SystemInfo` e da trait. A interface e
o core libretro usam **o mesmo** crate de core, de modo que uma correção de emulação vale para ambos.

| libretro | Obtido de |
|---|---|
| `retro_get_system_info` | `SystemInfo::name`, `extensions`; `need_fullpath = false`, `block_extract = false` (o RetroArch descompacta `.zip`/`.7z`) |
| `retro_get_system_av_info` | a geometria do quadro atual, `Core::fps()`, áudio a 48000 Hz |
| `retro_load_game` | `SystemInfo::create(48000)` + `Core::load`; formato de pixel XRGB8888 |
| `retro_run` | leitura da entrada → bitmask do RetroPad por jogador → `Core::run_frame` → vídeo e áudio (f32 → i16) |
| `retro_serialize*` | `Core::save_state` / `load_state` |
| `retro_get_memory_*` | `RETRO_MEMORY_SYSTEM_RAM` → `Core::system_ram` (cheats e RetroAchievements) |
| opções do core | `Core::options()` / `set_option()`, como variáveis `v_bear_<key>` ("label; valor\|valor", o primeiro valor é o padrão) |
| descritores de entrada | `SystemInfo::buttons` (apenas os botões que o sistema tem, com seus nomes) |

A API não tem ponteiro de contexto, então o core vive em uma variável global, e toda função exportada
captura panics: nenhum pode atravessar para o C.

**Opções:** nenhuma (`Core::options()` é vazio). **O que difere do aplicativo independente:**

- **Jogos em dois arquivos** (Super Dragon Force, duas ROMs de máscara): o core pede ao RetroArch o *caminho*
  (`need_fullpath` e `block_extract`), porque o RetroArch extrairia só um arquivo de um pacote, e lê ele mesmo o
  `.zip`, o `.7z` ou o arquivo solto, juntando todos os arquivos, o maior primeiro, como o aplicativo
  independente faz. Um `.7z` ou `.zip` com os dois chips funciona como está.
- **BIOS:** o core procura os quatro arquivos primeiro na pasta `system/supracan/` do RetroArch (a principal),
  depois em `system/superacan/` e `system/SuperAcan/` (a grafia do MiSTer), depois em `system/`, soltos ou dentro
  de arquivos `.zip` ou `.7z`, e os reconhece por CRC32 com qualquer nome. O aplicativo
  independente também lê a pasta de BIOS escolhida em seu menu.
- **Cheats do próprio RetroArch:** `retro_cheat_set` não faz nada. A work RAM é exposta
  (`retro_get_memory_data`), que é o que a busca de cheats e o RetroAchievements leem; os
  cheats `.cht` do aplicativo independente não são lidos pelo core.
- Filtros, shaders e a moldura em volta da imagem ficam a cargo do RetroArch.

## Save states, rewind e determinismo

O RetroArch também usa `retro_serialize` para **rewind, run-ahead e netplay**: o core ganha
esses recursos de graça se o estado for **completo, determinístico e de tamanho fixo**. Todo novo campo
de estado no core deve entrar no snapshot, e o teste "salvar → rodar → restaurar → rodar dá
a mesma imagem e RAM" deve continuar passando.

## O arquivo `.info`

O RetroArch precisa do `v_bear_libretro.info` ao lado do core (ou na sua pasta de info) para listá-lo
direito.

O do V-Bear (`crates/v-bear-libretro/v_bear_libretro.info`) tem `display_name = "Funtech - Super A'Can
(V-Bear)"`, `manufacturer = "Funtech"`, `systemname = "Super A'Can"`, `systemid = "super_acan"`,
`database = "Funtech - Super Acan"` (o nome do conjunto No-Intro copiado no libretro-database),
`supported_extensions = "bin|can|zip|7z"`, `savestate = "true"` e `cheats = "false"`, e lista os quatro
arquivos de BIOS como `firmware0` a `firmware3`, com caminhos `supracan/<arquivo>`. Se o scanner de conteúdo do
RetroArch reconhece os jogos por meio desse banco de dados não foi testado.

Duas coisas que custaram tempo: o RetroArch **guarda em cache as informações do core**, então, depois de copiar um novo `.info`, o
core pode continuar aparecendo como `v_bear_libretro` sem descrição e o navegador de arquivos continua escondendo arquivos `.bin`
e `.can`; limpe o cache de informações de core do RetroArch e reabra-o.
O `.info` vai na pasta `info/` do RetroArch.

## Compilar e instalar

```sh
cargo build --release -p v-bear-libretro   # target/release/libv_bear_libretro.so (v_bear_libretro.dll on Windows)
C=~/.config/retroarch/cores                  # ou a pasta de cores do Flatpak
install -m 755 target/release/libv_bear_libretro.so $C/v_bear_libretro.so
install -m 644 crates/v-bear-libretro/v_bear_libretro.info $C/
retroarch -L $C/v_bear_libretro.so "game.bin"
```

O core não pode depender de bibliotecas que o runtime do RetroArch não tenha (o Flatpak, por
exemplo): esse é mais um motivo para manter o crate do core livre de egui, cpal, arquivos e do
relógio.

## Testes

- O crate gerado tem um teste que executa a API C contra o core usado diretamente e
  compara quadros e uma ida e volta de save state; forneça uma ROM com
  `V_BEAR_TEST_ROM=/path/game cargo test -p v-bear-libretro`.
- O host da base (`libretro-host`) carrega qualquer core libretro como um `emu_api::Core`:
  `LIBRETRO_TEST_CORE=/path/core.so cargo test -p libretro-host --test real_core`.
- Compare quadros: o mesmo jogo no aplicativo independente e no core deve produzir a
  mesma imagem.

## O que falta

- Códigos de cheat do RetroArch (`retro_cheat_set`).
- `SET_MEMORY_MAPS`: não usado, então o RetroAchievements não consegue mapear endereços por região.
- Sem opções de core.

## RetroAchievements

Não investigado: não foi verificado se o RetroAchievements aceita o Super A'Can. O core já
expõe os 64 KB de work RAM (vistos a partir de `$FC0000` pelo 68000), mas sem `SET_MEMORY_MAPS`.
