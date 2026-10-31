//! Command-line mode without a window (`--frames N`): runs a game for N frames and
//! prints a summary; with the V_BEAR_* variables it captures pictures, sound and
//! traces (docs/compatibility.md). Used to test the core without the interface.

use v_bear_core::{bios, cart, console, input};

use std::path::PathBuf;
use std::process::ExitCode;

pub const USAGE: &str = "\
Usage: v-bear [GAME.zip|.bin | FOLDER] [options]

Options:
  -s, --scale <n>     window scale (1-6)
      --frames <n>    run n frames without a window and exit (testing)
  -b, --bios <path>   with --frames: BIOS as MAME supracan.zip, a folder or MiSTer boot.rom
                      (default: bios/ or supracan.zip next to the ROM, here or next to the program)
      --join <game> [out] [--force]
                      write a game that comes in parts (a .zip or .7z) as a single file, the parts
                      joined; out defaults to the game's name with the system's main extension
  -h, --help          show this help";

struct Args {
    rom: PathBuf,
    bios: Option<PathBuf>,
    frames: Option<u64>,
}

fn parse_args() -> Result<Args, String> {
    let mut rom = None;
    let mut bios = None;
    let mut frames = None;
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{USAGE}");
                std::process::exit(0);
            }
            "-s" | "--scale" => {
                it.next();
            }
            "-b" | "--bios" => bios = Some(PathBuf::from(it.next().ok_or("missing value for --bios")?)),
            "--frames" => {
                let v = it.next().ok_or("missing value for --frames")?;
                frames = Some(v.parse().map_err(|_| format!("invalid frame count: {v}"))?);
            }
            _ if arg.starts_with('-') => return Err(format!("unknown option: {arg}")),
            _ if rom.is_none() => rom = Some(PathBuf::from(arg)),
            _ => return Err(format!("unexpected argument: {arg}")),
        }
    }
    let rom = rom.ok_or("no ROM given")?;
    Ok(Args { rom, bios, frames })
}

pub fn run() -> ExitCode {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("v-bear: {e}\n\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let cart = match cart::Cartridge::load(&args.rom) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("v-bear: {}: {e}", args.rom.display());
            return ExitCode::FAILURE;
        }
    };
    println!("{}", cart.summary());

    let bios_path = args.bios.clone().or_else(|| default_bios(&args.rom));
    let bios = match bios_path {
        Some(p) => match bios::Bios::load(&p) {
            Ok(b) => {
                println!("BIOS: {}", p.display());
                Some(b)
            }
            Err(e) => {
                eprintln!("v-bear: BIOS {}: {e}", p.display());
                return ExitCode::FAILURE;
            }
        },
        None => {
            println!("BIOS: none, booting the cartridge directly (no sound, no pads yet)");
            None
        }
    };

    let mut console = console::Console::new(cart.clone(), bios.clone());
    // V_BEAR_SAV=file: the battery save, read before the run and written after it if the
    // game changed it (the same 32 KiB format as the interface's .sav, MAME and MiSTer).
    let sav = std::env::var_os("V_BEAR_SAV").map(std::path::PathBuf::from);
    if let Some(p) = &sav {
        match console.load_sram(p) {
            Ok(true) => println!("battery save: {}", p.display()),
            Ok(false) => println!("battery save: {} (new)", p.display()),
            Err(e) => eprintln!("v-bear: {}: {e}", p.display()),
        }
    }
    if let Some(n) = std::env::var("V_BEAR_TRACE").ok().and_then(|v| v.parse().ok()) {
        let after: u64 = std::env::var("V_BEAR_TRACE_AFTER").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
        for _ in 0..after {
            console.run_frame();
        }
        console.trace(n);
        return ExitCode::SUCCESS;
    }
    // Determinism check: V_BEAR_SNAPTEST=k saves a state at frame k (through
    // the save state format), runs to the end, loads it into a fresh machine,
    // runs again and compares the picture and the whole machine state.
    if let (Some(n), Some(k)) = (args.frames, std::env::var("V_BEAR_SNAPTEST").ok().and_then(|v| v.parse::<u64>().ok())) {
        for _ in 0..k {
            console.run_frame();
        }
        let state = console.save_state().expect("save state");
        for _ in k..n {
            console.run_frame();
        }
        let (first, end) = (console.bus.video.frame.clone(), console.save_state().expect("save state"));
        let mut console = console::Console::new(cart, bios);
        console.load_state(&state).expect("load state");
        for _ in k..n {
            console.run_frame();
        }
        let again = console.save_state().expect("save state");
        let same = first == console.bus.video.frame && end == again;
        println!(
            "state at {k} ({} KiB), compared at {n}: {}",
            state.len() / 1024,
            if same { "identical" } else { "DIFFERENT" }
        );
        return if same { ExitCode::SUCCESS } else { ExitCode::FAILURE };
    }
    if let Some(n) = args.frames {
        let every: u64 = std::env::var("V_BEAR_SHOT_EVERY").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
        let mut peak = 0f32;
        // Test script: V_BEAR_INPUT="600:start,900:a" holds each button for 8 frames.
        let script: Vec<(u64, input::Button)> = std::env::var("V_BEAR_INPUT")
            .unwrap_or_default()
            .split(',')
            .filter_map(|e| {
                let (f, b) = e.split_once(':')?;
                Some((f.parse().ok()?, input::Button::from_name(b)?))
            })
            .collect();
        let mut wav: Vec<f32> = Vec::new();
        let live_from: u64 = std::env::var("V_BEAR_LIVE").ok().and_then(|v| v.parse().ok()).unwrap_or(u64::MAX);
        // V_BEAR_RESET=k presses reset (F1) before frame k.
        let reset_at: Option<u64> = std::env::var("V_BEAR_RESET").ok().and_then(|v| v.parse().ok());
        for i in 0..n {
            if reset_at == Some(i) {
                console.reset();
            }
            console.trace_live = i >= live_from;
            for &(f, b) in &script {
                console.pads[0].set(b, (f..f + 8).contains(&i));
            }
            console.run_frame();
            if std::env::var_os("V_BEAR_WAV").is_some() {
                wav.extend_from_slice(&console.audio.samples);
            }
            peak = console.audio.samples.iter().fold(peak, |m, v| m.max(v.abs()));
            if every > 0 && (i + 1) % every == 0
                && let Some(out) = std::env::var_os("V_BEAR_SHOT") {
                    let p = format!("{}-{:05}.ppm", out.to_string_lossy(), i + 1);
                    let _ = save_ppm(&console, std::path::Path::new(&p));
                }
            if std::env::var_os("V_BEAR_PCS").is_some() {
                let r = &console.bus.snd.cpu.registers;
                println!(
                    "frame {i}: PC={:08X} SR={:04X} 6502 PC={:04X} S={:02X} ovl={:X}",
                    console.cpu.pc,
                    console.cpu.get_sr(),
                    r.program_counter,
                    r.stack_pointer.0,
                    console.bus.overlay
                );
            }
        }
        let b = &console.bus;
        if std::env::var_os("V_BEAR_REGS").is_some() {
            for row in 0..32 {
                let w: Vec<String> = (0..8).map(|i| format!("{:04X}", b.video.regs[row * 8 + i])).collect();
                println!("F00{:03X}: {}", row * 16, w.join(" "));
            }
            if let Ok(a) = std::env::var("V_BEAR_SND") {
                let a = usize::from_str_radix(&a, 16).unwrap_or(0);
                let ram = &console.bus.snd.cpu.memory.ram;
                let w: Vec<String> = (0..32).map(|i| format!("{:02X}", ram[(a + i) & 0xFFFF])).collect();
                println!("SND {a:04X}: {}", w.join(" "));
            }
            if let Ok(a) = std::env::var("V_BEAR_VRAM") {
                let a = usize::from_str_radix(&a, 16).unwrap_or(0);
                let w: Vec<String> = (0..16).map(|i| format!("{:04X}", b.video.vram_word(a + i))).collect();
                println!("VRAM word {a:04X}: {}", w.join(" "));
            }
        }
        let pal = b.video.regs[256..].iter().filter(|&&c| c != 0).count();
        let vram = b.video.vram.iter().filter(|&&v| v != 0).count();
        println!(
            "{n} frames, PC={:08X} SR={:04X} irqs={:?} palette={pal}/256 vram={vram} B flags={:04X} audio peak={peak:.3} sram={} (reads {}, writes {}) faults={} halted={}",
            console.cpu.pc,
            console.cpu.get_sr(),
            b.irq_taken,
            b.video.reg(0x08),
            b.sram.iter().filter(|&&v| v != 0).count(),
            b.sram_reads,
            b.sram_writes,
            console.faults,
            console.cpu.is_halted()
        );
        if let Some(out) = std::env::var_os("V_BEAR_WAV")
            && let Err(e) = save_wav(&wav, std::path::Path::new(&out)) {
                eprintln!("v-bear: {e}");
            }
        if let Some(out) = std::env::var_os("V_BEAR_SHOT") {
            // The last frame: PNG if the name ends in .png, PPM otherwise.
            let path = std::path::Path::new(&out);
            let result = if path.extension().is_some_and(|e| e.eq_ignore_ascii_case("png")) {
                save_png(&console, path)
            } else {
                save_ppm(&console, path).map_err(|e| e.to_string())
            };
            if let Err(e) = result {
                eprintln!("v-bear: {e}");
            }
        }
        // Work RAM (64 KB, big-endian), to compare with the MiSTer simulation in reference/.
        if let Some(out) = std::env::var_os("V_BEAR_RAM")
            && let Err(e) = std::fs::write(&out, &console.bus.wram) {
                eprintln!("v-bear: {}: {e}", std::path::Path::new(&out).display());
            }
        if let Some(p) = &sav
            && let Err(e) = console.save_sram(p) {
                eprintln!("v-bear: {}: {e}", p.display());
            }
        return ExitCode::SUCCESS;
    }
    ExitCode::SUCCESS
}

/// bios/ or supracan.zip next to the ROM, in the current folder or next to
/// the executable.
fn default_bios(rom: &std::path::Path) -> Option<PathBuf> {
    let exe_dir = std::env::current_exe().ok().and_then(|e| e.parent().map(PathBuf::from));
    let dirs = [rom.parent().map(PathBuf::from), Some(PathBuf::from(".")), exe_dir];
    dirs.into_iter()
        .flatten()
        .flat_map(|d| ["bios", "supracan.zip"].map(|n| d.join(n)))
        .find(|p| p.exists())
}

/// Writes the current frame as a binary PPM.
fn save_ppm(console: &console::Console, path: &std::path::Path) -> std::io::Result<()> {
    use v_bear_core::video::{HEIGHT, WIDTH};
    let mut out = format!("P6\n{WIDTH} {HEIGHT}\n255\n").into_bytes();
    for &px in &console.bus.video.frame {
        out.extend_from_slice(&[(px >> 16) as u8, (px >> 8) as u8, px as u8]);
    }
    std::fs::write(path, out)
}

/// Writes the current frame as a PNG.
fn save_png(console: &console::Console, path: &std::path::Path) -> Result<(), String> {
    use v_bear_core::video::{HEIGHT, WIDTH};
    let rgb: Vec<u8> = console.bus.video.frame.iter().flat_map(|&px| [(px >> 16) as u8, (px >> 8) as u8, px as u8]).collect();
    image::RgbImage::from_raw(WIDTH as u32, HEIGHT as u32, rgb)
        .ok_or("frame size")?
        .save(path)
        .map_err(|e| format!("{}: {e}", path.display()))
}

/// Writes stereo f32 samples at the sampler's rate as a 16-bit WAV.
fn save_wav(samples: &[f32], path: &std::path::Path) -> std::io::Result<()> {
    let rate = v_bear_core::sound::SAMPLE_RATE.round() as u32;
    let data_len = (samples.len() * 2) as u32;
    let mut out = Vec::with_capacity(44 + data_len as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&2u16.to_le_bytes()); // stereo
    out.extend_from_slice(&rate.to_le_bytes());
    out.extend_from_slice(&(rate * 4).to_le_bytes());
    out.extend_from_slice(&4u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for &s in samples {
        out.extend_from_slice(&((s.clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes());
    }
    std::fs::write(path, out)
}
