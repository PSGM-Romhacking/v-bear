# Precisión y validación

[English](accuracy.md) · [Português](accuracy.pt-BR.md) · **Español**

Cómo se compara V-Bear con otras implementaciones de la Super A'Can, el resultado y lo que aún falta.
**V-Bear no se ha comparado con una consola real**, y las referencias con las que se comparó son, ellas
mismas, reconstrucciones: lea "Límites de la referencia" antes de las cifras.

Las referencias entre corchetes ([n]) están en [Referencias](references.es.md).

## Cómo se hace la comparación

**1. Las dos "consolas"**
- **La referencia:** el core MiSTer de la Super A'Can es la descripción del circuito de la placa (en
  SystemVerilog). Un programa (Verilator) simula ese circuito en el PC, ciclo de reloj a ciclo de reloj.
  Es lento, pero es el propio core MiSTer en ejecución.
- **V-Bear:** el emulador, escrito en Rust, rápido. Cuando MAME y el core MiSTer discrepan, sigue al core
  MiSTer ([Hardware](hardware.es.md)).

Si V-Bear está en lo correcto, los dos deben mostrar **lo mismo** en el mismo instante.

**2. El mismo juego**

Ambos ejecutan la misma ROM con la misma BIOS, sin que nadie juegue: no se pulsa ningún botón. Por eso lo
que se compara es el inicio de cada juego, hasta el cuadro 300 (unos 5 segundos), no menús ni partidas.

**3. Pausa y captura en el mismo cuadro**

Tras N cuadros, ambos se detienen y guardan:
- **la imagen:** en su tamaño real (256 o 320 puntos de ancho, 224 o 240 líneas);
- **la work RAM:** los 64 KB que usa el 68000.

**4. Comparar, punto por punto**

Un script compara los archivos: cuántos píxeles tienen un color distinto (0 = imagen idéntica) y cuántos
bytes de la RAM tienen un valor distinto.

**5. Los dos nunca arrancan a la misma velocidad**

El mismo juego llega a la misma imagen con algunos cuadros de diferencia (de 0 a 13 aquí, y varía de un
juego a otro), porque la simulación lee el cartucho con un retardo fijo que no es el real. Por eso el
cuadro N de la simulación se compara con una ventana de cuadros de V-Bear, y se conserva el mejor: el que
tiene menos píxeles distintos y, entre empatados, el que tiene menos bytes de RAM distintos. El
desplazamiento encontrado forma parte del resultado (`LAG`). La consecuencia es que una imagen quieta
prueba poco sobre la alineación, y es la RAM la que decide.

**6. Cuando sale distinto: mirar dónde**

Una diferencia se mide, no se adivina. Se examinan el rectángulo que contiene lo que difiere, los valores
de los píxeles y, cuando la causa no está clara, una tercera fuente (MAME), antes de llamarlo error.

## Límites de la referencia

Esta es la parte que más importa para saber cómo leer las cifras.

- **El core MiSTer es una reconstrucción, no la consola.** Su README ("Development and QA") dice que se
  construyó mediante ingeniería inversa: videos de hardware real que muestran las pantallas de demostración
  de los juegos (grabados por Diskman), desensamblados (disassemblies) de toda la biblioteca y documentación
  anterior. Su autor no lo probó contra una consola: la consola cuesta miles de dólares y no existe flash
  cart. El mismo README dice que la biblioteca funciona "apparently bug free".
- **Por tanto, "idéntico al core MiSTer" significa que dos reconstrucciones independientes coinciden.** Es
  un indicio de que ambas están en lo correcto, no una prueba. Si coinciden en un error, esta comparación no
  lo ve. Si discrepan, no dice cuál se equivoca: hizo falta un tercer lado (abajo).
- **El repositorio del core es nuevo:** un commit (2026-09-11, `59ba275`), sin historial de correcciones.
- **La simulación es nuestra y tiene sus propios límites:** el cartucho se lee con un retardo fijo (una
  suposición); no se pulsa ningún botón; no se compara sonido, ni save state, ni save de batería; funciona
  a aproximadamente un cuadro por segundo. Detalles en [`reference/README.md`](../reference/README.es.md).

## Metodología (detalles técnicos)

1. **Referencia:** el SystemVerilog del core MiSTer simulado ciclo a ciclo con Verilator,
   en [`reference/`](../reference/README.es.md), sin modificar el código del core: `sim_top.sv` sustituye
   al framework de MiSTer (reloj, cartucho como un array, carga de la BIOS) y `sim.h` captura la imagen de
   la salida de video del core y lee la work RAM por su puerto de savestate.
2. **Misma entrada:** ninguna (ningún botón pulsado), BIOS cargada de los mismos cuatro archivos.
3. **Misma salida:** la imagen y los 64 KB de work RAM en el cuadro N.
4. **Normalización de la imagen:** V-Bear siempre produce 320×240 (líneas del H256 estiradas; el modo de 224
   líneas centrado, con 8 líneas negras arriba y abajo) y la simulación produce el tamaño real; la
   comparación aplica la conversión de `video.rs` al cuadro de la simulación y compara colores exactos
   (8 bits por canal).
5. **Alineación:** una ventana de ±12 cuadros de V-Bear en torno al esperado, como se describió arriba.
6. **Comandos:**

```bash
make -C reference
FRAMES=300 reference/compare.sh --jobs 6 --window 12 --cache /tmp/ref-cache target/release/v-bear roms/*.zip
```

7. **Tercera fuente:** MAME 0.289 (Flatpak `org.mamedev.MAME`), controlado por `tools/mame/drive.lua`, con
   la captura de cada cuadro. Se usó para decidir la diferencia de la ventana 0 que se describe abajo.

## Resultado

En el cuadro 300, sin ningún botón pulsado, 12 juegos, core MiSTer simulado (`LAG` es el desplazamiento de
V-Bear en cuadros, y no es fiable en una imagen quieta):

| Juego | LAG | Píxeles distintos | Bytes de RAM distintos |
|---|---|---|---|
| Boom Zoo | 5 | **0** | 1 |
| Formosa Duel | 9 | **0** | 9 |
| Gambling Lord | 2 | 5.629 | 124 |
| Journey to the Laugh | 9 | **0** | 2 |
| Magical Pool | 3 | **0** | 451 |
| Monopoly: Adventure in Africa | 2 | **0** | 1 |
| Rebel Star | 0 | **0** | **0** |
| Sango Fighter | −2 | 10.643 | 52 |
| Speedy Dragon | 6 | **0** | 37 |
| Super Dragon Force | 16 | **0** | 868 |
| Super Taiwanese Baseball League | 2 | **0** | 13 |
| The Son of Evil | −4 | **0** | 6 |

- **Imagen idéntica, píxel por píxel y color por color, en 10 de 12 juegos.**
- **RAM:** idéntica en Rebel Star; como máximo 13 bytes distintos de 65.536 en otros 6 (Boom Zoo, Formosa
  Duel, Journey, Monopoly, Baseball, The Son of Evil).
- **También comparado, fuera de esta tabla:** la introducción de la BIOS (el logo "A'can"), cuadros 100 a
  125 de Boom Zoo, es idéntica en los 26 cuadros (0 píxeles de 71.680 a 5 bits por canal de color;
  comprobado a 8 bits por canal solo en el cuadro 120), con un desplazamiento constante de 1 cuadro.

**Lo que hubo que corregir para llegar hasta aquí:**

| Problema | Dónde | Efecto antes de la corrección |
|---|---|---|
| El recorte de la ventana usaba la entrada L−1 de la tabla para la línea L (`render.rs`, `window`) | video | una forma dibujada por la ventana 0 (el círculo creciente de Magical Pool) una línea más abajo de lo correcto: 76 píxeles en un anillo en el cuadro 300, 122 y 176 en los cuadros 310 y 315. Tras la corrección: 0 píxeles en los cuadros 290, 300 y 305 (3, 23 y 61 en 295, 310 y 315, no investigado) |

**Cómo se decidió el caso de la ventana.** V-Bear leía la entrada de la tabla de la línea anterior porque
el código del core MiSTer dice "the pair fetched during line L-1 clips line L". El `render_y` del core ya
es `vpos + 1`, así que lo que busca durante la línea L−1 es la entrada L. MAME indexa la tabla por la
propia línea. En la simulación el círculo abarca las filas 101 a 138 (centro 119,5) y V-Bear lo dibujaba en
102 a 139 (centro 120,5); con V-Bear desplazado una línea hacia atrás, la diferencia pasó de 76 píxeles a
4. El círculo en las capturas de MAME también está centrado en la fila 119,5, en los cuadros medidos (298 y
301). Dos referencias independientes frente a V-Bear.

## Qué sigue siendo distinto

- **Sango Fighter (imagen).** El logo aparece con un fade con sus tres canales de color (R, G, B), cada uno
  subiendo un nivel en un cuadro distinto de un ciclo de 4 cuadros. Medido en los cuadros 290 a 305 (mismo
  número de cuadro en ambos lados, media de cada canal): en la simulación los pasos ocurren en los cuadros
  291 (G), 292 (R) y 294 (B), y cada 4 cuadros después; en V-Bear, en 293 (G), 294 (R) y 292 (B). Cada cuarto
  cuadro (290, 294, 298, 302, 306, 310, 314) los tres canales tienen el mismo valor en ambos lados y la
  imagen es **idéntica, 0 píxeles, con lag 0**; en los otros tres cuadros de cada ciclo difieren de 9.000 a
  12.600 píxeles, en 1 nivel de uno o más canales. Es decir, ambos producen el mismo fade, y lo que difiere
  es **en qué cuadro del ciclo se actualiza cada canal**. No se encontró la causa: el propio código del
  juego, la fase de un temporizador o de una interrupción, o el retardo fijo de lectura del cartucho en la
  simulación podrían producirla.
- **Gambling Lord (imagen).** Idéntica en 8 de los 26 cuadros de 290 a 315 (292, 294, 298, 302, 306, 309, 310
  y 314) y de 1.900 a 5.600 píxeles en los otros, con el mismo tipo de diferencia (1 o 2 niveles de un canal
  de color). No analizada canal por canal.
- **Magical Pool, Speedy Dragon y Super Dragon Force (RAM).** Difieren 451, 37 y 868 bytes con la imagen
  idéntica. No investigado. Super Dragon Force viene en dos archivos de ROM (dos mask ROM); una ejecución
  anterior de la comparación usó solo uno de ellos (un error de `compare.py`, corregido), y su cifra se
  descartó. Los 868 son con los 3 MB completos y una ventana de ±40 cuadros (la mejor alineación, 16, está dentro).
- **Unos pocos bytes en la RAM de los demás juegos** (1 a 13): en Boom Zoo en el cuadro 120, 9 bytes: 2
  contadores con un paso de diferencia y 7 bytes en la cima de la pila del 68000. Probablemente porque los
  dos lados se detienen en puntos ligeramente distintos del cuadro (no probado; la RAM se comparó solo en un
  punto del cuadro).
- **Cuadros posteriores, menús y partidas.** No se comparó nada más allá del cuadro 300 y sin botón
  pulsado, salvo las pruebas contra MAME en [Compatibilidad](compatibility.es.md).

## Clasificación

V-Bear **no** declara un nivel en la escala de la Emulation General Wiki: no se evaluó, y la referencia con
la que se comparó no es la consola. Lo que se puede decir se limita a "comparado con el core MiSTer
simulado, en los casos anteriores, y con MAME 0.289 en las escenas listadas en
[Compatibilidad](compatibility.es.md)".

## Validación contra la consola

Ninguna. No hay mediciones hechas en una Super A'Can real, ni por nosotros ni por otros, ni ROMs de prueba
con fotos de una consola real. La validación del propio core MiSTer (ingeniería inversa a partir de videos)
es lo más cercano que existe, y está descrita arriba.

## Estado

| # | Elemento | Estado |
|---|---|---|
| 1 | Ventana 0 recortada por la entrada equivocada de la tabla (una forma una línea más abajo) | **hecho:** Magical Pool, 0 píxeles en los cuadros 290, 300 y 305 |
| 2 | Sango Fighter y Gambling Lord: los canales de color de un fade se actualizan en cuadros distintos del ciclo | pendiente: causa desconocida; la imagen es idéntica cada 4 cuadros |
| 3 | La RAM difiere en Magical Pool, Speedy Dragon y Super Dragon Force | pendiente: no investigado |
| 4 | Comparar después del cuadro 300 y con botones pulsados | pendiente: la simulación aún no tiene entrada |
| 5 | La simulación lee el cartucho con un retardo fijo (una suposición) | pendiente: un retardo distinto cambiaría la alineación, no el resultado |
| 6 | Comparar el sonido | pendiente: la simulación no tiene salida de sonido |
| 7 | Comparar con una consola real | ningún cambio posible: no hay ninguna que medir |

## Cómo se validó cada parte

Cada parte de la emulación, con su prueba. Solo el core MiSTer simulado y MAME sirven de referencia; ninguno
de los dos es la consola.

| Parte | Prueba | Fuerza |
|---|---|---|
| Imagen (tiles, sprites, fondos, prioridades, paleta) | 0 píxeles distintos frente al core MiSTer simulado en 10 de 12 juegos en el cuadro 300; intro de la BIOS idéntica en 26 cuadros | fuerte, en los primeros 300 cuadros |
| Ventanas y tablas por línea | círculo de Magical Pool: 0 píxeles en los cuadros 290, 300 y 305, decidido con la simulación y con MAME | fuerte para la ventana 0; las otras ventanas no se aislaron |
| 68000 y 65C02 (crates `m68k` y `mos6502`) | indirecta: los juegos funcionan y la RAM es idéntica en Rebel Star y difiere en hasta 13 bytes en otros 6 | débil: no se ejecutó ninguna suite de pruebas de instrucciones |
| Interrupciones y temporización | misma RAM e imagen que la simulación; el fundido de Sango Fighter y Gambling Lord **no** es idéntico | parcial |
| Sonido | ninguna contra referencia: la simulación no tiene salida de audio | ninguna |
| Controles | escenas jugadas contra MAME 0.289, listadas en [Compatibilidad](compatibility.es.md) | parcial |
| Guardado de batería | guardado y cargado a mano en 2 de los 6 juegos con batería | parcial |
| Juego en dos partes (Super Dragon Force) | archivo unido con CRC32 `c4fe1ca1`, el que consta en la base de juegos (`games.rs`) | fuerte |

## Cada elemento en detalle

**2. Sango Fighter y Gambling Lord.** Para encontrar la causa, registrar las escrituras del 68000 en la
paleta (`$F00200` a `$F003FF`: cuadro, línea, entrada, valor) en V-Bear y en la simulación, y ver en qué
cuadro se escribe cada canal. Si el juego los escribe en el mismo cuadro en ambos lados y solo la imagen
difiere, es una cuestión de renderizado; si los escribe en cuadros distintos, la diferencia está en la
temporización que conduce el juego (una interrupción o un temporizador), en V-Bear o en la simulación.

**3. La RAM.** La simulación se detiene al final de un cuadro y V-Bear al final del suyo; un juego que
mantiene un contador o una tabla que cambia durante el cuadro diferirá en unos pocos bytes en cualquier
alineación. Para separar eso de una diferencia real, comparar la RAM en varios puntos del cuadro y listar
qué direcciones difieren.

**4. Botones.** `--press-start` en la comparación de Mil Capivaras pulsa Start en cuadros fijos en ambos. La
simulación necesitaría los mandos conectados en `sim_top.sv` (la entrada `pad` ya existe) y un guion de
pulsaciones en `sim.h`; V-Bear ya tiene `V_BEAR_INPUT`.

## Orden sugerido

El elemento 1 está hecho. Lo que falta, del más barato al más caro:

| # | Elemento | Esfuerzo | Ganancia |
|---|---|---|---|
| 3 | RAM en Magical Pool, Speedy Dragon y Super Dragon Force: comparar en varios puntos del cuadro | pequeño | separa una diferencia real de un efecto del punto de parada |
| 2 | Sango Fighter y Gambling Lord: registrar las escrituras en la paleta en ambos lados | medio | encuentra (o descarta) un error de temporización en V-Bear |
| 4 | Botones en la simulación | medio | extiende la comparación más allá de los primeros 300 cuadros |
| 5 | Retardo del cartucho en la simulación | pequeño | solo cambia la alineación |
| 6 | Sonido | grande | exige una salida de audio en la simulación |
