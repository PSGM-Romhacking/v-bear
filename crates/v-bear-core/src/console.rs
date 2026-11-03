//! The machine: wires the CPUs, memory, video, sound and input together and
//! runs them line by line.

use m68k::{CpuCore, CpuType, CycleBatchControl, CycleBatchExit};

use crate::audio::Audio;
use crate::bios::Bios;
use crate::bus::{Bus, irq};
use crate::cart::Cartridge;
use crate::input::Pad;
use crate::video::{HEIGHT, LINES};

/// Master clock (crystal U13).
pub const MCLK: f64 = 53_693_175.0;
/// 3420 MCLK per line, 262 lines: ~59.92 Hz.
pub const FPS: f64 = MCLK / (3420.0 * LINES as f64);
/// 68000 cycles per line (MCLK / 5).
const CPU_CYCLES_PER_LINE: i32 = 3420 / 5;
/// The line is run in slices, alternating the 68000 and the 65C02 (CPU_CLK / 3).
const SLICES: i32 = 4;
const CPU_CYCLES_PER_SLICE: i32 = CPU_CYCLES_PER_LINE / SLICES;
const SND_CYCLES_PER_SLICE: u32 = CPU_CYCLES_PER_SLICE as u32 / 3;

/// A copy of the whole machine, taken between frames. Leaves out the ROM,
/// the BIOS and the battery SRAM, which stay as loaded.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Snapshot {
    /// The 68000 core is not Clone; it round-trips through its serde form.
    cpu: String,
    bus: Bus,
    overshoot: i32,
    frame_no: u64,
}

pub struct Console {
    pub cpu: CpuCore,
    pub bus: Bus,
    pub audio: Audio,
    pub pads: [Pad; 2],
    /// Cycles run past the end of the previous line (instructions are never split).
    overshoot: i32,
    pub frame_no: u64,
    /// Debug aid: print every instruction while set.
    pub trace_live: bool,
    /// Faults taken by the 68000 (illegal, A-line, F-line), a sign of a crash.
    pub faults: u32,
    /// CRC32 of the ROM, stamped on save states.
    rom_crc: u32,
}

impl Console {
    pub fn new(cart: Cartridge, bios: Option<Bios>) -> Self {
        let mut cpu = CpuCore::new();
        cpu.set_cpu_type(CpuType::M68000);
        let rom_crc = crate::bios::crc32(&cart.rom);
        let mut bus = Bus::new(cart, bios);
        cpu.reset(&mut bus);
        Self {
            cpu,
            bus,
            audio: Audio::new(),
            pads: [Pad::default(); 2],
            overshoot: 0,
            frame_no: 0,
            trace_live: false,
            faults: 0,
            rom_crc,
        }
    }

    /// Turns the console off and on: everything starts over except the
    /// cartridge's battery SRAM.
    pub fn reset(&mut self) {
        let cart = std::mem::take(&mut self.bus.cart);
        let mut fresh = Self::new(cart, self.bus.bios.take());
        fresh.bus.sram = std::mem::take(&mut self.bus.sram);
        fresh.bus.sram_dirty = self.bus.sram_dirty;
        fresh.bus.video.frame = std::mem::take(&mut self.bus.video.frame);
        *self = fresh;
    }

    /// Debug aid: runs `n` instructions one at a time, printing each PC.
    pub fn trace(&mut self, n: usize) {
        for _ in 0..n {
            let pc = self.cpu.pc;
            let op = self.bus.read16(pc);
            let ext: Vec<String> = (1..4).map(|i| format!("{:04X}", self.bus.read16(pc + 2 * i))).collect();
            self.cpu.set_irq(self.bus.ipl());
            let r = self.cpu.step(&mut self.bus);
            println!("{pc:06X}  {op:04X} {}  {r:?}", ext.join(" "));
        }
    }

    /// Saves the machine state in memory.
    pub fn snapshot(&self) -> Result<Snapshot, String> {
        Ok(Snapshot {
            cpu: serde_json::to_string(&self.cpu).map_err(|e| e.to_string())?,
            bus: self.bus.clone(),
            overshoot: self.overshoot,
            frame_no: self.frame_no,
        })
    }

    /// Puts back a state from `snapshot`. The battery SRAM keeps its
    /// current contents, as on a real console, but counts as changed.
    pub fn restore(&mut self, snap: &Snapshot) -> Result<(), String> {
        self.cpu = serde_json::from_str(&snap.cpu).map_err(|e| e.to_string())?;
        let mut bus = snap.bus.clone();
        bus.cart = std::mem::take(&mut self.bus.cart);
        bus.bios = self.bus.bios.take();
        bus.sram = std::mem::take(&mut self.bus.sram);
        bus.sram_dirty = true;
        // The picture on screen stays until the next frame replaces it.
        bus.video.frame = std::mem::take(&mut self.bus.video.frame);
        self.bus = bus;
        self.overshoot = snap.overshoot;
        self.frame_no = snap.frame_no;
        Ok(())
    }

    /// The machine state as a save state file's contents.
    pub fn save_state(&self) -> Result<Vec<u8>, String> {
        // Same layout as `Snapshot`, but borrowing the bus: no copy of the machine
        // (the frontend saves one every other frame for rewind).
        #[derive(serde::Serialize)]
        struct SnapshotRef<'a> {
            cpu: String,
            bus: &'a Bus,
            overshoot: i32,
            frame_no: u64,
        }
        let snap = SnapshotRef {
            cpu: serde_json::to_string(&self.cpu).map_err(|e| e.to_string())?,
            bus: &self.bus,
            overshoot: self.overshoot,
            frame_no: self.frame_no,
        };
        crate::state::encode(self.rom_crc, &snap)
    }

    /// Puts back a state from a save state file's contents.
    pub fn load_state(&mut self, data: &[u8]) -> Result<(), String> {
        let snap: Snapshot = crate::state::decode(self.rom_crc, data)?;
        self.restore(&snap)
    }

    /// Loads the battery SRAM from `path`, if the file exists.
    pub fn load_sram(&mut self, path: &std::path::Path) -> std::io::Result<bool> {
        match std::fs::read(path) {
            Ok(data) => {
                let n = data.len().min(self.bus.sram.len());
                self.bus.sram[..n].copy_from_slice(&data[..n]);
                Ok(true)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Writes the battery SRAM to `path` if the game changed it.
    pub fn save_sram(&mut self, path: &std::path::Path) -> std::io::Result<()> {
        if self.bus.sram_dirty {
            std::fs::write(path, &self.bus.sram)?;
            self.bus.sram_dirty = false;
        }
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.bus.cart.name
    }

    /// Runs the machine for one video frame.
    pub fn run_frame(&mut self) {
        for (i, pad) in self.pads.iter().enumerate() {
            self.bus.snd.bus().pads[i] = pad.bits();
        }
        self.audio.samples.clear();
        if crate::debug::on("vregs") {
            eprintln!("--- frame {}", self.frame_no + 1);
        }
        for line in 0..LINES {
            self.start_line(line);
            // The UM6618 builds the next line into a line buffer while this
            // one is shown (MiSTer: render_y = vpos + 1), so line L sees the
            // registers and VRAM as they are when line L-1 begins.
            let next = (line + 1) % LINES;
            if (next as usize) < HEIGHT {
                self.bus.video.render_line(next as usize);
            }
            for slice in 0..SLICES {
                self.run_cpu(slice);
                self.bus.snd.run(SND_CYCLES_PER_SLICE, &mut self.audio.samples);
                if std::mem::take(&mut self.bus.snd.bus().mailbox) {
                    self.bus.raise(irq::MAILBOX);
                }
            }
        }
        self.bus.video.end_frame();
        self.bus.video.odd_frame = !self.bus.video.odd_frame;
        self.frame_no += 1;
    }

    /// Interrupt events at the start of a line (MiSTer um6618.sv).
    fn start_line(&mut self, line: u16) {
        let bus = &mut self.bus;
        bus.video.vpos = line;
        if line == HEIGHT as u16 {
            bus.raise(irq::VBLANK);
            bus.frc_vblank();
            if bus.irq_mask & 0x80 != 0 {
                bus.snd.vblank_nmi();
            }
        }
        if line != 0 && line < HEIGHT as u16 {
            bus.raise(irq::LINE);
        }
        // Raster compare: "line on" raises IRQ5, "line off" drops it.
        let on = bus.video.reg(0x0A);
        let off = bus.video.reg(0x0C);
        if on & 0x8000 != 0 && line & 0xFF == on & 0xFF {
            bus.raise(irq::LINE_ONOFF);
        }
        if off & 0x8000 != 0 && line & 0xFF == off & 0xFF {
            bus.clear(irq::LINE_ONOFF);
        }
    }

    fn run_cpu(&mut self, slice: i32) {
        let slice_start = slice * CPU_CYCLES_PER_SLICE;
        let mut budget = CPU_CYCLES_PER_SLICE - self.overshoot;
        while budget > 0 {
            // DMA holds the CPU off the bus.
            let stall = std::mem::take(&mut self.bus.stall) as i32;
            if stall > 0 {
                self.bus.tick_frc(stall as u32);
                budget -= stall;
                continue;
            }
            self.cpu.set_irq(self.bus.ipl());
            let trace = self.trace_live;
            let result = self.cpu.run_for_cycles_with_hook(&mut self.bus, budget, |cpu, bus, _| {
                if trace {
                    let op = bus.read16(cpu.pc);
                    let ext = bus.read16(cpu.pc + 2);
                    let ext2 = bus.read16(cpu.pc + 4);
                    eprintln!("{:06X} {op:04X} {ext:04X} {ext2:04X} D0={:08X} A0={:08X}", cpu.pc & 0xFF_FFFF, cpu.dar[0], cpu.dar[8]);
                }
                cpu.set_irq(bus.ipl());
                if bus.stall > 0 { CycleBatchControl::Return } else { CycleBatchControl::Continue }
            });
            let mut used = result.cycles;
            match result.exit {
                CycleBatchExit::Stopped => used = used.max(budget), // waits for an interrupt
                CycleBatchExit::TrapInstruction { trap_num } => {
                    used += self.cpu.take_trap_exception(&mut self.bus, trap_num)
                }
                CycleBatchExit::AlineTrap { .. } => {
                    self.faults += 1;
                    used += self.cpu.take_aline_exception(&mut self.bus)
                }
                CycleBatchExit::FlineTrap { .. } => {
                    self.faults += 1;
                    used += self.cpu.take_fline_exception(&mut self.bus)
                }
                CycleBatchExit::Breakpoint { .. } => used += self.cpu.take_bkpt_exception(&mut self.bus),
                CycleBatchExit::IllegalInstruction { .. } => {
                    self.faults += 1;
                    used += self.cpu.take_illegal_exception(&mut self.bus)
                }
                CycleBatchExit::BudgetExhausted | CycleBatchExit::BoundaryRequested => {}
            }
            self.bus.tick_frc(used.max(0) as u32);
            budget -= used;
            let pos = slice_start + (CPU_CYCLES_PER_SLICE - budget).clamp(0, CPU_CYCLES_PER_SLICE);
            // 342 dots a line in H256, 420 in H320 (MiSTer um6618_vtiming.sv).
            let dots = if self.bus.video.reg(0x08) & 0x0100 != 0 { 420 } else { 342 };
            self.bus.video.hpos = (pos as u32 * dots / CPU_CYCLES_PER_LINE as u32) as u16;
        }
        self.overshoot = -budget;
    }
}
