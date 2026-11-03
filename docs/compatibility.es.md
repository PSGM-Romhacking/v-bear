# Compatibilidad

[English](compatibility.md) · [Português](compatibility.pt-BR.md) · **Español**

Estado de cada juego en V-Bear, con la BIOS. Actualizar en cada ronda de pruebas, sin borrar las
anteriores (el historial queda en las secciones por fecha).

ROMs: las 12 del set No-Intro, verificadas por CRC32 con el softlist de MAME (ver `references.es.md`).

## Cómo probar

Todo sin ventana, con `--frames N` (la BIOS se encuentra en `bios/`):

```sh
# capturas cada 450 cuadros, con un guion de botones (cuadro:botón, mantiene 8 cuadros)
V_BEAR_INPUT="1500:start,1560:a" V_BEAR_SHOT=/tmp/g V_BEAR_SHOT_EVERY=450 \
  ./target/release/v-bear "roms - Funtech - Super Acan/<juego>.zip" --frames 7200
# sonido en WAV (44 744 Hz, estéreo)
V_BEAR_WAV=/tmp/juego.wav ./target/release/v-bear <rom> --frames 1800
```

El resumen final muestra los IRQ atendidos por nivel, paleta/VRAM en uso, pico de audio, bytes de SRAM,
`faults` (excepciones ilegales, A-line, F-line del 68000) y `halted` (CPU detenida por doble falla).
Otras claves: `V_BEAR_REGS`, `V_BEAR_VRAM=<palabra>`, `V_BEAR_SND=<dir>`, `V_BEAR_LIVE=<cuadro>`
(rastrea el 68000), `V_BEAR_PCS`, `V_BEAR_DEBUG=snd,t65,dma,irq,watch,vregs,sndr,pal`,
`V_BEAR_SNAPTEST=<cuadro>` (savestate determinista), `V_BEAR_RESET=<cuadro>` (reinicia ahí),
`V_BEAR_STATE=<archivo>` (parte de un save state grabado por la interfaz, para reproducir un fallo del momento en que se guardó), `V_BEAR_RAM=<archivo>` (graba los 64 KB de la RAM de trabajo al final, para comparar con `reference/`).
`V_BEAR_SHOT` sin `_EVERY` graba solo la última pantalla, en PNG si el nombre termina en `.png`.

## 2026-10-21

Dos guiones: uno fijo (Start/A/direcciones cada 60 cuadros, 7200 cuadros) y uno aleatorio
(semilla 7, un botón cada 12 cuadros, 20 000 cuadros ≈ 5,5 min de juego). En los 12 juegos:
`faults=0`, `halted=false`.

| Juego | Hasta dónde llegó | Observaciones |
|---|---|---|
| Boom Zoo | partidas del rompecabezas contra varios adversarios | — |
| Formosa Duel | título, partida | — |
| Gambling Lord | selección de personaje, partidas de mahjong | — |
| Journey to the Laugh | cuatro fases de plataformas, jefe araña | — |
| Magical Pool | título, menú de modos, mapa | el guion aleatorio no sale del título (necesita Start en el momento justo) |
| Monopoly – Adventure in Africa | tablero, días avanzando, compra y venta | — |
| Rebel Star (En) | introducción, título, mapa con diálogo | — |
| Sango Fighter | selección, luchas en dos escenarios | corregido en esta fecha: la lucha quedaba cubierta por la ventana 0 (IRQ4 no se activaba) |
| Speedy Dragon | introducción, varias fases | corregido el 2026-10-20: carrera driver × timeout (esperas en la RAM de sonido) |
| Super Dragon Force | título, mazmorra, mapa del mundo | — |
| Super Taiwanese Baseball League | menús, partido con marcador | usa la SRAM de batería (1849 bytes grabados) |
| The Son of Evil | título, Start lleva a la escena del lago | — |

**Aún no verificado:** fidelidad del sonido (solo se comprobó que hay música y efectos), juegos hasta el
final, modo de 2 jugadores, comparación lado a lado con MiSTer/consola real.

## 2026-10-21 (2.ª ronda)

Tras corregir la velocidad del 65C02 (corría ~3,5% rápido): mismo guion aleatorio de
20 000 cuadros, `faults=0` y `halted=false` en los 12. Novedades vistas: Magical Pool llega a la mesa
de billar; Gambling Lord a la mesa de mahjong con los cuatro jugadores. Speedy Dragon sigue bien: con la
velocidad correcta basta 1 ciclo de espera en la RAM de sonido (V-Bear usa 2; con 0 el driver se
recarga sin fin). Savestate (F5/F9) probado: restaurar y ejecutar 600 cuadros da una pantalla idéntica
en los 12 juegos.

## 2026-10-21 — comparación con MAME 0.289

Herramienta: `tools/mame/compare.sh <jogo.zip>` (fuera de git; ver `references.es.md`). Captura los
mismos cuadros en los dos emuladores, arma las imágenes lado a lado y cuenta los píxeles distintos (MAME
muestra 224 líneas; V-Bear tiene 8 más arriba y abajo).

- **Imagen:** en las escenas en que los dos están en el mismo punto, las pantallas coinciden. Speedy Dragon,
  título en el cuadro 1800: 99,2% de los píxeles iguales, y la diferencia es solo el cuadro de la animación del personaje.
  Formosa Duel, título: 95,6%, la diferencia es la posición de las nubes. Logos (Funtech, Panda, AV
  artisan), títulos de Boom Zoo, Gambling Lord, Monopoly, Super Dragon Force, Journey to the Laugh y el
  estadio de Super Taiwanese Baseball verificados a ojo, iguales.
- **Sonido:** Speedy Dragon, 6 s de música del título en espectrograma: mismas notas, timbres y tempo
  (el patrón se repite cada 1,6 s en ambos); volumen del mismo orden. La música empieza ~0,5 s antes
  en V-Bear.
- **Secuencia:** diverge, porque MAME avanza solo por títulos y menús sin ninguna entrada (sus
  puertos quedan en reposo; causa no investigada, ver `references.es.md`). Ejemplos sin entrada:
  en MAME, Sango Fighter corta la introducción en el cuadro ~1100 y entra en una lucha; Formosa Duel va
  a la elección de adversarios; Rebel Star y The Son of Evil quedan con la pantalla negra o con
  basura. En V-Bear los juegos se quedan en el título y las demostraciones, alternando, sin nadie jugando.

## 2026-10-21 — savestate en disco

`V_BEAR_SNAPTEST=900 … --frames 2400`: graba el estado en el formato del archivo en el cuadro 900, ejecuta hasta
2400, carga en una máquina nueva, ejecuta de nuevo y compara la imagen y el estado completo (byte a byte).
Idéntico en los 12 juegos; cada estado tiene ~260 KiB. Una prueba unitaria cubre el 65C02 detenido en WAI y el
flanco del NMI. Las teclas F5/F6/F7/F9 en la ventana aún no se probaron a mano.

## 2026-10-21 — teclas de uso

Reset (F1) verificado sin ventana: Sango Fighter reiniciado en el cuadro 600 y ejecutado 1800 más da la
misma pantalla (píxel a píxel) y los mismos contadores de IRQ que 1800 cuadros desde el arranque. PNG de la pantalla
idéntico a la captura PPM. Pausa, avance de cuadro, Tab y F12 en la ventana aún no probados a mano.

## 2026-10-21 — migración a la interfaz de la base

El núcleo pasó a `crates/v-bear-core` sin cambiar el comportamiento: con el mismo guion de botones,
la pantalla en el cuadro 2400 es idéntica, byte a byte, a la de la versión minifb en los 12 juegos. Según el contrato
(`cargo test -p v-bear-core --test contract`, con `V_BEAR_TEST_ROM` y `V_BEAR_TEST_BIOS`): Super
Dragon Force (dos partes), Sango Fighter y Rebel Star identificados en la base de datos, con imagen, audio a la
tasa pedida y save state exacto. Core libretro: carga Sango Fighter con la BIOS en la carpeta
*system* (prueba `real_core` de `libretro-host`). Interfaz: capturas automáticas (`EMU_DOCSHOTS`)
con Sango Fighter en marcha, la ficha del juego y las carpetas nuevas (BIOS, saves de batería). Aún no
probado a mano en la interfaz: controles, rewind, grabación, save de batería.

## 2026-10-21 — save de batería (SRAM), versión 0.5.0

**Hardware:** una SRAM 62256 de 32 KiB (U2) en la placa PCCAR 16003-1B, en `$EC0000-$ECFFFF`, solo en la mitad
baja del bus: el byte `n` queda en `$EC0000 + 2n + 1`. El MiSTer suelda la SRAM en todos los
cartuchos; el softlist de MAME declara `nvram` de 32 KiB en 6 juegos (algunos con "tamaño y presencia no
confirmados"). El `.sav` de V-Bear tiene 32 768 bytes en ese orden, el mismo del `nvram` de MAME y del `.sav`
del MiSTer.

**Accesos medidos** (contadores `reads`/`writes` en el resumen del modo sin ventana, 3600 cuadros con Start/A):

| Juego | SRAM en MAME | Lecturas | Escrituras | Observación |
|---|---|---|---|---|
| Super Taiwanese Baseball League | sí | 29 968 | 2 162 | formatea al encender; con el `.sav` grabado, lee 10 bytes y no reformatea |
| Super Dragon Force | sí | 24 240 | 0 | busca saves ya en el título |
| Rebel Star | sí | 1 (al elegir LOAD GAME) | 0 | sin save, vuelve al título |
| The Son of Evil | sí | 0 | 0 | guardar y cargar confirmados a mano el 2026-10-30 |
| Monopoly | sí | 0 | 0 | LOAD GAME abre 5 saves y empieza un juego sin tocar `$EC` ni ninguna dirección fuera del mapa (`V_BEAR_DEBUG=unmapped`); dónde guarda el progreso es una pregunta abierta |
| Gambling Lord | sí (presencia no confirmada) | 0 | 0 | |
| los otros 6 | no | 0 | 0 | |

**Ida y vuelta (béisbol):** `V_BEAR_SAV=x.sav` en el modo sin ventana lee el archivo antes y lo graba después.
Primera ejecución: lee 10 bytes, formatea (2 162 escrituras), graba `x.sav`. Segunda: lee los mismos 10
bytes, 0 escrituras, la misma pantalla en el cuadro 3000.

**Corregido:** la interfaz buscaba el `.sav` solo en la carpeta de saves (`~/.config/v-bear/saves/`); lo que la
versión antigua dejó junto a la ROM era ignorado. Ahora, sin el archivo en la carpeta de saves, usa el
`<juego>.sav` de la carpeta del juego y lo copia a la carpeta de saves en la próxima grabación (verificado: copia
idéntica).

**Para los testers** (lo que solo se comprueba jugando):
1. Guardar y cargar en cada juego con SRAM: Son of Evil, Super Dragon Force y Rebel Star (puntos de
   guardado), Monopoly (5 saves), Gambling Lord, béisbol (temporada).
2. Cerrar V-Bear justo después de guardar y comprobar que el save sigue ahí (el `.sav` se graba hasta ~5 s
   después del cambio, al cerrar el juego y al salir).
3. Traer un `.sav` del MiSTer o el `nvram` de MAME y ver si el juego lo reconoce.

## 2026-10-30 — core de RetroArch y saves

Core compilado para Linux (glibc 2.27), Windows, FreeBSD y macOS (solo compiló; FreeBSD y macOS no
fueron ejecutados). Se probó el `.so` de Linux con un programa que simula RetroArch (ctypes):

- BIOS encontrada suelta en `system/`, en `system/supracan/`, en un `.zip` de 4 archivos y en el `supracan.zip`
  de MAME con el `umc6650.bin` suelto al lado. El `supracan.zip` de MAME 0.289 solo tiene 3 archivos
  (sin la clave `umc6650.bin`), así que no basta. Según un relato en Reddit (no verificado aquí), el
  zip más antiguo traía `internal_68k.bin` y `umc6650.bin`, y el más nuevo cambió el segundo por los
  dos `internal_6502`: en ambos casos faltan archivos para V-Bear.
- Save state: serializa 267 KB y restaurar da el mismo cuadro en los juegos probados (Boom Zoo, béisbol).
- Save de batería: el core entrega 32 KB; en el béisbol el juego graba 1849 bytes y la SRAM vuelve idéntica
  en una segunda sesión. Son of Evil, Super Dragon Force, Rebel Star, Monopoly y Gambling Lord quedan
  con la SRAM vacía en 3000 cuadros sin botones (solo graban al guardar de verdad).
- Confirmado a mano: **The Son of Evil** guarda y carga. Super Dragon Force, Rebel Star, Monopoly
  y Gambling Lord siguen por probar.
- Los 10 `.bin` verificados tienen el CRC32 de No-Intro.

## 2026-10-30 — comparación con el core MiSTer simulado

Los 12 juegos se compararon con el core MiSTer de la Super A'Can, simulado con Verilator (carpeta
`reference/`), en el cuadro 300, sin presionar ningún botón, con la misma BIOS en ambos lados. Método, números y
límites en [accuracy.es.md](accuracy.es.md).

- **Imagen idéntica, píxel a píxel y color a color, en 10 de los 12 juegos.** Gambling Lord y Sango Fighter
  difieren en algunos cuadros: los canales de color de un fade se actualizan en cuadros distintos de un ciclo de 4 cuadros, y la imagen es idéntica cada cuarto cuadro (causa no encontrada).
- **Hallazgo y corrección:** la ventana 0 usaba la entrada `L−1` de la tabla para recortar la línea `L`, y el círculo
  creciente de Magical Pool salía una línea más abajo. El MiSTer (`render_y = vpos + 1`) y MAME 0.289
  concuerdan entre sí contra V-Bear; corregido en `render.rs`. Antes: 76 píxeles distintos en el cuadro 300;
  después: 0.
- **El core MiSTer no es la consola.** Se hizo por ingeniería inversa (videos de hardware real, desensamblados),
  sin prueba en consola, y el repositorio tiene un único commit. "Idéntico al MiSTer" es la concordancia de dos
  reconstrucciones, no una prueba.
- **RAM:** idéntica en Rebel Star, con como máximo 13 bytes distintos en otros 6 juegos; Magical Pool,
  Speedy Dragon y Super Dragon Force tienen diferencias aún no investigadas.
