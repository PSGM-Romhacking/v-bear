# Referencia: el core MiSTer simulado con Verilator

[English](README.md) · [Português](README.pt-BR.md) · **Español**

La herramienta que compara V-Bear con una segunda implementación del mismo hardware. Simula, ciclo
a ciclo, el (System)Verilog del
[core MiSTer de la Super A'Can](https://github.com/MiSTer-devel/SuperACan_MiSTer), ejecuta un juego y
guarda la imagen y la work RAM, para poder compararlas con lo que V-Bear produce en el mismo fotograma.
El método y los resultados están en [Precisión y validación](../docs/accuracy.es.md).

No forma parte del emulador: nada de aquí se compila con `cargo build`.

## Antes de usarla: qué es y qué no es esta referencia

**Es una segunda reconstrucción, no la consola.** El core MiSTer se escribió a partir de ingeniería
inversa (su README, "Development and QA"): videos de hardware real que muestran las pantallas de
demostración de los juegos, grabados por Diskman, desensamblados de toda la biblioteca y documentación
anterior. Su autor no lo probó contra una consola real: la consola cuesta miles de dólares y no existe
flash cart. Su propio README dice que la biblioteca funciona "apparently bug free" (aparentemente sin
errores).

Lo que se deduce de esto para quien lee los resultados:

- **Idéntico al core MiSTer no es idéntico a la consola.** Que dos reconstrucciones coincidan es
  evidencia de que ambas están bien, no una prueba. Si coinciden en un error, la comparación no lo ve.
- **Una discrepancia no dice quién se equivoca.** Una diferencia puede estar en V-Bear, en el core MiSTer
  o en esta simulación. Hizo falta un tercer lado (MAME 0.289) para resolver el caso de la ventana 0.
- **El repositorio del core es nuevo:** un solo commit (2026-09-11, `59ba275`), sin historial de
  correcciones que muestre qué se ha encontrado y corregido ya.

Limitaciones de la simulación en sí (nuestras, no del core):

- **El cartucho se lee con un retardo fijo** (`CART_LATENCY`, 12 ciclos de `clk_sys` en `sim_top.sv`). En
  un MiSTer real la ROM viene de la SDRAM, con retardo variable. Es una suposición, y cambia el tiempo de
  arranque de cada juego: por eso los dos lados no están en el mismo fotograma (véase "Alineación" más
  abajo).
- **Aún sin entrada:** la simulación no pulsa botones. Solo se compara el inicio de cada juego (la
  introducción de la BIOS y el comienzo del juego), no menús ni partidas.
- **Lenta:** alrededor de 1 fotograma por segundo (medido: 120 fotogramas en unos 2 min 20 s), así que 300
  fotogramas tardan ~5 min por juego y 3.000 fotogramas ~50 min.
- **Sin sonido, sin guardado de batería, sin save state.** Solo la imagen y los 64 KB de work RAM.
- **La RAM solo tiene sentido en el mismo punto del fotograma.** Los dos lados se detienen en puntos
  ligeramente distintos del fotograma y dejan restos distintos en la cima de la pila del 68000 y en
  algunos contadores; eso no es un error de emulación.

## Uso

Requisitos: Verilator 5 y un compilador C++17; Python 3. El core MiSTer no está en este repositorio: el
`Makefile` lo busca en `_referencias/mister-59ba275` (una copia local, fuera de git, registrada en
[Referencias](../docs/references.es.md)); `make MISTER=/other/path` apunta a otra copia.

```bash
make -C reference                          # builds reference/acan-ref (~10 s)
cargo build --release                      # builds V-Bear
FRAMES=300 reference/compare.sh target/release/v-bear "roms/Game.zip" ...
```

`compare.sh` ejecuta cada juego en ambos e imprime, por juego y fotograma: el **lag** (el desfase en
fotogramas que se encontró), los **píxeles** distintos y los **bytes de RAM** distintos (0 = idéntico).
Opciones: `--jobs N` (juegos en paralelo; cada simulación usa un núcleo), `--bios DIR` (la carpeta con
los cuatro archivos, por defecto `bios/`), `--window W` (cuántos fotogramas alrededor buscar, por defecto
4) y `--offset K`. Fotogramas por variable: `FRAMES="300 600"`.

La referencia sola:

```bash
reference/acan-ref GAME.bin --bios bios --shot 300 out.ppm --dump-ram ram.bin
reference/acan-ref GAME.bin --bios bios --frames-out 290 315 prefix   # several frames as PPM
```

V-Bear escribe el equivalente con `V_BEAR_SHOT=<file>` y `V_BEAR_RAM=<file>` (modo sin ventana,
`--frames N`).

## Alineación

La simulación y V-Bear no llegan al mismo fotograma al mismo tiempo. En la introducción de la BIOS el
desfase es de 1 fotograma, constante. Al inicio de cada juego varía de 0 a unos 13 fotogramas, según el
juego. Por eso la comparación busca, en una ventana de fotogramas de V-Bear, el que mejor coincide con el
fotograma N de la simulación, e imprime el desfase (`LAG`). En una imagen quieta muchos fotogramas dan 0
píxeles y el `LAG` no es fiable; en una animación sí lo es.

Dos formatos de imagen distintos se normalizan antes de comparar: V-Bear siempre produce 320×240 (líneas
H256 estiradas; el modo de 224 líneas centrado, con 8 líneas negras arriba y abajo), y la simulación
produce el tamaño real (256×224, 320×224, 320×240...). La comparación aplica la misma conversión de
`crates/v-bear-core/src/video.rs` y compara colores exactos (8 bits por canal).

## Qué esperar

La tabla de resultados y la lista de hallazgos están en [Precisión y validación](../docs/accuracy.es.md).
Aparecen dos patrones de diferencia que no se atribuyeron a un error de emulación (son
interpretaciones, no pruebas):

- **Un fade cuyos canales de color suben en cuadros distintos** (Sango Fighter, Gambling Lord): la imagen es
  idéntica cada cuarto cuadro y, en los cuadros intermedios, una región difiere en 1 nivel de uno o más
  canales (123 contra 132), porque V-Bear y la simulación actualizan los tres canales en cuadros distintos
  del ciclo de 4 cuadros. No se encontró la causa ([Precisión](../docs/accuracy.es.md)).
- **Restos de la pila y contadores en la RAM** (unos pocos bytes): los dos lados se detienen en puntos
  ligeramente distintos del fotograma. En Boom Zoo, 9 bytes: 2 contadores con un paso de diferencia y 7
  bytes en la cima de la pila. Esto también es una hipótesis: la RAM solo se comparó en el mismo
  fotograma, no en varios puntos del fotograma.

## Archivos

| Archivo | Qué es |
|---|---|
| `sim_top.sv` | la cima de la simulación: la consola del core MiSTer sin el framework MiSTer; la ROM del cartucho es un array, leído por el mismo protocolo request/busy/done del puente SDRAM |
| `sim.h` | la simulación: reloj, carga de la BIOS y de la ROM, captura de video, el bucle de fotogramas, lectura de la RAM |
| `ref_main.cpp` | la línea de comandos de `acan-ref` |
| `Makefile` | compila con Verilator y copia `microrom.mem` y `nanorom.mem` (`fx68k` los lee del directorio actual) |
| `compare.py`, `compare.sh` | ejecutan la referencia y V-Bear y comparan |

Cómo trata Verilator el core: el código del core se lee sin cambios. Para compilarlo hicieron falta
`-Wno-BLKANDNBLK` (una limitación conocida de `fx68k` en Verilator) y el orden de los archivos
(`mos6502_pkg.sv` primero), y **no** se debe usar `--public-flat-rw` (expone todas las señales y hace que
GCC tarde más de 25 minutos en un solo archivo; el array de la ROM es público mediante un comentario en el
propio `sim_top.sv`, y la RAM se lee por el puerto `ss_wram` del core).

## Licencias

| Parte | Licencia |
|---|---|
| El core MiSTer (`_referencias/mister-59ba275`, no incluido aquí) | GPL-2.0 (su archivo `LICENSE`) |
| El resto de esta carpeta | GPL-2.0 o posterior (como en Mil Capivaras: se compila junto con el core GPL-2.0, y V-Bear es GPL-3.0) |

La referencia es un programa aparte: no está enlazada al emulador.
