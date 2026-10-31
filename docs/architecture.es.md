# Arquitectura y cómo funciona

[English](architecture.md) · [Português](architecture.pt-BR.md) · **Español**

Cómo está organizado el código, cómo funciona la emulación y el razonamiento detrás de cada decisión.
V-Bear es un workspace de Cargo con la consola separada de la
interfaz; solo se conocen a través de un contrato pequeño (`emu-api`).

## Estructura

```
crates/api/             contrato core ↔ frontend (sin dependencias)
crates/v-bear-core/   Funtech Super A'Can: máquina, memoria, base de datos de juegos, pruebas
crates/frontend/        frontend reutilizable (egui, menús, biblioteca, mandos,
                        save states, rewind, grabación, carátulas, idiomas, configuración)
crates/v-bear/        ejecutable: core + frontend + identidad visual (nombre, icono, Acerca de)
crates/v-bear-libretro/   v_bear_libretro.so = core + API libretro
crates/libretro-host/, libretro-player/, toy*/   de la base: un host para cores libretro ya hechos
                        y un ejemplo de juguete; V-Bear no los usa
assets/                 iconos, fuentes, bezels, i18n/<código>.json, games/
docs/                   la documentación (lista en el README)
```

- `v-bear-core` → solo `emu-api` (+ las bibliotecas que necesita la emulación).
- `emu-frontend` → solo `emu-api`: no conoce ninguna consola.
- `crates/v-bear` une los dos; el core libretro usa el core y `emu-api`,
  nunca el frontend.

Dependencias del core (`crates/v-bear-core/Cargo.toml`): `m68k` 0.14 (MIT, el 68000, con su feature
`serde`) y `mos6502` 0.10 (BSD-3-Clause, el 65C02), ambos de crates.io, sin nada vendorizado ni bifurcado;
`serde`, `serde_json` y `bincode` para los save states; `zip` para leer una ROM o la BIOS desde un `.zip`.
Las licencias de los crates están listadas en el README.

## Contrato (`emu-api`), siguiendo las convenciones de libretro

| Aspecto | Decisión |
|---|---|
| Controles | bitmask **RetroPad** por jugador (B, Y, Select, Start, ↑ ↓ ← →, A, X, L, R, L2, R2, L3, R3); el sistema declara cuáles usa y el nombre de cada uno |
| Video | `&[u32]` **XRGB8888** + ancho/alto/relación de aspecto por cuadro |
| Audio | **estéreo intercalado f32**, frecuencia elegida por el frontend |
| Save state | bytes opacos (`retro_serialize`); el encabezado del archivo (magic, hash de la ROM, miniatura) pertenece al frontend |
| Memoria | `system_ram()` (`RETRO_MEMORY_SYSTEM_RAM`) |
| Metadatos | `SystemInfo`: nombre, extensiones, nombre en libretro-thumbnails, forma de la carátula, botones, FPS, resolución, identificación por hash |
| Tasa de cuadros | `Core::fps()` (por defecto: la de `SystemInfo`) |
| Opciones | `Core::options()` / `set_option()`, como las *core options* de libretro |
| Depuración | `debug_views()`: imágenes (con cuadrícula opcional) y tablas en grupos |

Lo que hace la Super A'Can con cada fila:

| Aspecto | En V-Bear |
|---|---|
| Controles | 2 jugadores. El mando es estilo SNES: Arriba, Abajo, Izquierda, Derecha, A, B, X, Y, L, R, Select y Start; cada bit del RetroPad va al bit del mando con el mismo nombre (`PAD_MAP` en `core.rs`); L2, R2, L3 y R3 no se usan |
| Video | siempre 320 × 240; los modos de 256 puntos se estiran al mismo ancho, como los muestra un televisor, y el modo de 224 líneas deja en blanco 8 líneas arriba y abajo. Los píxeles son el xBGR555 de la paleta expandido a XRGB8888; relación de aspecto 4:3 |
| Audio | el flujo estéreo de 44.744,3 Hz del sampler, multiplicado por 4 (deja mucho margen) y remuestreado a la frecuencia del frontend |
| Estado | un encabezado de archivo (`VBEARST`, versión del formato, el CRC32 de la ROM) seguido del estado completo de la máquina; ~267 KB. La ROM, la BIOS y la SRAM de batería quedan fuera |
| RAM | los 64 KB de work RAM, vistos desde `$FC0000`, para los cheats y el depurador |
| Batería | `battery_ram()`: la SRAM de 32 KB del cartucho (`.sav`) |
| Opciones | ninguna |
| FPS | MCLK / (3420 × 262) = 53.693.175 / 896.040 ≈ 59,92 Hz (`FPS_NUM`/`FPS_DEN` en `core.rs`): el cristal de 53,693175 MHz, 3.420 clocks maestros por línea y 262 líneas |

## El crate del core

| Archivo | Contenido |
|---|---|
| `lib.rs` | los `pub mod`s y `pub use core::{INFO, …Core}` |
| `core.rs` | el **adaptador**: `static INFO: SystemInfo` e `impl emu_api::Core` (RetroPad → entrada nativa, imagen → XRGB8888, audio → estéreo f32, estado ↔ bytes) |
| `console.rs` | la máquina: conecta las dos CPU, el bus, el video, el sonido y los mandos, y los ejecuta línea por línea (`run_frame`); también `snapshot`/`restore` y los archivos de save state y de SRAM |
| `bus.rs` | el bus del 68000: decodificación de direcciones, el puerto host del UM6619, interrupciones, el contador libre y los dos canales de DMA del host |
| `bios.rs` | los cuatro archivos de la BIOS, identificados por CRC32 |
| `cart.rs` | el cartucho: detección del orden de bytes, `.zip` con varias mask ROMs, la SRAM |
| `video.rs`, `render.rs` | el UM6618: registros, paleta, VRAM y el renderizador (tilemaps, ROZ, ventanas, sprites, mixer) |
| `sound.rs` | el lado de sonido del UM6619: el 65C02, su RAM y página de E/S, los puertos de los mandos, los buzones con el 68000 y el sampler de 16 canales |
| `audio.rs` | remuestrea la salida del sampler a la frecuencia del dispositivo |
| `input.rs` | el registro de desplazamiento de 16 bits del mando |
| `state.rs` | el formato del archivo de save state |
| `debug.rs` | interruptores de depuración leídos una vez desde `V_BEAR_DEBUG` (`snd`, `t65`, `dma`, `irq`...), para el modo de pruebas sin ventana |
| `games.rs` | base de datos de juegos (identificación por CRC32) y los textos de los juegos |

## El frontend

`emu-frontend` se comparte tal cual. Cada archivo de `app/` es un bloque `impl App` (bucle de emulación,
entrada, navegación con mando, menús, pantalla, biblioteca, save states, grabación, controles,
diálogos). Lee todo sobre el sistema desde `SystemInfo`; lo único específico del sistema
es lo que la aplicación le pasa: `brand::set(Brand { name, id, version, icon_png, about,
repo_url, bezels, free_games, thumbnails_url })` y los textos de idioma propios de la aplicación
(`i18n::add_catalog`).

## Pruebas

`cargo test --workspace` ejecuta todo lo que no necesita un juego:

- `games.rs`: todo juego tiene sus textos en todos los idiomas, y un juego se encuentra con cualquier orden de bytes;
- `sound.rs`: el save state del 65C02 conserva el estado de espera y el flanco del NMI, el mando se lee empezando por el bit
  más significativo, el buzón y los latches, y una voz de sampler en bucle;
- `state.rs`: el archivo de save state hace el viaje de ida y vuelta y rechaza otro juego, otra versión de formato y basura;
- `crates/v-bear/src/main.rs`: los textos de los juegos son válidos (lo comprueba el frontend);
- `crates/v-bear-libretro`: la API C de libretro coincide con el core y los estados hacen el viaje de ida y vuelta.

`crates/v-bear-core/tests/contract.rs` ejecuta un juego real a través del contrato `emu-api` (900 cuadros:
una imagen la mayor parte del tiempo, la cantidad correcta de sonido, y un save state que hace que los 120 cuadros siguientes
sean idénticos). Necesita archivos que no están en el repositorio, por lo que se omite a menos que estén definidas `V_BEAR_TEST_ROM` (un
`.zip` de juego) y `V_BEAR_TEST_BIOS` (una carpeta con los cuatro archivos de la BIOS).

La comparación con otras implementaciones no forma parte de `cargo test`: véanse
[Precisión y validación](accuracy.es.md) y [Compatibilidad](compatibility.es.md).

## Cómo funciona la emulación

**Base de tiempo.** El reloj maestro es el cristal de 53.693.175 Hz (`MCLK`, `console.rs`). Una línea son 3.420
clocks maestros y un cuadro 262 líneas, lo que da 59,92 cuadros por segundo. El 68000 corre a MCLK / 5 (684
ciclos por línea) y el 65C02 a ese reloj dividido por 3.

**Un cuadro** (`Console::run_frame`). Para cada una de las 262 líneas:

1. `start_line` dispara las interrupciones de la línea: vertical blank en la línea 240 (y el NMI de la CPU de sonido, si
   está habilitado), la interrupción de línea en las líneas 1 a 239 y la comparación de raster "line on" / "line off";
2. el UM6618 construye la línea siguiente en un búfer de línea mientras se muestra esta (en el core de MiSTer,
   `render_y = vpos + 1`), así que `render_line(next)` se ejecuta ahora: la línea L ve los registros y la VRAM tal como están
   cuando empieza la línea L−1;
3. la línea se ejecuta en 4 porciones, alternando el 68000 (`run_cpu`, 171 ciclos) y el 65C02 con el sampler (57 ciclos de ellos):
   después de cada lote de instrucciones, la posición horizontal se deriva de los ciclos usados.

Las instrucciones nunca se dividen: los ciclos que se ejecutan más allá del final de una porción (`overshoot`) se descuentan de la
siguiente. Un DMA mantiene al 68000 fuera del bus (`stall`) mientras el contador sigue corriendo.

**CPU.** El 68000 es el crate `m68k`, ejecutado en lotes de ciclos con un hook que actualiza el nivel de interrupción
después de cada instrucción; las instrucciones ilegales, A-line y F-line se cuentan en `faults`, que es como un fallo
aparece en las pruebas. El 65C02 es el crate `mos6502` (`Mos65C02`), dentro de `sound.rs`, con la página de E/S del UM6619
decodificada a su alrededor.

**Video** (`video.rs`, `render.rs`). Cada una de las 240 líneas visibles se construye a partir de las capas en el
orden del mixer: cuatro tilemaps, la capa ROZ, la ventana 0 y la ventana 1, y luego los sprites (hasta 128 entradas en la
tabla, con escalado, mosaico y máscaras). El mixer trabaja en el espacio de índices de la paleta; la paleta tiene 256 entradas de
xBGR555. El ancho es de 256 o 320 puntos (`$F00008`, bit 8), y el bit 9 deja en blanco 8 líneas arriba y
abajo. Los detalles del chip están en [Hardware](hardware.es.md).

**Sonido.** El sampler de 16 canales produce una muestra estéreo cada 80 ciclos de la CPU de sonido
(44.744,3 Hz). `audio.rs` la multiplica por 4 y la remuestrea a la frecuencia que pide el frontend.

**Save states.** `Console::snapshot` copia la máquina entre cuadros: el 68000 pasa por su forma `serde`
(JSON), el resto (`Bus`, que también contiene la CPU de sonido) por `bincode`. La ROM, la BIOS y la
SRAM de batería no se guardan, de modo que un estado cargado después no deshace un guardado hecho en el juego. Un estado es exacto:
cargarlo y ejecutar 120 cuadros da la misma imagen que haberlos ejecutado desde el principio (lo comprueba
`tests/contract.rs`).

## El razonamiento detrás de cada decisión

| Decisión | Por qué, y qué cuesta |
|---|---|
| Cuando MAME y el core de MiSTer discrepan, seguir al core de MiSTer | modela toda la placa y ejecuta toda la biblioteca; las discrepancias quedan registradas en [Hardware](hardware.es.md). Costo: el core de MiSTer es en sí mismo una reconstrucción, con un commit y sin pruebas en una consola real ([Precisión](accuracy.es.md)) |
| El 68000 y el 65C02 son crates, no se escribieron aquí | crates existentes, MIT y BSD-3-Clause. Costo: V-Bear depende de cómo cuentan los ciclos; el 68000 no es `Clone`, así que el estado pasa por su forma `serde` |
| La máquina corre línea por línea, en 4 porciones por línea | suficiente para los efectos que usan los juegos (interrupciones de raster, scroll por línea y tablas de ventana) y barato. Costo: no es cycle-accurate dentro de una línea, y la posición horizontal se deriva, no se simula |
| El renderizador construye una línea entera de una vez, una línea por adelantado | es lo que hace el core de MiSTer (`render_y = vpos + 1`). Costo: la línea L ve los registros y la VRAM como estaban cuando empezó la línea L−1 |
| Salida fija de 320 × 240 | el frontend trabaja con un tamaño fijo por sistema; las líneas de 256 puntos se estiran, como las muestra un televisor, y el modo de 224 líneas se muestra con barras negras. Costo: no se ve el tamaño real de la imagen; la comparación con el core de MiSTer lo convierte de vuelta |
| Relación de aspecto 4:3 | un televisor; la relación de aspecto del píxel de la consola no se midió |
| La BIOS es obligatoria y se identifica por CRC32 | la boot ROM del 68000, las dos ROM de la CPU de sonido y la clave de lockout son necesarias para arrancar; el nombre del archivo no importa. Costo: el usuario tiene que encontrar los cuatro archivos, que no se distribuyen |
| Los juegos se identifican por CRC32 contra una base de datos de 12 juegos | el frontend muestra la carátula y la ficha del juego según el contenido. Costo: una ROM que no está en la base de datos (un homebrew, un hack) no tiene ficha |
| Los save states usan un encabezado con el CRC32 de la ROM | se rechaza un estado de otro juego. Costo: el formato tiene versión; cambiar las estructuras guardadas deja ilegibles los estados antiguos |
| Todo lo que necesita la interfaz está incrustado | icono, fuentes, traducciones y la base de datos de juegos están en el ejecutable; no hay nada que instalar. Costo: un binario más grande |
