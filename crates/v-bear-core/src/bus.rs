//! The 68000 bus: address decode, the UM6619 host port, interrupts, the
//! free running counter and the two host DMA channels.
//!
//! Behaviour follows the MiSTer core (docs/hardware.md §3, §6).

use serde::{Deserialize, Serialize};

use crate::bios::Bios;
use crate::cart::Cartridge;
use crate::sound::Sound;
use crate::video::Video;

/// CPU clock: MCLK / 5.
pub const CPU_HZ: u32 = 10_738_635;
/// Wait states on the sound RAM window: two byte cycles through the arbiter
/// the 65C02 has priority on (MiSTer um6619.sv/um6619_cpu.sv). Speedy Dragon
/// races its driver's start-up against a 68000 timeout and needs them: with
/// no wait it reloads the driver forever, one is enough, two leave a margin.
const SNDRAM_WAIT: u32 = 2;

/// Interrupt sources, by 68000 level.
pub mod irq {
    pub const FRC: u8 = 3;
    pub const LINE: u8 = 4;
    pub const LINE_ONOFF: u8 = 5;
    pub const MAILBOX: u8 = 6;
    pub const VBLANK: u8 = 7;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Bus {
    /// Not part of a save state: the ROM and BIOS come from the files loaded.
    #[serde(skip)]
    pub cart: Cartridge,
    #[serde(skip)]
    pub bios: Option<Bios>,
    pub wram: Vec<u8>,
    /// The 65C02, its RAM (U11, shared with the 68000 at $E80000) and the sampler.
    pub snd: Sound,
    /// U2, 32 KiB battery SRAM on the cartridge's low byte lane. Kept in its
    /// own file (.sav), not in save states.
    #[serde(skip)]
    pub sram: Vec<u8>,
    /// The SRAM changed since it was last saved.
    #[serde(skip)]
    pub sram_dirty: bool,
    /// UM6650 lockout chip: address register and its 32 bytes of RAM.
    lock_addr: u8,
    lock_ram: [u8; 32],
    pub video: Video,

    /// $E9001C: bit 0 sound CPU run, bit 1 low overlay off, bit 3 high overlay off.
    pub overlay: u16,
    /// $E90010: bit 7 gates VBL (IRQ7 and the 65C02 NMI), bit 4 gates IRQ4.
    pub irq_mask: u16,
    /// Pending requests, bit n = level n. Each is held until acknowledged.
    pub irq_req: u8,
    pub frc: Frc,
    dma: [DmaChannel; 2],
    /// $E90B3C, written and read once by the IPL.
    egg: u16,
    /// CPU cycles the bus held the CPU (DMA), to charge to the scheduler.
    pub stall: u32,
    /// Interrupts taken, by level (debug statistics).
    pub irq_taken: [u32; 8],
    /// Battery SRAM reads and writes by the 68000 (debug statistics, not saved).
    #[serde(skip)]
    pub sram_reads: u32,
    #[serde(skip)]
    pub sram_writes: u32,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Frc {
    pub ctrl: u16,
    pub reload: u16,
    pub value: u16,
    sub: u32,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
struct DmaChannel {
    src: u32,
    dst: u32,
    count: u16,
    ctrl: u16,
}

impl Bus {
    pub fn new(cart: Cartridge, bios: Option<Bios>) -> Self {
        // Without the BIOS both overlays start off (MiSTer's BIOS_BYPASS):
        // the 68000 boots straight from the cartridge.
        let overlay = if bios.is_some() { 0 } else { 0b1010 };
        let snd = Sound::new(bios.as_ref().map(|b| b.sound.as_slice()));
        Self {
            cart,
            bios,
            wram: vec![0; 0x1_0000],
            snd,
            sram: vec![0; 0x8000],
            sram_dirty: false,
            lock_addr: 0x7F,
            lock_ram: [0; 32],
            video: Video::new(),
            overlay,
            irq_mask: 0,
            irq_req: 0,
            frc: Frc::default(),
            dma: [DmaChannel::default(); 2],
            egg: 0,
            stall: 0,
            irq_taken: [0; 8],
            sram_reads: 0,
            sram_writes: 0,
        }
    }

    /// The level presented on !IPL: the highest pending, masked source.
    pub fn ipl(&self) -> u8 {
        let mut req = self.irq_req;
        if self.irq_mask & 0x80 == 0 {
            req &= !(1 << irq::VBLANK);
        }
        if self.irq_mask & 0x10 == 0 {
            req &= !(1 << irq::LINE);
        }
        if req == 0 { 0 } else { 7 - req.leading_zeros() as u8 }
    }

    pub fn raise(&mut self, level: u8) {
        self.irq_req |= 1 << level;
    }

    pub fn clear(&mut self, level: u8) {
        self.irq_req &= !(1 << level);
    }

    // ---- reads ------------------------------------------------------------

    /// Two mask ROM sockets selected by A21 (board PCCAR 16003-1B). Each
    /// ROM repeats across its 2 MiB window; an empty U6 reads as the
    /// connector's pull-ups (MiSTer cart_slot.sv).
    fn rom_word(&self, addr: u32) -> u16 {
        const SOCKET: usize = 0x20_0000;
        let rom = &self.cart.rom;
        let (base, size) = if addr & 0x20_0000 == 0 {
            (0, rom.len().min(SOCKET))
        } else if rom.len() > SOCKET {
            (SOCKET, rom.len() - SOCKET)
        } else {
            return 0xFFFF;
        };
        let mask = size.next_power_of_two() - 1;
        let a = base + ((addr as usize & mask & 0x1F_FFFE) % size);
        u16::from_be_bytes([rom[a], rom[a + 1]])
    }

    fn ipl_word(&self, addr: u32) -> Option<u16> {
        let bios = self.bios.as_ref()?;
        let a = addr as usize & 0xFFE;
        Some(u16::from_be_bytes([bios.ipl[a], bios.ipl[a + 1]]))
    }

    pub fn read16(&mut self, addr: u32) -> u16 {
        let addr = addr & 0xFF_FFFE;
        match addr >> 16 {
            0x00..=0x3F => {
                if addr < 0x1000 && self.overlay & 0b0010 == 0
                    && let Some(w) = self.ipl_word(addr) {
                        return w;
                    }
                self.rom_word(addr)
            }
            0xE8 => {
                let a = addr as usize & 0xFFFE;
                self.stall += SNDRAM_WAIT;
                if crate::debug::on("sndr") {
                    eprintln!("68 R {a:04X}");
                }
                let snd = self.snd.bus();
                // Without the BIOS there is no sound driver: fake its "ready"
                // flag at $0300, which every game polls after boot.
                if self.bios.is_none() && a == 0x300 {
                    return 0xFF00 | snd.host_read(0x301) as u16;
                }
                u16::from_be_bytes([snd.host_read(a), snd.host_read(a + 1)])
            }
            0xE9 => {
                if addr & 0xFFE0 == 0 {
                    self.host_read(addr)
                } else if addr & 0xFFE0 == 0x20 {
                    self.dma_read(addr)
                } else if addr & 0xFFFE == 0x0B3C {
                    self.egg
                } else {
                    0
                }
            }
            0xEB if addr & 0xFFFC == 0x0D00 => self.lockout_read(addr) as u16,
            0xEC => {
                self.sram_reads += 1;
                self.sram[(addr as usize >> 1) & 0x7FFF] as u16 | 0xFF00
            }
            0xF0 if addr & 0xFC00 == 0 => self.video.read_reg(addr),
            0xF4 | 0xF5 => self.video.read_vram(addr),
            0xF8..=0xFB => {
                if addr & 0xFF_F000 == 0xF8_0000 && self.overlay & 0b1000 == 0
                    && let Some(w) = self.ipl_word(addr) {
                        return w;
                    }
                self.rom_word(addr)
            }
            0xFC..=0xFF => {
                let a = addr as usize & 0xFFFE;
                u16::from_be_bytes([self.wram[a], self.wram[a + 1]])
            }
            // !BERR is tied high: every cycle is answered, unmapped reads give 0.
            _ => {
                if crate::debug::on("unmapped") {
                    eprintln!("unmapped read  {addr:06X}");
                }
                0
            }
        }
    }

    fn host_read(&mut self, addr: u32) -> u16 {
        match (addr >> 1) & 0xF {
            0x2 => {
                let snd = self.snd.bus();
                u16::from_be_bytes([snd.shadow_40c, snd.shadow_40d])
            }
            0x6 => {
                // $040A in both halves; reading acknowledges the mailbox IRQ.
                self.clear(irq::MAILBOX);
                let b = self.snd.bus().shadow_40a as u16;
                b << 8 | b
            }
            0x8 => self.irq_mask,
            0xA => self.frc.ctrl,
            0xB => self.frc.reload,
            0xC => {
                self.clear(irq::FRC);
                self.frc.value
            }
            0xE => self.overlay,
            _ => 0,
        }
    }

    fn dma_read(&self, addr: u32) -> u16 {
        let ch = &self.dma[(addr as usize >> 4) & 1];
        match (addr >> 1) & 7 {
            0 => (ch.src >> 16) as u16,
            1 => ch.src as u16,
            2 => (ch.dst >> 16) as u16,
            3 => ch.dst as u16,
            4 => ch.count,
            5 => ch.ctrl,
            _ => 0,
        }
    }

    // ---- writes -----------------------------------------------------------

    /// `mask` selects the byte lanes: 0xFF00 upper, 0x00FF lower, 0xFFFF both.
    pub fn write16(&mut self, addr: u32, data: u16, mask: u16) {
        let addr = addr & 0xFF_FFFE;
        match addr >> 16 {
            0xE8 => {
                let a = addr as usize & 0xFFFE;
                self.stall += SNDRAM_WAIT;
                let snd = self.snd.bus();
                if mask & 0xFF00 != 0 {
                    snd.host_write(a, (data >> 8) as u8);
                }
                if mask & 0x00FF != 0 {
                    snd.host_write(a + 1, data as u8);
                }
            }
            0xE9 => {
                if addr & 0xFFE0 == 0 {
                    self.host_write(addr, data);
                } else if addr & 0xFFE0 == 0x20 {
                    self.dma_write(addr, data, mask);
                } else if addr & 0xFFFE == 0x0B3C {
                    self.egg = data;
                }
            }
            0xEB if addr & 0xFFFC == 0x0D00 => {
                if mask & 0x00FF != 0 {
                    self.lockout_write(addr, data as u8);
                }
            }
            0xEC => {
                if mask & 0x00FF != 0 {
                    self.sram_writes += 1;
                    let a = (addr as usize >> 1) & 0x7FFF;
                    if self.sram[a] != data as u8 {
                        self.sram[a] = data as u8;
                        self.sram_dirty = true;
                    }
                }
            }
            // The UM6618 has no !UDS/!LDS: every access is a whole word (a
            // byte write carries the byte on both halves of the bus).
            0xF0 if addr & 0xFC00 == 0 => {
                if let Some(cmd) = self.video.write_reg(addr, data) {
                    self.sprite_dma(cmd);
                }
            }
            0xF4 | 0xF5 => self.video.write_vram(addr, data),
            0xFC..=0xFF => {
                let a = addr as usize & 0xFFFE;
                if mask & 0xFF00 != 0 {
                    self.wram[a] = (data >> 8) as u8;
                }
                if mask & 0x00FF != 0 {
                    self.wram[a + 1] = data as u8;
                }
            }
            _ => {
                if crate::debug::on("unmapped") {
                    eprintln!("unmapped write {addr:06X} = {data:04X} (mask {mask:04X})");
                }
            }
        }
    }

    /// The host port takes the whole data bus whatever the strobes: a byte
    /// write carries the byte on both halves, and games write these
    /// registers as bytes (Sango Fighter's $E90010 = $B8 enables IRQ4).
    fn host_write(&mut self, addr: u32, data: u16) {
        match (addr >> 1) & 0xF {
            // $E9000A: request to the 65C02 (its I/O page IRQ bit 5).
            0x5 => self.snd.command(),
            0x8 => {
                if crate::debug::on("irq") {
                    eprintln!("IRQ mask {:04X} -> {data:04X}", self.irq_mask);
                }
                self.irq_mask = data;
            }
            0xA => self.frc.write_ctrl(data),
            0xB => self.frc.reload = data,
            0xE => {
                // Bits 1 and 3 are one-way latches until reset.
                let v = data & 0xF;
                self.overlay = (v & 0b0101) | ((self.overlay | v) & 0b1010);
                // Bit 0 releases or holds the 65C02 (only with the BIOS's driver).
                if self.bios.is_some() {
                    self.snd.set_running(self.overlay & 1 != 0);
                }
            }
            _ => {}
        }
    }

    fn dma_write(&mut self, addr: u32, data: u16, mask: u16) {
        let n = (addr as usize >> 4) & 1;
        let ch = &mut self.dma[n];
        let index = (addr >> 1) & 7;
        let old = match index {
            0 => (ch.src >> 16) as u16,
            1 => ch.src as u16,
            2 => (ch.dst >> 16) as u16,
            3 => ch.dst as u16,
            4 => ch.count,
            5 => ch.ctrl,
            _ => return,
        };
        let v = merge(old, data, mask);
        match index {
            0 => ch.src = (ch.src & 0xFFFF) | (v as u32) << 16,
            1 => ch.src = (ch.src & 0xFFFF_0000) | v as u32,
            2 => ch.dst = (ch.dst & 0xFFFF) | (v as u32) << 16,
            3 => ch.dst = (ch.dst & 0xFFFF_0000) | v as u32,
            4 => ch.count = v,
            _ => {
                ch.ctrl = v;
                // Only a write touching the low byte triggers.
                if mask & 0x00FF != 0 && v & 0x8800 != 0 {
                    self.host_dma(n);
                }
            }
        }
    }

    /// Host DMA: count+1 word units. Bit 12 clear repeats the source word
    /// (fill), bit 10/9 count destination/source down, bit 8 pulls the
    /// destination back every 16 bytes (eight-word sprite records).
    fn host_dma(&mut self, n: usize) {
        let DmaChannel { mut src, mut dst, count, ctrl } = self.dma[n];
        if crate::debug::on("dma") {
            eprintln!("DMA{n} {src:08X} -> {dst:08X} x{:X} ctrl {ctrl:04X}", count as u32 + 1);
        }
        for _ in 0..=count {
            let w = self.read16(src);
            self.write16(dst, w, 0xFFFF);
            if ctrl & 0x1000 != 0 {
                src = if ctrl & 0x0200 != 0 { src.wrapping_sub(2) } else { src.wrapping_add(2) };
            }
            dst = if ctrl & 0x0400 != 0 { dst.wrapping_sub(2) } else { dst.wrapping_add(2) };
            if ctrl & 0x0100 != 0 && dst & 0xF == 0 {
                dst = dst.wrapping_sub(0x10);
            }
        }
        // Two four-clock bus cycles per unit, with the CPU stopped.
        self.stall += (count as u32 + 1) * 8;
    }

    /// Sprite DMA, programmed at $F00010-$F0001E and run by the UM6619.
    fn sprite_dma(&mut self, cmd: crate::video::SpriteDma) {
        let mut src = cmd.src;
        let mut dst = cmd.dst;
        let local = (0xF4_0000..0xF6_0000).contains(&(dst & 0xFF_FFFF));
        if crate::debug::on("sdma") {
            eprintln!("SDMA {src:08X} -> {dst:08X} x{:X} line {}", cmd.count as u32 + 1, self.video.vpos);
        }
        for _ in 0..=cmd.count {
            let w = if cmd.fill { 0 } else { self.read16(src) };
            if local {
                self.video.write_vram(dst, w);
            } else {
                self.write16(dst, w, 0xFFFF);
            }
            if !cmd.fill {
                src = src.wrapping_add(cmd.src_inc as u32 * 2);
            }
            dst = dst.wrapping_add(cmd.dst_inc as u32 * 2);
        }
        self.video.dma_done(src, dst);
        // A word bound for the chip's own VRAM costs one four-clock bus cycle (the read): the UM6618
        // writes it on its own bus, and up to two words are in flight (MiSTer um6619_dma.sv). A word
        // for the CPU bus costs a read and a write.
        let per: u32 = if local { 4 } else { 8 };
        self.stall += (cmd.count as u32 + 1) * per;
    }

    /// UM6650: $EB0D03 is the address port, $EB0D01 the data port.
    /// $20-$2F hold the key, $40-$5F are RAM, the rest reads zero.
    fn lockout_read(&self, addr: u32) -> u8 {
        if addr & 2 != 0 {
            return self.lock_addr;
        }
        let a = self.lock_addr as usize;
        match a {
            0x20..=0x2F => self.bios.as_ref().map_or(0, |b| b.key[a - 0x20]),
            0x40..=0x5F => self.lock_ram[a - 0x40],
            _ => 0,
        }
    }

    fn lockout_write(&mut self, addr: u32, v: u8) {
        if addr & 2 != 0 {
            self.lock_addr = v & 0x7F;
        } else if let 0x40..=0x5F = self.lock_addr {
            self.lock_ram[self.lock_addr as usize - 0x40] = v;
        }
    }

    /// Advances the free running counter by `cycles` CPU clocks.
    pub fn tick_frc(&mut self, cycles: u32) {
        if self.frc.tick(cycles) {
            self.raise(irq::FRC);
        }
    }

    /// Vertical retrace edge (also clocks the FRC in frame mode).
    pub fn frc_vblank(&mut self) {
        if self.frc.vblank() {
            self.raise(irq::FRC);
        }
    }
}

impl Frc {
    fn write_ctrl(&mut self, ctrl: u16) {
        self.ctrl = ctrl;
        // Starting the counter loads it from the period register.
        if ctrl >> 8 == 0xA2 || ctrl == 0xA300 {
            self.value = self.reload;
            self.sub = 0;
        }
    }

    fn mode(&self) -> Option<u32> {
        if self.ctrl >> 8 != 0xA2 {
            return None;
        }
        // Mode 0: once a second; 1: 1024 ticks per count; F: 9040.
        match self.ctrl & 0xF {
            0 => Some(CPU_HZ),
            1 => Some(1024),
            0xF => Some(9040),
            _ => None,
        }
    }

    /// Counts one unit; true when the interrupt fires.
    fn count(&mut self, once_a_second: bool) -> bool {
        if once_a_second {
            return true;
        }
        if self.value == 0 {
            self.value = self.reload;
            true
        } else {
            self.value -= 1;
            false
        }
    }

    fn tick(&mut self, cycles: u32) -> bool {
        let Some(limit) = self.mode() else { return false };
        self.sub += cycles;
        let mut fired = false;
        while self.sub >= limit {
            self.sub -= limit;
            fired |= self.count(self.ctrl & 0xF == 0);
        }
        fired
    }

    fn vblank(&mut self) -> bool {
        self.ctrl == 0xA300 && self.count(false)
    }
}

fn merge(old: u16, data: u16, mask: u16) -> u16 {
    (old & !mask) | (data & mask)
}

impl m68k::AddressBus for Bus {
    fn read_byte(&mut self, addr: u32) -> u8 {
        let w = self.read16(addr);
        if addr & 1 == 0 { (w >> 8) as u8 } else { w as u8 }
    }

    fn write_byte(&mut self, addr: u32, val: u8) {
        // The 68000 puts a byte on both halves of the data bus.
        let data = (val as u16) << 8 | val as u16;
        let mask = if addr & 1 == 0 { 0xFF00 } else { 0x00FF };
        self.write16(addr, data, mask);
    }

    fn read_word(&mut self, addr: u32) -> u16 {
        self.read16(addr)
    }

    fn write_word(&mut self, addr: u32, val: u16) {
        self.write16(addr, val, 0xFFFF);
    }

    fn read_long(&mut self, addr: u32) -> u32 {
        (self.read16(addr) as u32) << 16 | self.read16(addr.wrapping_add(2)) as u32
    }

    fn write_long(&mut self, addr: u32, val: u32) {
        self.write16(addr, (val >> 16) as u16, 0xFFFF);
        self.write16(addr.wrapping_add(2), val as u16, 0xFFFF);
    }

    /// Every source is level held until the 68000 takes its interrupt
    /// (autovectored through !VPA).
    fn interrupt_acknowledge(&mut self, level: u8) -> u32 {
        self.clear(level);
        self.irq_taken[level as usize & 7] += 1;
        u32::MAX
    }
}
