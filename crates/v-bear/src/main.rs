//! V-Bear: v-bear-core + emu-frontend.
//!
//! Usage: v-bear [GAME.zip|.bin | FOLDER] [--scale N]
//!        v-bear GAME --frames N [--bios PATH]   (no window: tests, see headless.rs)

// on Windows, a window app (no console window behind it)
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod headless;

use std::path::PathBuf;
use std::process::ExitCode;

use emu_frontend::{app, brand, config, i18n};

/// The About text; its translations are in `assets/i18n/<code>.json` (English text → translation).
const ABOUT: &str = "Funtech Super A'Can emulator in Rust, by Péricles S. Based on the MiSTer core and MAME.";

/// The icon, also embedded as the window's icon (title bar and taskbar).
const ICON: &[u8] = include_bytes!("../../../assets/icons/v-bear-256.png");

/// Narrowest window: the whole menu bar shows in any of the built-in languages. Its widest (Spanish,
/// 13 px Noto Sans, 28 px between items) is about 620 px, plus room for longer translations.
const MIN_WIDTH: f32 = 700.0;

/// Window icon: without it Windows shows the generic one in the title bar and, depending on the
/// version, in the taskbar (the .exe's own icon only covers Explorer).
fn window_icon() -> Option<egui::IconData> {
    let img = image::load_from_memory(ICON).ok()?.to_rgba8();
    Some(egui::IconData { width: img.width(), height: img.height(), rgba: img.into_raw() })
}

/// App identity; must come before loading the configuration (the folders use the `id`).
fn setup() {
    brand::set(brand::Brand {
        name: "V-Bear",
        id: "v-bear",
        version: env!("CARGO_PKG_VERSION"),
        icon_png: ICON,
        about: ABOUT,
        repo_url: "https://github.com/PSGM-Romhacking/v-bear",
        bezels: &[],
        free_games: &[], // no homebrew or free games for the system (searched 2026-10-23, docs/references.md)
        // second source of images, for games libretro-thumbnails lacks (Rebel Star)
        thumbnails_url: "https://raw.githubusercontent.com/PSGM-Romhacking/emulation-thumbnails/main",
    });
}

/// `v-bear --join GAME [OUTPUT] [--force]`: writes a game that comes in parts (a .zip or .7z with the
/// two ROM chips) as a single file, the parts joined, largest first. The same as Tools › Join game parts….
fn join_parts(args: &[String]) -> ExitCode {
    let force = args.iter().any(|a| a == "--force");
    let mut names = args.iter().filter(|a| !a.starts_with('-'));
    let Some(src) = names.next() else {
        eprintln!("usage: v-bear --join GAME [OUTPUT] [--force]   (OUTPUT defaults to the game's name with .{})", v_bear_core::INFO.extensions[0]);
        return ExitCode::from(2);
    };
    let src = std::path::Path::new(src);
    let dest = names.next().map(PathBuf::from).unwrap_or_else(|| {
        let stem = src.file_stem().map(|s| s.to_os_string()).unwrap_or_else(|| "game".into());
        PathBuf::from(stem).with_extension(v_bear_core::INFO.extensions[0])
    });
    if dest.exists() && !force {
        eprintln!("v-bear: {} already exists (--force overwrites it)", dest.display());
        return ExitCode::FAILURE;
    }
    match emu_frontend::rom::export_joined(src, &dest) {
        Ok(rom) => {
            let known = (v_bear_core::INFO.game_info)(&rom).map_or("not in the game database".to_string(), |g| format!("recognized as {}", g.title));
            println!("{}: {} bytes, CRC32 {:08x}, {known}", dest.display(), rom.len(), emu_api::crc32(&rom));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("v-bear: {e}");
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("{}", headless::USAGE);
        return ExitCode::SUCCESS;
    }
    if args.iter().any(|a| a == "--frames") {
        return headless::run();
    }
    if let Some(i) = args.iter().position(|a| a == "--join") {
        return join_parts(&args[i + 1..]);
    }
    setup();
    emu_frontend::crashlog::install();
    let cfg = config::Config::load();
    i18n::set_lang(&cfg.prefs.language);
    let mut rom: Option<PathBuf> = None;
    let mut scale: Option<u32> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "-s" | "--scale" => scale = args.next().and_then(|v| v.parse().ok()).map(|v: u32| v.clamp(1, 6)),
            _ if !a.starts_with('-') => rom = Some(PathBuf::from(a)),
            _ => eprintln!("unknown option: {a}"),
        }
    }

    let info = &v_bear_core::INFO;
    let explicit = scale.is_some();
    let scale = scale.unwrap_or(cfg.prefs.scale).clamp(1, 6);
    let size = if explicit { [(info.width as u32 * scale) as f32, (info.height as u32 * scale) as f32 + 28.0] } else { app::library_window_size(info) };
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("V-Bear")
        .with_app_id("v-bear")
        .with_inner_size(size)
        .with_min_inner_size([MIN_WIDTH.max(info.width as f32), info.height as f32 + 28.0])
        .with_fullscreen(cfg.prefs.couch);
    if let Some(icon) = window_icon() {
        viewport = viewport.with_icon(icon);
    }
    let options = eframe::NativeOptions { viewport, ..Default::default() };
    if let Err(e) = eframe::run_native("V-Bear", options, Box::new(move |cc| Ok(Box::new(app::App::new(cc, info, rom, scale))))) {
        eprintln!("v-bear: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    /// The About text has a translation in every built-in language.
    #[test]
    fn about_is_translated() {
        for code in ["pt", "es"] {
            emu_frontend::i18n::set_lang(code);
            assert_ne!(emu_frontend::i18n::tr(super::ABOUT), super::ABOUT, "{code}");
        }
        emu_frontend::i18n::set_lang("en");
    }

    /// Same games and fields in every language, no unknown field (checked by the frontend).
    #[test]
    fn game_texts_valid() {
        let problems = emu_frontend::sheets::check(&v_bear_core::INFO);
        assert!(problems.is_empty(), "{problems:?}");
    }
}
