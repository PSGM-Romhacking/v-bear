# Traducir V-Bear

[English](translating.md) · [Português](translating.pt-BR.md) · **Español**

V-Bear existe en inglés, portugués y español. (El inglés es el texto fuente; el portugués y el español vienen incluidos.) Se puede añadir cualquier otro idioma
**sin programar y sin compilar nada**: la traducción es un único archivo JSON
que el emulador lee al iniciar. Puedes quedártelo para ti o enviarlo para que se incluya
en la próxima versión, de modo que todos lo tengan.

## Contenido

1. [Qué necesitas](#1-qué-necesitas)
2. [Paso a paso](#2-paso-a-paso)
3. [El formato del archivo](#3-el-formato-del-archivo)
4. [Reglas que mantienen la interfaz funcionando](#4-reglas-que-mantienen-la-interfaz-funcionando)
5. [Probar tu traducción](#5-probar-tu-traducción)
6. [Corregir un idioma existente](#6-corregir-un-idioma-existente)
7. [Cuando una versión nueva añade textos](#7-cuando-una-versión-nueva-añade-textos)
8. [Enviarla para que se incluya](#8-enviarla-para-que-se-incluya)
9. [Fichas de los juegos (opcional)](#9-fichas-de-los-juegos-opcional)
10. [Preguntas](#10-preguntas)

## 1. Qué necesitas

- V-Bear **0.5.0 o más reciente**.
- Un editor de texto que guarde en **UTF-8** (cualquiera moderno: VS Code, Notepad++, Kate, gedit,
  micro…). Evita los procesadores de texto como Word o LibreOffice Writer.
- Unas dos a cuatro horas: son unos cientos de textos, la mayoría cortos (elementos de menú,
  botones).

## 2. Paso a paso

1. **Exporta la plantilla.** En el emulador, abre **Ayuda › Language / Idioma › Exportar plantilla de traducción…** y guarda el archivo. Ya se ofrece en la carpeta de idiomas.
2. **Nombra el archivo con el código del idioma**, en minúsculas:
   `fr.json` (francés), `de.json` (alemán), `it.json` (italiano), `ja.json` (japonés),
   `pt-PT.json` (portugués europeo), `zh-CN.json` (chino simplificado)… El nombre es lo que
   le dice al emulador de qué idioma se trata. Usa solo letras, dígitos y `-`.
3. **Traduce.** Abre el archivo en el editor de texto y reemplaza cada valor (el texto de la
   derecha) por su traducción. Deja los textos de la izquierda como están. Detalles en las dos
   secciones siguientes.
4. **Escribe el nombre del idioma** en `"@name"`, como lo escriben sus hablantes: `"Français"`,
   `"Deutsch"`, `"日本語"`. Es lo que aparece en el menú de idiomas.
5. **Pon el archivo en la carpeta de idiomas** y reinicia el emulador. Para encontrar la carpeta:
   **Emulación › Carpetas…**, línea **Idiomas**, botón **Abrir**. Por defecto es:

   | Sistema | Carpeta de idiomas |
   |---|---|
   | Linux, FreeBSD | `~/.config/v-bear/languages/` |
   | Windows | `%APPDATA%\v-bear\languages\` |
   | Modo portátil | `v-bear_data/languages/`, junto al programa |

6. **Elígelo** en **Ayuda › Language / Idioma**. Tu idioma aparece después de los
   incluidos.

Eso es todo para usarlo tú mismo. Para compartirlo, consulta la [sección 8](#8-enviarla-para-que-se-incluya).

## 3. El formato del archivo

La plantilla es una lista de pares `"texto original": "traducción"`. Recién exportada, cada
traducción es una copia del texto en inglés:

```json
{
  "@name": "Language name",
  "Open game…": "Open game…",
  "Search {n} games…": "Search {n} games…",
  "Screenshot saved: {file}": "Screenshot saved: {file}"
}
```

Traducida al francés, queda así:

```json
{
  "@name": "Français",
  "Open game…": "Ouvrir un jeu…",
  "Search {n} games…": "Rechercher parmi {n} jeux…",
  "Screenshot saved: {file}": "Capture enregistrée : {file}"
}
```

- **El lado izquierdo es la clave**: el texto en inglés exactamente como está en el programa. Nunca
  lo cambies, o el emulador no encontrará tu traducción y mostrará el inglés.
- **El lado derecho es lo que se muestra.** Puede dejarse como está (idéntico al inglés) o
  **vaciarse** (`""`), lo que significa "aún sin traducir": en su lugar se muestra el inglés. Así, un
  archivo incompleto ya funciona, y puedes traducir en varias sesiones.
- **El orden de las líneas no importa**, y se pueden quitar líneas (muestran el inglés).

## 4. Reglas que mantienen la interfaz funcionando

1. **Conserva los campos entre llaves** exactamente como están: `{n}`, `{name}`, `{file}`,
   `{path}`, `{error}`, `{system}`… El programa los reemplaza por un número, un nombre o una
   ruta. Pueden moverse dentro de la frase (`"{n} jeux"`), pero no traducirse ni quitarse.
2. **Conserva los "…"** al final de los elementos de menú que abren una ventana (`"Folders…"` →
   `"Dossiers…"`). Es una convención: "…" significa "abre algo donde vas a elegir".
3. **Conserva el "›"** en las rutas de menú (`Emulación › Controles y atajos…`) y traduce los
   nombres de ambos lados igual que tradujiste esos menús, para que la gente pueda seguirlos.
4. **Mantén el JSON válido:** cada línea termina con coma excepto la última; las comillas dentro de un
   texto se escriben `\"`; un salto de línea dentro de un texto es `\n`. Si el archivo tiene un error, el
   emulador muestra un mensaje al iniciar con el nombre del archivo y el motivo, e ignora ese
   archivo (los demás se siguen cargando).
5. **Los textos cortos siguen siendo cortos.** Los elementos de menú y los botones tienen espacio limitado. Si la
   traducción es mucho más larga que el inglés, busca una palabra más corta.
6. **No traduzcas** nombres de cosas: V-Bear, Funtech Super A'Can, RetroArch, libretro,
   RetroAchievements, los formatos de archivo (`.bin`, `.cht`, PNG, MP4,
   GIF), nombres de teclas (`Ctrl`, `Esc`, `F2`) y los nombres de carpeta `Named_Boxarts`,
   `Named_Titles` y `Named_Snaps`.
7. **Términos técnicos:** usa lo que realmente dicen los jugadores de tu idioma. "Save state",
   "ROM", "cheat" y "rewind" a menudo se dejan en inglés; en caso de duda, mira cómo
   se traduce RetroArch a tu idioma y síguelo.
8. **Las etiquetas de teclado y mando** (`KEYBOARD`, `CONTROLLER`, `EMULATOR`…) son encabezados
   de columna mostrados en mayúsculas: déjalos en mayúsculas.

## 5. Probar tu traducción

Después de reiniciar con tu idioma elegido, recorre estas pantallas; juntas muestran
casi todos los textos:

- [ ] todos los menús de la barra de menús, incluidos los submenús;
- [ ] la biblioteca (búsqueda, las pestañas de imagen, clic derecho en un juego, la pantalla de bienvenida cuando
      la carpeta está vacía);
- [ ] un juego en ejecución: los mensajes de la parte superior (guardar estado, captura de pantalla, avance rápido);
- [ ] **Emulación › Controles y atajos…** y **Ayuda › Controles**;
- [ ] **Emulación › Carpetas…**;
- [ ] **Herramientas › Depurador…** y **Herramientas › Trucos…** (las dos pestañas);
- [ ] **Ayuda › Ficha del juego…** y **Ayuda › Acerca de**;
- [ ] el menú rápido (botón Inicio/Guía de un mando, o **Vídeo › Modo sofá** y Esc).

Busca palabras cortadas, textos que se salen del botón e inglés que quedó atrás (una clave que
se cambió por accidente, o un valor todavía vacío).

## 6. Corregir un idioma existente

Para arreglar unos pocos textos de un idioma incluido (portugués o español), no necesitas toda la
plantilla: crea `pt.json` o `es.json` en la carpeta de idiomas con **solo las líneas que
quieres cambiar**. Tus textos ocupan el lugar de los incluidos; todo lo demás se queda.
`"@name"` se ignora en los idiomas incluidos. Si la corrección sirve para todos, envíala también
([sección 8](#8-enviarla-para-que-se-incluya)).

## 7. Cuando una versión nueva añade textos

Las versiones nuevas añaden y cambian textos. Para poner al día una traducción:

1. Exporta la plantilla de nuevo desde la versión nueva.
2. Compárala con tu archivo. Una herramienta de comparación de texto ayuda: la vista "comparar archivos" de
   VS Code, Meld, WinMerge o `diff`. Las líneas que existen solo en la plantilla nueva son los
   textos nuevos.
3. Copia las líneas nuevas a tu archivo y tradúcelas. Las líneas que existen solo en tu archivo
   son textos que el programa ya no usa: son inofensivas, pero se pueden borrar.

Hasta que lo hagas, los textos nuevos simplemente aparecen en inglés.

## 8. Enviarla para que se incluya

Las traducciones enviadas se añaden a la próxima versión, incluidas en el programa, con crédito al
traductor en el README y en las notas de la versión.

**Cómo enviar**, por orden de preferencia:

1. **Una issue en GitHub:** abre
   <https://github.com/PSGM-Romhacking/v-bear/issues/new>, con el título
   `Translation: <idioma>` (por ejemplo `Translation: Français`), y adjunta el archivo `.json`
   (si GitHub rechaza el adjunto, comprímelo antes como `.zip`). Indica cómo quieres
   que te den crédito: nombre o apodo y, opcionalmente, un enlace.
2. **Un pull request**, si usas Git: pon el archivo en `translations/<código>.json` y abre
   el pull request. El mantenedor lo divide en los catálogos del programa.

**Antes de enviar**, comprueba:

- [ ] el nombre del archivo es el código del idioma (`fr.json`);
- [ ] `"@name"` está completado con el nombre propio del idioma;
- [ ] el emulador lo carga sin mensaje de error, y las pantallas de la
      [sección 5](#5-probar-tu-traducción) se ven bien;
- [ ] se exportó de la **última versión** (o se actualizó como en la
      [sección 7](#7-cuando-una-versión-nueva-añade-textos)).

Una traducción parcial también es bienvenida: indícalo en la issue, y los textos que falten se mostrarán
en inglés hasta que alguien los complete.

**Licencia:** V-Bear es GPL-3.0 o posterior. Al enviar una traducción aceptas que se
distribuya bajo la misma licencia, como el resto del programa.

## 9. Fichas de los juegos (opcional)

Las descripciones en **Ayuda › Ficha del juego…** (de qué trata el juego, notas) no están en la
plantilla: pertenecen a la base de datos de juegos, un archivo por idioma, en el repositorio, en
`assets/games/` (`games.en.json`, `games.pt.json`, `games.es.json`). Para un idioma
que no las tenga, el emulador las muestra en inglés, así que traducirlas no es necesario para que la
interfaz esté completa. (El número de jugadores no es un texto: su frase, "1 o {n}
jugadores", está en la plantilla de la interfaz.)

Para traducirlas también, copia `games.en.json` a `games.<código>.json`, traduce solo los
valores de `"description"` y `"notes"` (conserva las claves de los juegos, como `"game-id"`, y los
nombres de los campos) y envíalo en la misma issue. Estas van incluidas en el programa, así que solo
aparecen después de la próxima versión.

## 10. Preguntas

Abre una issue en GitHub con el título `Translation question: <idioma>`. Las preguntas sobre
el significado de un texto son bienvenidas: algunos son cortos y dependen de la pantalla en la que están.
