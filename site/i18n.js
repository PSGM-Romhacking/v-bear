(function () {
  var T = {
    pt: {
      nav_features: "Recursos", nav_compat: "Compatibilidade", nav_screens: "Telas", nav_download: "Baixar",
      hero_title: "Um emulador do Funtech Super A'Can",
      hero_lead: "Jogue no computador os doze jogos do console de 16 bits lançado em Taiwan em 1995. Escrito em Rust a partir da documentação do hardware, do MAME e do core do MiSTer.",
      btn_download: "Baixar", btn_source: "Código-fonte",
      stat_games: "jogos rodam", stat_platforms: "Linux, Windows, FreeBSD e RetroArch",

      bios_sub: "O V-Bear precisa da BIOS do console, que não vem junto: os 4 arquivos abaixo. Um único supracan.zip do MAME não tem todos (o mais antigo traz internal_68k.bin e umc6650.bin; o mais novo, internal_68k.bin e os dois internal_6502).",

      feat_title: "Recursos", feat_sub: "Tudo o que se espera de um emulador moderno.", more: "Saiba mais →",
      f2_t: "Save states e rewind", f2_d: "Vários slots, avanço rápido e retrocesso enquanto você segura uma tecla ou botão.",
      f3_t: "Gravação de GIF e MP4", f3_d: "Capture telas, GIFs e vídeos MP4 (o MP4 precisa do ffmpeg).",
      f4_t: "Depurador e cheats", f4_d: "Visores de CPU, memória e tiles, e um gerenciador de cheats.",
      f5_t: "Core libretro", f5_d: "Jogue dentro do RetroArch no Linux, Windows e FreeBSD.",

      compat_title: "Compatibilidade",
      compat_sub: "Os 12 jogos ligam e são jogáveis com a BIOS.",
      compat_s1: "jogáveis", compat_s3: "saves de bateria confirmados",
      compat_all: "Ver a lista completa",
      compat_search: "Buscar por nome, desenvolvedora ou CRC",
      compat_c_game: "Título", compat_c_year: "Ano", compat_c_players: "Jogadores", compat_c_save: "Save de bateria",
      compat_c_status: "Status", 
      st_run: "Sem bugs encontrados",
      st_frz: "Trava nas batalhas", st_rare: "Raramente trava", st_vis2: "Defeitos visuais",
      compat_ok: "✓ Jogável",  sr_yes: "Sim, a testar", sr_ok: "✓ Testado", sr_unc: "Não confirmado",
      compat_none: "Nenhum jogo encontrado.",
      compat_crc: "CRC32 das cópias No-Intro. O Rebel Star também é reconhecido na ordem de bytes de cópias mais antigas (a3469172).",

      shots_title: "Telas",
      s1: "Biblioteca com telas de jogo", s2: "Biblioteca em lista", s3: "Super Dragon Force", s4: "Ficha do jogo",
      s5: "Modo sofá", s6: "Menu rápido", s7: "Pastas: BIOS, saves de bateria e outras", s8: "Cheats", s9: "Visor de memória",

      bios_file: "Arquivo",
      dl_title: "Baixar", dl_sub: "Versão 0.5.2.",
      dl_glibc: "glibc 2.27 ou mais nova (Ubuntu 18.04, Debian 10, RHEL 8 e posteriores)",
      cl1: "Interface nova: biblioteca com capas, fichas dos jogos, controles configuráveis, save states com miniatura, rewind, gravação, cheats, modo sofá.",
      cl2: "Menu BIOS: escolha a pasta e veja cada arquivo com tamanho, CRC32 e se foi encontrado.",
      cl3: "Saves de bateria numa pasta própria, compatíveis com MAME e MiSTer; um .sav antigo ao lado do jogo é aproveitado.",
      cl4: "Modo lista, tempo de jogo e histórico.",
      cl5: "Gênero e descrição de cada jogo em inglês, português e espanhol.",
      cl6: "Relatório de erro: se o programa fechar sozinho, a próxima abertura mostra onde está o relatório.",
      cl7: "A janela nunca abre maior que a tela.",
      cl10: "Magical Pool: o taco não some mais na tela de força da tacada.",
      cl11: "Journey to the Laugh: o herói não some mais ao tomar dano (visto no Windows).",
      cl12: "Sango Fighter: o horizonte está completo. Super Dragon Force: os menus não mostram mais molduras de outras telas.",
      cl_all: "Changelog completo →",
      dl_btn: "Versões anteriores", 
      foot_lic: "Software livre sob GPL-3.0 ou posterior.",
      foot_tm: "Super A'Can é marca de seus donos. O V-Bear não tem ligação com a Funtech.",
    },
    es: {
      nav_features: "Funciones", nav_compat: "Compatibilidad", nav_screens: "Capturas", nav_download: "Descargar",
      hero_title: "Un emulador de la Funtech Super A'Can",
      hero_lead: "Juega en tu ordenador a los doce juegos de la consola de 16 bits lanzada en Taiwán en 1995. Escrito en Rust a partir de la documentación del hardware, MAME y el core de MiSTer.",
      btn_download: "Descargar", btn_source: "Código fuente",
      stat_games: "juegos funcionan", stat_platforms: "Linux, Windows, FreeBSD y RetroArch",

      bios_sub: "V-Bear necesita la BIOS de la consola, que no se incluye: los 4 archivos de abajo. Un único supracan.zip de MAME no los trae todos (el antiguo trae internal_68k.bin y umc6650.bin; el nuevo, internal_68k.bin y los dos internal_6502).",

      feat_title: "Funciones", feat_sub: "Todo lo que se espera de un emulador moderno.", more: "Más información →",
      f2_t: "Save states y rebobinado", f2_d: "Varias ranuras, avance rápido y rebobinado mientras mantienes una tecla o botón.",
      f3_t: "Grabación de GIF y MP4", f3_d: "Captura pantallas, GIFs y vídeos MP4 (el MP4 necesita ffmpeg).",
      f4_t: "Depurador y trucos", f4_d: "Visores de CPU, memoria y tiles, y un gestor de trucos.",
      f5_t: "Core libretro", f5_d: "Juega dentro de RetroArch en Linux, Windows y FreeBSD.",

      compat_title: "Compatibilidad",
      compat_sub: "Los 12 juegos arrancan y se pueden jugar con la BIOS.",
      compat_s1: "jugables", compat_s3: "partidas de batería confirmadas",
      compat_all: "Ver la lista completa",
      compat_search: "Buscar por nombre, desarrolladora o CRC",
      compat_c_game: "Título", compat_c_year: "Año", compat_c_players: "Jugadores", compat_c_save: "Partida de batería",
      compat_c_status: "Estado", 
      st_run: "Sin errores encontrados",
      st_frz: "Se bloquea en los combates", st_rare: "Se bloquea raramente", st_vis2: "Fallos visuales",
      compat_ok: "✓ Jugable",  sr_yes: "Sí, por probar", sr_ok: "✓ Probada", sr_unc: "Sin confirmar",
      compat_none: "No se encontraron juegos.",
      compat_crc: "CRC32 de los volcados No-Intro. Rebel Star también se reconoce en el orden de bytes de copias más antiguas (a3469172).",

      shots_title: "Capturas de pantalla",
      s1: "Biblioteca con capturas de juego", s2: "Biblioteca en lista", s3: "Super Dragon Force", s4: "Ficha del juego",
      s5: "Modo sofá", s6: "Menú rápido", s7: "Carpetas: BIOS, partidas de batería y más", s8: "Trucos", s9: "Visor de memoria",

      bios_file: "Archivo",
      dl_title: "Descargar", dl_sub: "Versión 0.5.2.",
      dl_glibc: "glibc 2.27 o más reciente (Ubuntu 18.04, Debian 10, RHEL 8 y posteriores)",
      cl1: "Interfaz nueva: biblioteca con carátulas, fichas de los juegos, controles configurables, save states con miniatura, rebobinado, grabación, trucos, modo sofá.",
      cl2: "Menú BIOS: elige la carpeta y ve cada archivo con su tamaño, CRC32 y si se encontró.",
      cl3: "Partidas de batería en su propia carpeta, compatibles con MAME y MiSTer; se aprovecha un .sav antiguo junto al juego.",
      cl4: "Vista de lista, tiempo de juego e historial.",
      cl5: "Género y descripción de cada juego en inglés, portugués y español.",
      cl6: "Informe de errores: si el programa se cierra solo, el siguiente inicio muestra dónde está el informe.",
      cl7: "La ventana nunca se abre más grande que la pantalla.",
      cl10: "Magical Pool: el taco ya no desaparece en la pantalla de fuerza del golpe.",
      cl11: "Journey to the Laugh: el héroe ya no desaparece al recibir daño (visto en Windows).",
      cl12: "Sango Fighter: el horizonte está completo. Super Dragon Force: los menús ya no muestran marcos de otras pantallas.",
      cl_all: "Changelog completo (en portugués) →",
      dl_btn: "Versiones anteriores", 
      foot_lic: "Software libre bajo GPL-3.0 o posterior.",
      foot_tm: "Super A'Can es marca de sus dueños. V-Bear no está afiliado a Funtech.",
    }
  };

  var nodes = document.querySelectorAll("[data-i18n], [data-i18n-html]");
  var orig = {};
  nodes.forEach(function (n) {
    var k = n.getAttribute("data-i18n") || n.getAttribute("data-i18n-html");
    orig[k] = n.hasAttribute("data-i18n-html") ? n.innerHTML : n.textContent;
  });

  var phNodes = document.querySelectorAll("[data-i18n-ph]");
  phNodes.forEach(function (n) { orig["ph:" + n.getAttribute("data-i18n-ph")] = n.placeholder; });

  function apply(lang) {
    phNodes.forEach(function (n) {
      var k = n.getAttribute("data-i18n-ph");
      n.placeholder = lang !== "en" && T[lang] && T[lang][k] ? T[lang][k] : orig["ph:" + k];
    });
    nodes.forEach(function (n) {
      var html = n.hasAttribute("data-i18n-html");
      var k = n.getAttribute(html ? "data-i18n-html" : "data-i18n");
      var v = lang === "en" || !T[lang] || !T[lang][k] ? orig[k] : T[lang][k];
      if (html) n.innerHTML = v; else n.textContent = v;
    });
    document.documentElement.lang = lang === "pt" ? "pt-BR" : lang;
    document.querySelectorAll(".lang button").forEach(function (b) {
      b.setAttribute("aria-pressed", b.dataset.lang === lang ? "true" : "false");
    });
    try { localStorage.setItem("lang", lang); } catch (e) {}
  }

  document.querySelectorAll(".lang button").forEach(function (b) {
    b.addEventListener("click", function () { apply(b.dataset.lang); });
  });

  var lang = "en";
  try { lang = localStorage.getItem("lang") || ""; } catch (e) {}
  if (!lang) {
    var nav = (navigator.language || "en").slice(0, 2);
    lang = T[nav] ? nav : "en";
  }
  apply(lang);
})();
