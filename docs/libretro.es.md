# V-Bear como core de RetroArch (libretro)

[English](libretro.md) · [Português](libretro.pt-BR.md) · **Español**

V-Bear también se distribuye como un core `v_bear_libretro` (`.so` en Linux y FreeBSD, `.dll` en
Windows) que se ejecuta dentro de RetroArch. RetroArch se encarga entonces de ventana, video, shaders,
audio, controles, remapeo, save states, rewind, run-ahead, netplay y RetroAchievements.

Lo que se probó:

- **Windows:** ejecutado a mano en RetroArch. El core carga, la BIOS se encuentra en `system/supracan/`, y
  los juegos `.bin`, `.can`, `.zip` y `.7z` se abren desde el cargador de contenido.
- **Linux:** el `.so` fue manejado por un programa que
  hace el papel de RetroArch mediante la API C (búsqueda de la BIOS, `retro_run`, una ida y vuelta de save state de
  ~267 KB, la RAM de batería de 32 KB). El mismo programa comprueba que la BIOS se encuentra en
  `system/supracan/`, `system/superacan/` y `system/SuperAcan/`, suelta, en un `.zip` y en un `.7z`, y que
  Super Dragon Force (dos partes, en un `.zip` o `.7z`) carga y muestra la misma imagen que el archivo unido.
- **FreeBSD y macOS:** el core de FreeBSD lo compila la pipeline de release; no hay release para macOS.
- La imagen, el sonido y la tasa de cuadros no se compararon entre el core y la aplicación independiente;
  el core informa la misma imagen de 320 × 240, aspecto 4:3 y 59,92 fps que la aplicación (`INFO`).

## Qué es un core libretro

Una biblioteca dinámica que exporta funciones C con nombres fijos (`retro_*`). RetroArch la carga,
entrega la ROM y llama a `retro_run()` una vez por cuadro; el core devuelve el cuadro
y el audio, y lee los botones mediante *callbacks* registrados de antemano. El core
**no** abre ventana, no reproduce sonido, no lee el teclado ni crea hilos: solo
emula. Referencia: `libretro.h` en
<https://github.com/libretro/libretro-common/blob/master/include/libretro.h> y
<https://docs.libretro.com/development/cores/developing-cores/>.

## Cómo se construye aquí

`crates/v-bear-libretro` (`crate-type = ["cdylib"]`) envuelve `v-bear-core` mediante el
trait `emu_api::Core`; no usa egui ni `emu-frontend`. Se genera con
`tools/new-emulator.sh --libretro` a partir de `tools/templates/libretro.rs.in` y no tiene nada
específico del sistema: todo viene de `SystemInfo` y del trait. La interfaz y
el core libretro usan **el mismo** crate de core, de modo que una corrección de emulación sirve para ambos.

| libretro | Se obtiene de |
|---|---|
| `retro_get_system_info` | `SystemInfo::name`, `extensions`; `need_fullpath = false`, `block_extract = false` (RetroArch descomprime `.zip`/`.7z`) |
| `retro_get_system_av_info` | la geometría del cuadro actual, `Core::fps()`, audio a 48000 Hz |
| `retro_load_game` | `SystemInfo::create(48000)` + `Core::load`; formato de píxel XRGB8888 |
| `retro_run` | lectura de entrada → bitmask de RetroPad por jugador → `Core::run_frame` → video y audio (f32 → i16) |
| `retro_serialize*` | `Core::save_state` / `load_state` |
| `retro_get_memory_*` | `RETRO_MEMORY_SYSTEM_RAM` → `Core::system_ram` (trucos y RetroAchievements) |
| opciones del core | `Core::options()` / `set_option()`, como variables `v_bear_<key>` ("label; valor\|valor", el primer valor es el predeterminado) |
| descriptores de entrada | `SystemInfo::buttons` (solo los botones que tiene el sistema, con sus nombres) |

La API no tiene puntero de contexto, así que el core vive en una variable global, y toda función exportada
captura los panics: ninguno puede cruzar hacia C.

**Opciones:** ninguna (`Core::options()` está vacío). **Qué difiere de la aplicación independiente:**

- **Juegos en dos archivos** (Super Dragon Force, dos ROM de máscara): el core pide a RetroArch la *ruta*
  (`need_fullpath` y `block_extract`), porque RetroArch extraería solo un archivo de un paquete, y lee él mismo
  el `.zip`, el `.7z` o el archivo suelto, uniendo todos los archivos, el mayor primero, como hace la aplicación
  independiente. Un `.7z` o `.zip` con los dos chips funciona tal cual.
- **BIOS:** el core busca los cuatro archivos primero en la carpeta `system/supracan/` de RetroArch (la principal),
  luego en `system/superacan/` y `system/SuperAcan/` (la grafía del MiSTer), luego en `system/`, sueltos o dentro
  de archivos `.zip` o `.7z`, y los reconoce por CRC32 con cualquier nombre. La aplicación
  independiente también lee la carpeta de BIOS elegida en su menú.
- **Los trucos propios de RetroArch:** `retro_cheat_set` no hace nada. La work RAM está expuesta
  (`retro_get_memory_data`), que es lo que leen la búsqueda de trucos y RetroAchievements; los
  trucos `.cht` de la aplicación independiente no los lee el core.
- Los filtros, los shaders y el marco alrededor de la imagen corresponden a RetroArch.

## Save states, rewind y determinismo

RetroArch también usa `retro_serialize` para **rewind, run-ahead y netplay**: el core los obtiene
gratis si el estado es **completo, determinista y de tamaño fijo**. Todo campo de estado nuevo
en el core debe entrar en el snapshot, y la prueba "guardar → ejecutar → restaurar → ejecutar da
la misma imagen y RAM" debe seguir pasando.

## El archivo `.info`

RetroArch necesita `v_bear_libretro.info` junto al core (o en su carpeta de info) para listarlo
bien.

El de V-Bear (`crates/v-bear-libretro/v_bear_libretro.info`) tiene `display_name = "Funtech - Super A'Can
(V-Bear)"`, `manufacturer = "Funtech"`, `systemname = "Super A'Can"`, `systemid = "super_acan"`,
`database = "Funtech - Super Acan"` (el nombre del conjunto No-Intro copiado en libretro-database),
`supported_extensions = "bin|can|zip|7z"`, `license = "GPLv3"`, `needs_fullpath = "true"` (el core lee el archivo por sí mismo),
`database_match_archive_member = "true"` (la lista conserva el `.zip` o `.7z`, que Super Dragon Force necesita),
`savestate = "true"`, `cheats = "false"` y `core_options = "false"`, una `description` corta, y lista los cuatro
archivos de BIOS como `firmware0` a `firmware3`, con rutas `supracan/<archivo>`. No se probó si el escáner de contenido de
RetroArch reconoce los juegos a través de esa base de datos.

Dos cosas que costaron tiempo: RetroArch **guarda en caché la información del core**, así que, tras copiar un nuevo `.info`, el
core puede seguir mostrándose como `v_bear_libretro` sin descripción y el explorador de archivos sigue ocultando los archivos `.bin`
y `.can`; limpie la caché de información de cores de RetroArch y vuelva a abrirlo.
El `.info` va en la carpeta `info/` de RetroArch.

## Enviando el core a libretro

Para que el core entre en el Core Downloader de RetroArch, libretro pide cuatro cosas
([documentación](https://docs.libretro.com/development/cores/developing-cores/#add-your-core-to-libretro-infrastructure)):

- el archivo `.info`, enviado como pull request a `libretro-super` (`dist/info/`);
- un `.gitlab-ci.yml` en la raíz del repositorio, que está en este: compila solo
  `v-bear-libretro` con las plantillas Rust de libretro, para Linux x64 y Windows x64, y renombra la
  biblioteca a `v_bear_libretro.so` / `.dll`;
- un espejo del repositorio en el GitLab de libretro, que un mantenedor crea a petición (Discord, #programming);
- opcionalmente, la página del core en `libretro/docs`.

El archivo de CI nunca se ejecutó: solo los servidores de libretro lo compilan. No hay plantilla para FreeBSD,
así que esa compilación se queda en la release de este repositorio.

## Compilar e instalar

```sh
cargo build --release -p v-bear-libretro   # target/release/libv_bear_libretro.so (v_bear_libretro.dll on Windows)
C=~/.config/retroarch/cores                  # o la carpeta de cores del Flatpak
install -m 755 target/release/libv_bear_libretro.so $C/v_bear_libretro.so
install -m 644 crates/v-bear-libretro/v_bear_libretro.info $C/
retroarch -L $C/v_bear_libretro.so "game.bin"
```

El core no debe depender de bibliotecas que el runtime de RetroArch no tenga (el Flatpak, por
ejemplo): ese es un motivo más para mantener el crate del core libre de egui, cpal, archivos y del
reloj.

## Pruebas

- El crate generado tiene una prueba que ejecuta la API C contra el core usado directamente y
  compara cuadros y una ida y vuelta de save state; désele una ROM con
  `V_BEAR_TEST_ROM=/path/game cargo test -p v-bear-libretro`.
- El host de la base (`libretro-host`) carga cualquier core libretro como un `emu_api::Core`:
  `LIBRETRO_TEST_CORE=/path/core.so cargo test -p libretro-host --test real_core`.
- Compare cuadros: el mismo juego en la aplicación independiente y en el core debe producir la
  misma imagen.

## Qué falta

- Códigos de trucos de RetroArch (`retro_cheat_set`).
- `SET_MEMORY_MAPS`: no se usa, así que RetroAchievements no puede mapear direcciones por región.
- Sin opciones de core.

## RetroAchievements

No investigado: no se comprobó si RetroAchievements acepta la Super A'Can. El core ya
expone los 64 KB de work RAM (vistos desde `$FC0000` por el 68000), pero sin `SET_MEMORY_MAPS`.
