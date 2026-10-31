# Super A'Can — hardware

[English](hardware.md) · [Português](hardware.pt-BR.md) · **Español**

Referencia técnica para V-Bear. Las citas [n] apuntan a `references.es.md`.
Primera versión: 2026-10-20 (MAME `60dbfc6`, MiSTer `59ba275`).

Leyenda: **[C]** confirmado en el código · **[D]** documentación/wiki · **[?]** incierto o desconocido ·
**⚠** MAME y MiSTer divergen.

**Regla de desempate de V-Bear:** cuando MAME y MiSTer divergen, seguir a MiSTer (modela la placa
entera y ejecuta toda la biblioteca), registrando aquí la divergencia.

---

## 1. Visión general e historia

- Consola de 16 bits lanzada solo en Taiwán y China el 25/10/1995, descontinuada el 04/04/1996;
  NT$2.900; nombre en clave "F-16" [31].
- Funtech (Dunhuang Technology, subsidiaria de UMC) en Taiwán y Sino Wealth en China. UMC fabricó los
  chips UM6618 (video) y UM6619 (sistema + sonido). Panda Entertainment fue la principal tercera parte [31][39].
- Fracasó por el precio y por la competencia 3D. Se dice que Funtech perdió ~US$6 millones y destruyó
  el equipo de producción. CD-ROM y mejora de CPU/gráficos planificados y nunca lanzados [31].
- **12 juegos lanzados, todos volcados (dumped)**; hay inéditos conocidos (Dinosaur War, City Escape, Quick
  Fighting Attack, Journey to the Center of the Earth…) [5][31].
- 2026: Piko Interactive compró IPs de la consola (solo fragmento de búsqueda; la página dio 403) [40][?].

## 2. Especificaciones

| Ítem | Valor | Fuente |
|---|---|---|
| Cristal U13 | 53,693175 MHz (MCLK); va solo al UM6618, que genera el CPU_CLK [C] | [1][7][15][30] |
| CPU principal | MC68000P10. ⚠ **MiSTer: MCLK/5 = 10,738635 MHz** [C][10], igual que Wikipedia [31]. MAME: MCLK/6 = 8,949 MHz [C][1] | |
| CPU de sonido | 65C02 **integrado en el UM6619** (no es un chip separado). ⚠ **MiSTer: CPU_CLK/3 = 3,579545 MHz** [12] (coincide con los "3,58 MHz" de Giant Bomb [39]). MAME: MCLK/12 ≈ 4,47 MHz, con el comentario "/4 hace fallar speedyd" [1] | |
| Work RAM | 64 KiB: 2× 32K×8, uno por byte lane [C][7][30] | |
| Sound RAM | U11, **64 KiB** tal como la ven el 65C02 y el 68000: MiSTer la instancia con 16 bits de dirección (`sram_async #(.ADDR_WIDTH(16))` en `mainboard.sv`) [7] y MAME modela 64 KiB planos [1]. **Corrección (2026-10-20):** la primera versión de este documento decía 32 KiB; los drivers de sonido están en `$E800-$FFFF` y los datos en `$8400+`, que colisionarían con 32 KiB | |
| VRAM | 128 KiB: 2× UM61512 64K×8 [7][30] | |
| Paleta/DAC | UM70C188: 256 entradas xBGR555 (32.768 colores), compatible con UM70C171 [27][1] | |
| Codificador | KA2195D, RGB→NTSC; salidas RF y compuesto [7][31] | |
| Resolución | 256 o 320 × 240 (o 224 con el bit de overscan); 262 líneas en total; ~59,92 Hz [17][1] | |
| Video (UM6618) | 4 tilemaps, 1 capa ROZ, sprites, 2 ventanas de clip. Mezcla en espacio de índice de paleta; translucidez = suma de índices [15][21] | |
| Sprites | Hasta 128 entradas en MiSTer (SPRITE_MAX), con escala y mosaico [20]. hw_docs: 32/línea y 320/cuadro [29][?]; Wikipedia: 40/línea, hasta 256×256 px [31][?] | |
| Sonido (UM6619) | 16 canales PCM de 8 bits sin signo, estéreo, salida de 44.744 Hz (reloj de sonido /80) [2][13] | |
| Cartucho | Hasta 4 MB (2 mask ROMs de 16 Mbit) + SRAM de batería de 32 KiB + lockout UM6650 [24] | |

## 3. Mapas de memoria

### 68000

MAME [1] y MiSTer [8][15] coinciden, salvo donde se indica.

| Rango | Contenido |
|---|---|
| `$000000-$3FFFFF` | ROM del cartucho. Los primeros 4 KiB quedan superpuestos por el **IPL interno** hasta el bit 1 de `$E9001C` (latch de una sola vía) [1][11] |
| `$E80000-$E8FFFF` | Ventana de la sound RAM (bytes del 6502, big-endian). ⚠ En MAME, `$E80200/$E80202` leen los controles directamente [1]; no existe en MiSTer |
| `$E90000-$E9001F` | Host port del UM6619 (sección 6) |
| `$E90020-$E9002F` / `$E90030-$E9003F` | Canales de DMA 0 y 1 |
| `$E90B3C` | "Easter egg": el IPL escribe 9 constantes (nombre chino de UMC) y lee una vez [8] |
| `$EB0D01` (datos) / `$EB0D03` (dirección) | Lockout UM6650, byte bajo [3][24] |
| `$EC0000-$ECFFFF` | SRAM del cartucho, solo en el byte impar (32 KiB en una ventana de 64 KiB) [24][4] |
| `$F00000-$F001FF` | Registros del UM6618: 256 palabras, todas legibles, **solo acceso por palabra** (el chip no tiene UDS/LDS) [16] |
| `$F00200-$F003FF` | Paleta: 256 × xBGR555 [1][15] |
| `$F40000-$F5FFFF` | VRAM 128 KiB |
| `$F80000-$FBFFFF` | Espejo de 256 KiB de la ROM; primeros 4 KiB superpuestos por el IPL hasta el bit 3 de `$E9001C` [1][11][24] |
| `$FC0000-$FFFFFF` | Work RAM 64 KiB, espejada cada 64 KiB [1][8] |

- Los accesos del 68000 a la ventana de la sound RAM (`$E80000`) pasan por el árbitro del UM6619, donde el 65C02 tiene
  prioridad: son dos ciclos de byte y generan espera [8][12]. V-Bear cobra 2 ciclos de CPU por acceso;
  con el 65C02 a CPU_CLK/3 exacto, 1 ciclo ya basta (V-Bear usa 2 de margen); sin ninguna espera, Speedy Dragon pierde la carrera entre su timeout (0xC000 vueltas de 44 ciclos ≈ 0,201 s) y
  la inicialización del driver (≈ 0,207 s, con la detección de control por timeout) y recarga el driver en bucle.
- Sin bus error: `!BERR` atado a alto. Un acceso sin respuesta cuelga la máquina, por eso el UM6619
  responde a cada ciclo [8].
- Sango Fighter escribe 32 palabras más allá del final de la paleta [8].
- IACK por autovector (`!VPA`) [8].

### 65C02

Sound RAM (64 KiB), con la página de I/O en `$0400-$04FF` decodificada en los 16 bits [1][12].

| Dirección | Función |
|---|---|
| `$0000-$3FFF` | En el reset, recibe la copia de la **imagen interna de 16 KiB** (dos ROMs de 8 KiB): vectores + driver [1][12] |
| `$0402/$0403` | Shift registers de los pads 1 y 2 |
| `$0404/$0405` | Latches de bytes enviados por el 68k. Leen `$CD` cuando están vacíos; limpian IRQ bit 3/bit 2 [12] |
| `$0406` | Lee 0 (posible presencia de pad) |
| `$0407` | Control de los pads (flanco de bajada): bits 1-0 latch, 3-2 shift, 5-4 clear [1][12] |
| `$0409` | Ack del IRQ bit 4 |
| `$040A` | Escritura: IRQ6 al 68k (mailbox). Lectura: ack del IRQ bit 5, activado por el 68k en `$E9000A` [1][12]. ⚠ Un comentario en `um6619_irq.sv` dice "$0300" [9]; el código usa `$040A` |
| `$040C/$040D` | Bytes que el 68k lee en `$E90004/5` (pedido de DMA de sonido) |
| `$0410` | Habilitación de IRQ |
| `$0411` | Estado de IRQ. ⚠ MAME: la lectura limpia todo; MiSTer: no limpia nada [12] |
| `$0412` | Ack de NMI |
| `$0420` / `$0422` | Dirección / dato de los registros del sampler; lectura de `$0420` = estado [1][12] |

- Bits de IRQ del 6502: `$40` DMA de sonido, `$04`/`$08` latches, `$10`, `$20` pedido del 68k, `$80` timer de sonido [1].
- La NMI viene del vblank, condicionada al bit 7 de `$E90010` [1][12].
- hw_docs lista IRQs de mouse en los puertos DE-9 [29][?].

## 4. Video (UM6618)

### Registros globales [1][16][29]

- `$00`: estado. Bit 15 = vblank (MAME usa el bit 1 como cuadro impar).
- `$02`: scanline. `$04`: punto en la línea (MiSTer lo implementa; Boom Zoo y Baseball lo leen).
- `$08`: flags de video — bit 8 H320 · bit 9 overscan/224 líneas · bit 10 entrelazado [?] ·
  bits 7-4 habilitación de los tilemaps 0-2 (y 3) [?] · bit 3 sprites · bit 2 ROZ · bit 1 ventana 0 ·
  bit 0 ventana 1 (MiSTer [22]).
- `$0A` / `$0C`: comparadores de raster "line on"/"line off" (bit 15 habilita, bits 7-0 línea) → IRQ5.
- `$10-$1E`: DMA de sprites — cuenta, destino+incremento, origen+incremento, disparo en el bit 15.
  Bit 8 = rellena con cero; bits 13/14 fuerzan el destino en la VRAM. MiSTer devuelve los punteros finales [15].
- `$20`: base de la tabla de sprites (<<2) · `$22`: cantidad − 1 · `$24`: color mono ·
  `$26`: bit 0 = 8bpp (1) / 4bpp (0).
- `$1F0`: modo de píxel (bits 4-3) y modo GFX (bits 2-0), que define la profundidad de las capas 0 y 1.
  ⚠ hw_docs: 000 = 2bpp en todas; 001 = 4/4/2; 010 = 8/4; 011 = 4+linescroll; 100 = 8+linescroll [29].
  LUTs de MAME: `{2,1,0,1,0…}` [1]. En MiSTer las capas 2 y 3 son siempre 2bpp [15][18].

### Tilemaps

Bloques de 8 palabras en `$100/$120/$140/$160` [1][18][29]. ⚠ MiSTer implementa los **4**; MAME solo 3
(TODO en `$F00160`).

- `+0` flags: bits 15-13 prioridad · bits 11-8 tamaño del mapa (2 = 16×16, 4 = 32×32, 6 = 64×32,
  A = 128×32, C = 64×64 tiles) · bit 8 tiles 16×16 [?] · bit 6 **mezcla aditiva** (MiSTer [18]) ·
  bit 5 wrap · bits 4-2 mosaico · bits 1/0 flip global X/Y (XOR en el scroll).
- `+2` modo: bit 14 tabla de linescroll X · bit 11 tabla de line-select · bit 9 prioridad por tile ·
  bit 8 una entrada por línea, de lo contrario la misma para todas (MiSTer).
- `+4` / `+6` scroll X/Y. ⚠ MiSTer: X 11 bits con signo (por causa de Magical Pool), Y 12 bits; MAME 12/12.
- `+8` base del mapa · `+A` modo/banco de tiles (bits 14-12) · `+C` tabla de linescroll ·
  `+E` tabla de line-select.
- Entrada de tile: bits 15-12 paleta · 11 flip X · 10 flip Y · 9-0 tile. En 2bpp la paleta avanza de 4 en 4.
- Tiles 8×8 lineales empaquetados: 8bpp = 64 B, 4bpp = 32 B, 2bpp = 16 B [1][18].

### ROZ (`$180-$19E`) [19][1][29]

- `$180` modo: bits 1-0 profundidad (0 = logo 1bpp del BIOS, 1 = 2bpp, 2 = 4bpp, 3 = 8bpp o bitmap
  lineal vía `$1F0`) · bit 5 wrap · bits 15-13 prioridad · bit 6 aditivo.
- `$182` control: bit 15 tabla Y · bit 6 tabla de escala `$198` · bit 8 dos palabras X por línea.
- `$184-$18A`: origen X/Y, 24 bits con signo, 8 bits fraccionarios.
- `$18C-$192`: coeficientes A/B/C/D en 8.8. MiSTer:
  `srcX = (scrollX + y·B + x·A) >> 8`, `srcY = (scrollY + y·D + x·C) >> 8`.
- `$194` base del mapa · `$196` banco (16 KiB por banco).
- Tablas por línea: `$198`, `$19A`, `$19C` (leída y descartada), `$19E`. Cero en la 1ª tabla apaga la línea.
- MAME trata estas tablas como "unk_base", con HACK [1].

### Sprites (4 palabras por entrada) [1][20]

- W0: bits 12-9 altura en tiles (LUT `{1..10,12,16,20,22,24,26}`) · bits 8-0 Y.
  ⚠ MAME: bit 14 = enable. MiSTer: **bits 15-13 = escala vertical**.
- W1: bits 15-12 banco · 11 flip X · 10 flip Y · bits 9-8 máscara (0 normal, 1 enmascarado/backup,
  2 solo máscara, 3 máscara frontal) · bits 2-0 log2 del ancho.
- W2: bits 10-9 prioridad · bits 8-0 X. ⚠ MAME: bits 15-13 escala X (no implementada).
  MiSTer: **bits 15-11 escala horizontal**, con divisor + Bresenham.
- W3: bit 15 "direct" (tile único en la propia palabra), de lo contrario puntero a una lista de tiles en la VRAM.
  1×1 también se trata como direct.

### Mixer [21][1]

- La prioridad menor queda al frente.
- MiSTer desempata por bits de "tie"; ROZ gana a una capa normal con la misma prioridad.
- Los sprites con blend suman solo los pens, sin carry.

### Ventanas (`$1D0` y `$1D8`) [22][29]

- Rellenan una franja horizontal con un color; los bordes vienen de una tabla en la VRAM, un par por cuadro o
  por línea (bit 8). Bit 11 invierte · bits 7-0 color · bits 14-13 prioridad.
- Efecto del scroll vertical de la ventana: desconocido [?].
- Con un par por línea, la línea L usa la entrada L de la tabla: el MiSTer la lee durante la línea L−1, pero su
  `render_y` ya es la línea siguiente [22], y MAME indexa por la propia línea [1]. V-Bear usó L−1 hasta
  2026-10-30 y dibujaba esas formas una línea más abajo (véase [Precisión y validación](accuracy.es.md)).

### Timing [17]

- 3420 MCLK por línea en los dos modos, 2560 activos. H256 = 342 puntos de MCLK/10; H320 = 420 puntos
  (70 a /8 y 30 a /10). ⚠ MAME usa htotal 455 a /8 en H320 (= 3640 MCLK/línea) [1].
- VBL en la línea 240.
- Arbitraje de la VRAM (MiSTer): 8 slots por punto [23].

## 5. Audio (sampler del UM6619) [2][13]

- 256 bytes de registros detrás de `$0420/$0422`.
- `$11/$12`: período del timer, alterna cada 5·(0x10000−n) clocks de sonido (MiSTer). ⚠ MAME: 10·(…) [2].
- `$14`: control del timer (bit 7 inicia; bits 7-6 = 00 detienen). Leer hace ack y devuelve la fase en el bit 6.
- `$16`: leer hace ack del IRQ de DMA/stream. `$17`: key on/off (bits 7-4 ≠ 0 = on, bits 3-0 = voz).
- Por voz *v*:
  - `$2v/$3v` pitch — incremento = pitch<<6 en un acumulador 16.16.
  - `$4v` control — el bit 1 habilita el envelope (solo MiSTer).
  - `$5v` longitud = 0x40 << ((d>>1)&7); bit 0 = one-shot.
  - `$6v/$7v` inicio ÷ 0x40.
  - `$9v` ≠ 0 = stream por DMA; al final genera IRQ y reinicia.
  - `$Av-$Dv` envelope (ataque, decay, sustain, release, nivel). **MiSTer lo implementa; MAME solo lo almacena.**
  - `$Ev` volumen en nibbles E/D.
- Muestras: 8 bits sin signo, centro 0x80. En MiSTer, el stream termina en un byte cero [13].
- MAME: `MACHINE_IMPERFECT_SOUND` (canales ruidosos, loops trabados) [1][5]. MiSTer: envelope medido en la
  consola real (boot chime) [13].

## 6. DMA, IRQ, timers y controles

### Host port `$E90000-$E9001F` [1][8][9][10][11]

- `$04/$05`: lee `$040C/$040D` del 6502.
- `$0A`: la escritura activa el IRQ bit 5 en el 6502.
- `$0C`: lee `$040A` y hace ack del IRQ6 (MiSTer).
- `$10`: máscara de IRQ — bit 7 VBL (IRQ7 + NMI del 6502), bit 4 línea visible (IRQ4).
- Los registros del host port graban la palabra entera del bus, sin mirar UDS/LDS [8]; como el
  68000 repite el byte en las dos mitades, una escritura de byte vale para los dos lados. Los juegos usan esto:
  Sango Fighter graba `$B8` como byte en `$E90010` para activar el IRQ4, cuyo manejador copia los bordes de la
  ventana 0 de `$FC0070/$FC0072` a la VRAM línea por línea (la cortina de las transiciones y de la pelea). Solo los
  registros de DMA (`$E90020-$3F`) respetan las mitades [14].
- `$14` control del FRC · `$16` recarga · `$18` valor actual (MAME lo llama "RNG"; la lectura hace ack del IRQ3).
- `$1C`: bit 0 suelta/retiene el 6502 · bit 1 desactiva el overlay bajo · bit 3 desactiva el overlay alto ·
  bit 2 desconocido.

### IRQs del 68k (autovector) [9][29]

1 expansión · 2 cartucho · 3 FRC · 4 línea visible · 5 raster on/off · 6 mailbox del 6502 · 7 VBL.

### FRC ⚠

- MiSTer: `$A2xx`; modo 0 = 1 Hz, modo 1 = 1024·(n+1) ticks, modo F = **9040**·(n+1). `$A300` cuenta cuadros [10].
- MAME: modo F = **8192**·período, con hacks por juego [1].

### DMA del host (2 canales) [1][14]

- Registros: origen, destino, cuenta, control. Disparo con `$8800`/`$A800`.
- Bit 12 palabra (de lo contrario repite el origen = relleno) · bit 8 retrocede el destino cada 16 bytes
  (registros de sprite) · bits 10/9 decremento.
- MiSTer: **bus master real** (BR/BG/BGACK, ciclos de 4 clocks, CPU detenida).
  MAME: instantáneo, con casos especiales para staiwbbl y rebelst.

### Controles [26][1]

- Pad serial de 16 bits en el esquema del SNES, activo en bajo, MSB primero.
- Bits: 15 A · 14 B · 13 Start · 12 Select · 11 Up · 10 Down · 9 Left · 8 Right · 7 X · 6 Y · 5 L ·
  4 R · 3-0 en 1.
- **Quien lee los controles es el 6502**, vía `$0407/$0402`.
- El multitap existe en MiSTer [26]; el diseño real es desconocido [?].

### Boot y BIOS

- El IPL del 68k (4 KiB) verifica el UM6650: claves en `$20-$2F` comparadas con "UMC 1994 (C)" +
  checksums; prueba la RAM en `$40-$5F` y escribe `$FF` en `$09` [24][3]. Después libera los overlays y el 6502 [11].
- MiSTer tiene `BIOS_BYPASS`, que salta el IPL [11].
- **El BIOS es indispensable para jugar:** todo juego, después de la intro, carga su driver de sonido (por DMA,
  en `$E800-$FFFF` de la sound RAM) y conversa con él por `$0300` (cola de comandos; el driver responde `$FF`),
  `$E9000A` (pedido) y `$040A` (respuesta/IRQ6). Quien lee los controles es ese driver, que escribe el estado de los
  pads en `$0200/$0202` para el 68000. Sin el BIOS, V-Bear simula solo el "listo" en `$0300`.

| Archivo | Tamaño | CRC32 | Observación |
|---|---|---|---|
| `internal_68k.bin` | 4 KiB | `8d575662` | palabras con bytes intercambiados |
| `internal_6502_1.bin` | 8 KiB | `fc9fb05f` | copiado a la sound RAM en `$0000` |
| `internal_6502_2.bin` | 8 KiB | `bf950ab7` | copiado a `$2000` |
| `umc6650.bin` | 16 B | `0ba78597` | clave del lockout |

MiSTer acepta también la concatenación de los cuatro (20.496 B) como `boot.rom` [25]. Los BIOS tienen
copyright de UMC/Funtech: V-Bear no los distribuye — los carga del usuario o usa HLE/bypass.

### Formato de las ROMs

- `.bin` crudo, palabras de 16 bits. MAME lee en big-endian nativo [4]; MiSTer dice que "la mayoría de los
  dumps tiene las palabras con bytes intercambiados" y lo detecta por la tabla de vectores [24] → **autodetectar**.
- Más de 2 MB usa la 2ª mask ROM (A21); las ROMs menores se espejan [24].
- Saves de 32 KiB [24].

## 7. Juegos

MAME: atributo `supported` del softlist [5]. MiSTer: README [6].

| Serie | ID MAME | Título | Año / editora | Tamaño | SRAM | MAME |
|---|---|---|---|---|---|---|
| F001 | formduel | Formosa Duel | 1995 AV Artisan | 1 MB | – | **no** |
| F002 | sangofgt | Sango Fighter | 1995 Panda | 3 MB | – | parcial |
| F003 | sonevil | The Son of Evil | 1995 Funtech | 2 MB | 32K? | parcial |
| F004 | speedyd | Speedy Dragon ("Sonic Dragon") | 1995 AV Artisan | 2 MB | – | **no** |
| F005 | staiwbbl | Super Taiwanese Baseball League | 1995 C&E | 2 MB | 32K? | **no** (sonido) |
| F006 | jttlaugh | Journey to the Laugh (嘻遊記), probablemente el "C.U.G." de Wikipedia [?] | 1995 Funtech | 2 MB | – | parcial |
| F007 | slghtsag | Super Light Saga / Dragon Force | 1996 Kingformation | 3 MB | 32K? | parcial |
| F008 | monopoly | Monopoly / African Adventure | 1995 Panda | 1 MB | 32K? | parcial |
| F009 | gamblord | Gambling Lord | 1996 Funtech | 2 MB | 32K? | parcial |
| F010 | magipool | Magical Pool | 1996 Funtech | 2 MB | – | parcial |
| F011 | boomzoo | Boom Zoo | 1996 Funtech | 512 KB | – | parcial |
| F012 | rebelst | Rebel Star (REBEL) | 1996 Horng Shen | 2 MB | 32K? | **no** |

- SRAM con "?" = "unconfirmed" en el propio softlist [5].
- MiSTer: ejecuta "the entire game library (apparently) bug free"; hecho a partir de videos de attract
  mode del hardware real (Diskman) y desensamblado de todos los juegos [6].
- Existen parches de traducción al inglés y paquetes de cheats [33].

## 8. Emuladores existentes y lagunas

- **MAME** (driver `supracan` desde la 0.269, 2024 [37]): `IMPERFECT_SOUND | IMPERFECT_GRAPHICS`;
  4 juegos sin soporte, 8 parciales [1][5]. Lagunas: solo 3 tilemaps; ROZ y tablas por línea con hacks;
  sin escala de sprites; prioridades aproximadas; sin envelope de sonido; clocks posiblemente errados;
  DMA instantáneo; FRC con hacks. Corre también en libretro-mame y en Batocera (pide `supracan.zip` y
  `umc6650.zip`) [32]. Informe de 2023: 5 de 11 juegos funcionando [38] (desactualizado).
- **MiSTer SuperACan_MiSTer**: commit único "Initial commit", 2026-09-11, Jamie Blanks (kitrinx) [6].
  Modela la placa PCGAM 16000-2A como netlist, con UM6618, UM6619, UM6650, UM70C188, KA2195D, además de
  savestates, cheats y multitap. **La referencia más completa hoy**; los comentarios del RTL documentan
  el hardware mejor que cualquier otra fuente.
- **Otros:** no se encontró ningún emulador independiente (standalone).

**Desconocido en todas las fuentes:** datasheets de UMC (no existen, según MiSTer [8][15]) ·
bit 2 de `$E9001C` · scroll Y de las ventanas · tabla `$19C` del ROZ · IRQs de expansión y cartucho ·
entrelazado · formato real del multitap y del mouse.

### Dónde V-Bear sigue a cada uno

Las ⚠ de arriba marcan los puntos en que MAME y MiSTer discrepan. V-Bear sigue a MiSTer en todos (la regla de
desempate de arriba), salvo donde una comparación mostró lo contrario:

| Punto | MAME | MiSTer | V-Bear |
|---|---|---|---|
| Reloj del 68000 | MCLK/6 | MCLK/5 | MiSTer |
| Reloj del 65C02 | MCLK/12 | CPU_CLK/3 | MiSTer |
| Tilemaps | 3 | 4 | MiSTer (4) |
| Tabla de ventanas, línea L | entrada L | par leído en la línea L−1, cuyo `render_y` ya es `vpos + 1`, o sea la entrada L | entrada L (corregido el 30/10/2026; ver [Precisión](accuracy.es.md)) |
| `$0411` (estado de IRQ) | la lectura borra todo | no borra nada | MiSTer |
| Escala de sprites | no implementada | bits 15-13 de la primera palabra | MiSTer |

## 9. Licencias

- **MAME:** `supracan.cpp`, `umc6619_sound.cpp`, `umc6650.cpp` y `bus/supracan/*` son **BSD-3-Clause**
  [C][1][2][3][4] (copyright Angelo Salese, Ryan Holtz, superctr). El softlist `supracan.xml` es **CC0** [5].
- **MiSTer:** el `LICENSE` del repositorio es **GPL v2**; `SuperAcan.sv` dice "v2 or later" [28]. Los archivos
  `um6618*`/`um6619*`/`board`/`cart` traen solo "Copyright (c) 2026 Jamie Blanks" → valen por el LICENSE
  GPLv2 [?: sin SPDX por archivo]. `rtl/fx68k` es GPL v3; `rtl/6502/mos6502*` es MIT [28].
- Consecuencia: traducir la lógica del RTL a V-Bear genera una obra derivada **GPL**. Para otra licencia,
  usar el RTL solo como documentación de comportamiento (clean-room) o portar solo el código de MAME.

## 10. NESdev

- Wiki: búsquedas de "Super A'Can", "Funtech" y "UM6619" sin resultado [36].
- Foro: solo menciones de pasada, nada técnico:
  - 2011, etabeta (dev de MESS) cita al Super A'Can entre los sistemas que MESS "emula totalmente" [34] —
    **contradice** el estado real (MAME aún marca el driver como imperfecto en 2026).
  - 2010, "Top 10 Failed Gaming Consoles": specs "comparables para 2D", difícil de programar [35].
- La búsqueda de phpBB no indexa "A'Can" (el apóstrofo rompe el término); también se buscaron "acan",
  "supracan", "super acan", "UM6619", "UMC 6650", "Funtech", "Sango Fighter".
- **Conclusión:** NESdev no complementa ni corrige a MAME/MiSTer.
