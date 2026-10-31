# Lecciones aprendidas

[English](lessons.md) · [Português](lessons.pt-BR.md) · **Español**

Lo que enseñó el proyecto, para quien quiera hacer lo mismo en otro sistema. Cada lección viene de algo que
ocurrió aquí; los números están en [Precisión y validación](accuracy.es.md) y
[Compatibilidad](compatibility.es.md).

## Enfoque

- **Ten dos implementaciones para comparar, no una.** V-Bear se escribió a partir del MiSTer core y de
  MAME. Ninguno de los dos es la consola: el MiSTer core también es una reconstrucción (un commit, ninguna
  prueba en una consola real). Cuando discrepan, hace falta una tercera opinión. El bug de la ventana
  (abajo) solo se resolvió porque MAME coincidió con el MiSTer core frente a V-Bear.
- **Un comentario en el código fuente es una afirmación, no una medición.** V-Bear recortaba cada línea con
  la entrada de la tabla de ventanas de la línea *anterior* porque el MiSTer core dice "el par obtenido
  durante la línea L-1 recorta la línea L". Lo que la frase omite es que el `render_y` del core ya es
  `vpos + 1`, así que el par que obtiene es la entrada L. La imagen estuvo equivocada por una línea hasta
  que la comparación lo mostró.
- **"Idéntico a la referencia" nunca significa "idéntico a la consola".** Indica qué referencia, qué
  fotogramas y qué juegos, y mantén lo que no se comparó en la lista de lo que falta.

## Comparación con un core FPGA simulado

- **Simula el Verilog del core con Verilator y compara imagen y RAM.** Es la única forma, aquí, de ejecutar
  el hardware del MiSTer sin un MiSTer. El costo es la velocidad: cerca de un fotograma por segundo para
  esta consola, así que 300 fotogramas toman cinco minutos por juego.
- **No compiles con `--public-flat-rw`.** Hace que Verilator exponga todas las señales de un core que tiene
  decenas de miles de ellas, y el compilador de C++ gasta más de 25 minutos en un solo archivo generado.
  Marca solo lo que lees con un comentario `public_flat_rw`, y lee el resto por puertos que el core ya tiene
  (la work RAM aquí se lee por el puerto de savestate).
- **Pequeños detalles de build que cuestan una tarde:** el paquete del 6502 tiene que ir primero en la
  lista de archivos; `-Wno-BLKANDNBLK` para el core del 68000 (`fx68k`); y `fx68k` lee dos archivos `.mem`
  de microcódigo del directorio *actual*, así que la simulación tiene que ejecutarse junto a ellos, o el
  68000 no funciona, con solo una advertencia en pantalla.
- **Dos implementaciones nunca arrancan a la misma velocidad.** El mismo juego llega a la misma imagen con
  algunos fotogramas de diferencia (de 0 a 13 aquí, y varía por juego), porque la lectura del cartucho
  tarda un tiempo distinto. Compara el fotograma N de una con una ventana de fotogramas de la otra, e
  informa el desfase.
- **Un empate no es un resultado.** En una imagen quieta, muchos fotogramas dan 0 píxeles distintos; tomar
  el primero comparó la RAM en el fotograma equivocado (15.000 bytes distintos en un juego). Desempata por
  el fotograma más cercano al esperado.
- **Mira qué difiere antes de decidir que es un bug.** Dos grupos de diferencias no se atribuyeron a un
  error: un fade cuyos tres canales de color suben en cuadros distintos de un ciclo de 4 cuadros (la imagen
  es idéntica cada cuarto cuadro; no se encontró la causa), y unos pocos bytes de la pila y de los contadores del
  68000, probablemente porque los dos lados se detienen en puntos ligeramente distintos del fotograma (no
  probado). Una diferencia era real: una forma recortada por la ventana del chip de vídeo, dibujada una línea más abajo de lo correcto.
- **Comprueba que la referencia ejecutó el mismo juego.** La primera comparación dio a la simulación solo uno de
  los dos archivos de Super Dragon Force (un juego en dos chips de ROM; el zip lista primero el chip de 1 MB), así
  que su resultado de RAM (8.682 bytes) no significaba nada: con las partes unidas dio 868. Cuando una cifra
  parezca extraña, comprueba la entrada antes de culpar a cualquier emulador.
- **Mide dónde está una forma, no solo cuántos píxeles difieren.** El bug de la ventana parecía ruido (76
  píxeles en un anillo); la caja delimitadora del círculo mostró que estaba exactamente una línea más
  abajo, y la cuenta bajó de 76 a 4 al desplazarlo de vuelta.

## Interfaz y RetroArch

- **Identifica la BIOS por CRC32, no por nombre.** Los mismos cuatro archivos vienen de MAME, de MiSTer y de
  dumps con muchos nombres, y los sets de MAME no contienen todos (la clave `umc6650.bin` está en su propio
  set).
- **RetroArch guarda en caché la información del core.** Tras un nuevo `.info`, el core aparecía como
  `v_bear_libretro`, sin descripción ni licencia, y el explorador de archivos ocultaba `.bin` y `.can`,
  hasta que se limpió la caché. El síntoma parece un bug del core y no lo es.
- **Prueba primero el core libretro sin RetroArch.** Un programa pequeño que hace el papel de RetroArch por
  la API C (búsqueda de la BIOS, fotogramas, save state, RAM de batería) encuentra los fallos obvios en
  segundos; lo que queda es lo que solo muestra RetroArch, como la caché de arriba.
- **Una base de frontend ahorra meses,** pero las necesidades propias del sistema (una BIOS obligatoria en
  varios archivos, saves de batería, juegos en dos archivos) van más allá del contrato de la base y hay que
  añadirlas con cuidado.

## Modelo de hardware

- **Sigue la placa, no el manual.** Cuando MAME y el MiSTer core discrepan en relojes o temporización,
  gana el core que modela la placa entera y ejecuta toda la biblioteca, y la discrepancia queda anotada
  ([Hardware](hardware.es.md)).
- **Dale tiempo al 68000 en la ventana de la RAM de sonido.** Speedy Dragon pierde una carrera entre un
  timeout y la inicialización del driver de sonido si el acceso no cuesta nada, y recarga el driver para
  siempre.
