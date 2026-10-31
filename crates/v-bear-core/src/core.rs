//! The system behind the `emu-api` contract: what the frontend (and the libretro core)
//! see. This is the only file that knows both sides; the machine's own types stay
//! behind it.

use emu_api::{BiosFile, Button, Core, Frame, PadButton, SystemInfo};

use crate::bios::{self, Bios};
use crate::cart::Cartridge;
use crate::console::Console;
use crate::input::Button as Pad;
use crate::video::{HEIGHT, WIDTH};

/// 53.693175 MHz master clock, 3420 clocks per line, 262 lines: ~59.92 Hz.
const FPS_NUM: u64 = 53_693_175;
const FPS_DEN: u64 = 3420 * 262;

/// Work RAM, 64 KiB, seen from $FC0000 (mirrored up to $FFFFFF).
const RAM_ADDRESS: u32 = 0xFC_0000;

pub static INFO: SystemInfo = SystemInfo {
    name: "Funtech Super A'Can",
    short_name: "Super A'Can",
    // the first is the one a joined ROM is saved with (Tools › Join game parts…); both hold the same bytes
    extensions: &["can", "bin"],
    thumbnails: "Funtech - Super Acan",
    cover_aspect: 0.66,
    players: 2,
    // The SNES-style pad: A right, B bottom, X top, Y left, L/R shoulders.
    buttons: &[
        Button { id: PadButton::Up, label: "Up" },
        Button { id: PadButton::Down, label: "Down" },
        Button { id: PadButton::Left, label: "Left" },
        Button { id: PadButton::Right, label: "Right" },
        Button { id: PadButton::A, label: "A" },
        Button { id: PadButton::B, label: "B" },
        Button { id: PadButton::X, label: "X" },
        Button { id: PadButton::Y, label: "Y" },
        Button { id: PadButton::L, label: "L" },
        Button { id: PadButton::R, label: "R" },
        Button { id: PadButton::Select, label: "Select" },
        Button { id: PadButton::Start, label: "Start" },
    ],
    // H320 × 240; H256 modes are stretched to the same width, as a TV shows them.
    width: WIDTH,
    height: HEIGHT,
    aspect: 4.0 / 3.0,
    ram_address: RAM_ADDRESS,
    fps_num: FPS_NUM,
    fps_den: FPS_DEN,
    game_info: crate::games::game_info,
    game_texts: crate::games::TEXTS,
    bios: &[
        BiosFile { name: "internal_68k.bin", crc32: bios::IPL_CRC, size: 4096, required: true },
        BiosFile { name: "internal_6502_1.bin", crc32: bios::SND1_CRC, size: 8192, required: true },
        BiosFile { name: "internal_6502_2.bin", crc32: bios::SND2_CRC, size: 8192, required: true },
        BiosFile { name: "umc6650.bin", crc32: bios::KEY_CRC, size: 16, required: true },
    ],
    // Super Dragon Force comes as two mask ROMs.
    multi_file_roms: true,
    create: |rate| Box::new(VBearCore::new(rate)),
};

/// RetroPad → A'Can pad.
const PAD_MAP: [(PadButton, Pad); 12] = [
    (PadButton::A, Pad::A),
    (PadButton::B, Pad::B),
    (PadButton::X, Pad::X),
    (PadButton::Y, Pad::Y),
    (PadButton::L, Pad::L),
    (PadButton::R, Pad::R),
    (PadButton::Select, Pad::Select),
    (PadButton::Start, Pad::Start),
    (PadButton::Up, Pad::Up),
    (PadButton::Down, Pad::Down),
    (PadButton::Left, Pad::Left),
    (PadButton::Right, Pad::Right),
];

pub struct VBearCore {
    sample_rate: u32,
    bios: Option<Bios>,
    console: Option<Console>,
    /// Shown before a game is loaded.
    blank: Vec<u32>,
}

impl VBearCore {
    pub fn new(sample_rate: u32) -> Self {
        Self { sample_rate, bios: None, console: None, blank: vec![0; WIDTH * HEIGHT] }
    }

    /// The machine, for the app's command-line tools (tests, captures).
    pub fn console(&mut self) -> Option<&mut Console> {
        self.console.as_mut()
    }
}

impl Core for VBearCore {
    fn info(&self) -> &'static SystemInfo {
        &INFO
    }

    fn set_bios(&mut self, files: &[Option<Vec<u8>>]) {
        let files: Vec<Vec<u8>> = files.iter().flatten().cloned().collect();
        self.bios = Bios::from_files(&files).ok();
    }

    fn load(&mut self, rom: &[u8]) -> Result<(), String> {
        let cart = Cartridge::from_bytes(rom.to_vec(), String::new()).map_err(|e| e.to_string())?;
        self.console = Some(Console::new(cart, self.bios.clone()));
        Ok(())
    }

    fn reset(&mut self) {
        if let Some(c) = &mut self.console {
            c.reset();
        }
    }

    fn run_frame(&mut self, pads: &[u16]) {
        let Some(c) = &mut self.console else { return };
        for (pad, &bits) in c.pads.iter_mut().zip(pads) {
            for (from, to) in PAD_MAP {
                pad.set(to, bits & from.bit() != 0);
            }
        }
        c.run_frame();
    }

    fn video(&self) -> Frame<'_> {
        let pixels = self.console.as_ref().map_or(&self.blank, |c| &c.bus.video.frame);
        Frame { pixels, width: WIDTH, height: HEIGHT, pitch: WIDTH, aspect: INFO.aspect }
    }

    fn take_audio(&mut self, out: &mut Vec<f32>) {
        if let Some(c) = &mut self.console {
            out.extend(c.audio.resampled(self.sample_rate));
            c.audio.samples.clear();
        }
    }

    fn save_state(&self) -> Vec<u8> {
        self.console.as_ref().and_then(|c| c.save_state().ok()).unwrap_or_default()
    }

    fn load_state(&mut self, data: &[u8]) -> Result<(), String> {
        self.console.as_mut().ok_or("no game loaded")?.load_state(data)
    }

    fn system_ram(&self) -> &[u8] {
        self.console.as_ref().map_or(&[], |c| &c.bus.wram)
    }

    fn system_ram_mut(&mut self) -> &mut [u8] {
        match &mut self.console {
            Some(c) => &mut c.bus.wram,
            None => &mut [],
        }
    }

    fn battery_ram(&self) -> Option<&[u8]> {
        self.console.as_ref().map(|c| c.bus.sram.as_slice())
    }

    fn battery_ram_mut(&mut self) -> Option<&mut [u8]> {
        self.console.as_mut().map(|c| c.bus.sram.as_mut_slice())
    }

    fn load_battery(&mut self, data: &[u8]) {
        if let Some(c) = &mut self.console {
            let n = data.len().min(c.bus.sram.len());
            c.bus.sram[..n].copy_from_slice(&data[..n]);
        }
    }
}
