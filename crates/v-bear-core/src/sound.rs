//! The UM6619's sound side: the 65C02, its RAM and I/O page, the two
//! joypad ports, the mailboxes with the 68000, and the 16-channel sampler.
//!
//! Follows MiSTer um6619_cpu.sv and um6619_snd.sv (docs/hardware.md
//! §3 65C02, §5, §6).

use mos6502::cpu::{CPU, WaitState};
use mos6502::instruction::Mos65C02;
use mos6502::memory::Bus as Bus6502;
use mos6502::registers::{StackPointer, Status};
use serde::{Deserialize, Serialize};

/// One output sample every 80 sound-CPU cycles: 44744.3 Hz.
pub const SAMPLE_RATE: f64 = 10_738_635.0 / 3.0 / 80.0;
const CYCLES_PER_SAMPLE: u32 = 80;

type Cpu = CPU<SoundBus, Mos65C02<true, true>>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(from = "SoundState", into = "SoundState")]
pub struct Sound {
    pub cpu: Cpu,
    /// $E9001C bit 0: the 65C02 runs (set) or is held in reset.
    running: bool,
    /// Cycles the last instruction ran past the previous budget, paid back
    /// from the next one so the 65C02 keeps CPU_CLK/3 on average.
    overshoot: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SoundBus {
    /// U11, 64 KiB (MiSTer mainboard.sv: sram_async with 16 address bits).
    pub ram: Vec<u8>,
    /// The I/O page's plain storage ($0400-$04FF).
    #[serde(with = "crate::state::bytes")]
    io: [u8; 256],
    irq_enable: u8,
    irq_status: u8,
    latch: [u8; 2],
    latch_full: [bool; 2],
    shift: [u8; 2],
    shift_ctrl: u8,
    /// Copies of $040A/$040C/$040D read back through the host port.
    pub shadow_40a: u8,
    pub shadow_40c: u8,
    pub shadow_40d: u8,
    nmi: bool,
    /// The 65C02 wrote $040A: interrupt level 6 on the 68000.
    pub mailbox: bool,
    /// Buttons held (bit set = pressed), pad bit layout.
    pub pads: [u16; 2],
    pad_sr: [u16; 2],
    pub sampler: Sampler,
}

/// The 65C02 core has no serde support: its registers, cycle count and
/// wait state are saved by hand.
#[derive(Serialize, Deserialize)]
struct SoundState {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    p: u8,
    cycles: u64,
    /// 0 running, 1 WAI, 2 STP.
    wait: u8,
    bus: SoundBus,
    running: bool,
    overshoot: u32,
}

impl From<Sound> for SoundState {
    fn from(s: Sound) -> Self {
        let r = s.cpu.registers;
        let wait = match s.cpu.wait_state() {
            WaitState::Running => 0,
            WaitState::WaitingForInterrupt => 1,
            WaitState::WaitingForReset => 2,
        };
        Self {
            a: r.accumulator,
            x: r.index_x,
            y: r.index_y,
            sp: r.stack_pointer.0,
            pc: r.program_counter,
            p: r.status.bits(),
            cycles: s.cpu.cycles,
            wait,
            bus: s.cpu.memory,
            running: s.running,
            overshoot: s.overshoot,
        }
    }
}

impl From<SoundState> for Sound {
    fn from(s: SoundState) -> Self {
        // The core keeps two things private: the wait state and the last NMI
        // level it saw (NMI is edge triggered). Both are put back by running
        // instructions on a scratch copy of the bus, which is then dropped.
        // States are taken between frames, when the core has already seen
        // the NMI line, so its last level equals the line.
        let mut scratch = s.bus.clone();
        scratch.irq_enable = 0;
        let mut cpu = CPU::new(scratch, Mos65C02);
        let pc = s.pc as usize;
        if s.running && s.bus.nmi {
            // A NOP followed by the NMI: the core now holds the line as seen.
            cpu.memory.ram[pc] = 0xEA;
            cpu.memory.nmi = true;
            cpu.registers.program_counter = s.pc;
            cpu.single_step();
        }
        if let Some(op) = [None, Some(0xCB), Some(0xDB)][s.wait.min(2) as usize] {
            cpu.memory.ram[pc] = op; // WAI or STP
            cpu.registers.program_counter = s.pc;
            cpu.single_step();
        }
        cpu.memory = s.bus;
        let r = &mut cpu.registers;
        r.accumulator = s.a;
        r.index_x = s.x;
        r.index_y = s.y;
        r.stack_pointer = StackPointer(s.sp);
        r.program_counter = s.pc;
        r.status = Status::from_bits_truncate(s.p);
        cpu.cycles = s.cycles;
        Self { cpu, running: s.running, overshoot: s.overshoot }
    }
}

impl Sound {
    pub fn new(image: Option<&[u8]>) -> Self {
        let mut ram = vec![0; 0x1_0000];
        // The internal 16 KiB image is copied into sound RAM as the machine
        // leaves reset.
        if let Some(img) = image {
            ram[..img.len()].copy_from_slice(img);
        }
        let bus = SoundBus {
            ram,
            io: [0; 256],
            irq_enable: 0,
            irq_status: 0,
            latch: [0; 2],
            latch_full: [false; 2],
            shift: [0; 2],
            shift_ctrl: 0,
            shadow_40a: 0,
            shadow_40c: 0,
            shadow_40d: 0,
            nmi: false,
            mailbox: false,
            pads: [0; 2],
            pad_sr: [0xFFFF; 2],
            sampler: Sampler::new(),
        };
        Self { cpu: CPU::new(bus, Mos65C02), running: false, overshoot: 0 }
    }

    pub fn bus(&mut self) -> &mut SoundBus {
        &mut self.cpu.memory
    }

    /// $E9001C bit 0.
    pub fn set_running(&mut self, run: bool) {
        if crate::debug::on("snd") && run != self.running {
            let m = &self.cpu.memory.ram;
            eprintln!("65 run={run} reset vector {:02X}{:02X}", m[0xFFFD], m[0xFFFC]);
        }
        if run && !self.running {
            self.cpu.reset();
        }
        if !run {
            self.cpu.memory.nmi = false;
        }
        self.running = run;
    }

    /// Vertical retrace with $E90010 bit 7 set.
    pub fn vblank_nmi(&mut self) {
        if self.running {
            self.cpu.memory.nmi = true;
        }
    }

    /// $E9000A: request to the 65C02 (I/O page IRQ bit 5).
    pub fn command(&mut self) {
        if crate::debug::on("snd") {
            eprintln!("68 cmd (irq enable {:02X})", self.cpu.memory.irq_enable);
        }
        let b = &mut self.cpu.memory;
        if b.irq_enable & 0x20 != 0 {
            b.irq_status |= 0x20;
        }
    }

    /// Runs `cycles` sound-CPU cycles; the sampler runs even while the CPU is held.
    pub fn run(&mut self, cycles: u32, out: &mut Vec<f32>) {
        let mut done = self.overshoot.min(cycles);
        self.overshoot -= done;
        while done < cycles {
            let n = if self.running {
                let before = self.cpu.cycles;
                if crate::debug::on("t65") {
                    let pc = self.cpu.registers.program_counter as usize;
                    let m = &self.cpu.memory.ram;
                    let r = &self.cpu.registers;
                    eprintln!(
                        "65 {pc:04X} {:02X} {:02X} {:02X} A={:02X} X={:02X} Y={:02X}",
                        m[pc], m[(pc + 1) & 0xFFFF], m[(pc + 2) & 0xFFFF], r.accumulator, r.index_x, r.index_y
                    );
                }
                self.cpu.single_step();
                (self.cpu.cycles - before).max(1) as u32
            } else {
                cycles - done
            };
            let b = &mut self.cpu.memory;
            b.sampler.tick(n, &b.ram, out);
            done += n;
        }
        self.overshoot += done - cycles;
    }
}

impl SoundBus {
    /// 68000 window at $E80000, one byte.
    pub fn host_read(&self, a: usize) -> u8 {
        self.ram[a & 0xFFFF]
    }

    pub fn host_write(&mut self, a: usize, v: u8) {
        if crate::debug::on("snd") && a < 0x500 {
            eprintln!("68 W {a:04X} = {v:02X}");
        }
        self.ram[a & 0xFFFF] = v;
        match a & 0xFFFF {
            // Writes to $E80404/$E80405 fill a byte latch and raise its interrupt.
            0x404 => {
                self.latch[0] = v;
                self.latch_full[0] = true;
                self.irq_status |= 0x08;
            }
            0x405 => {
                self.latch[1] = v;
                self.latch_full[1] = true;
                self.irq_status |= 0x04;
            }
            // Both CPUs write $040A; keep the host's readback copy in step.
            0x40A => self.shadow_40a = v,
            _ => {}
        }
    }

    fn irq_live(&self) -> u8 {
        (self.sampler.irq_timer as u8) << 7 | (self.sampler.irq_dma as u8) << 6 | (self.irq_status & 0x3F)
    }

    fn io_read(&mut self, r: u8) -> u8 {
        match r {
            0x02 => self.shift[0],
            0x03 => self.shift[1],
            0x04 | 0x05 => {
                let n = (r & 1) as usize;
                self.irq_status &= !(0x08 >> n);
                let v = if self.latch_full[n] { self.latch[n] } else { 0xCD };
                self.latch_full[n] = false;
                v
            }
            0x06 => 0,
            0x09 => {
                self.irq_status &= !0x10;
                0
            }
            0x0A => {
                self.irq_status &= !0x20;
                self.shadow_40a
            }
            0x10 => self.irq_enable,
            0x11 => self.irq_live(),
            0x12 => {
                self.nmi = false;
                0
            }
            0x20 => 0,
            0x22 => self.sampler.read(),
            _ => self.io[r as usize],
        }
    }

    fn io_write(&mut self, r: u8, v: u8) {
        if crate::debug::on("snd") && r != 0x07 && r != 0x20 && r != 0x22 {
            eprintln!("65 IO W {r:02X} = {v:02X}");
        }
        self.io[r as usize] = v;
        match r {
            0x07 => {
                // Falling edges: bits 1-0 latch, 3-2 shift a bit in, 5-4 clear.
                let fell = self.shift_ctrl & !v;
                self.shift_ctrl = v;
                for p in 0..2 {
                    if fell & (1 << p) != 0 {
                        // The pad's shift register: active low, bits 3-0 high.
                        self.pad_sr[p] = !self.pads[p] | 0x000F;
                    }
                    if fell & (4 << p) != 0 {
                        let bit = (self.pad_sr[p] >> 15) as u8;
                        self.shift[p] = self.shift[p] << 1 | bit;
                        self.pad_sr[p] <<= 1;
                    }
                    if fell & (0x10 << p) != 0 {
                        self.shift[p] = 0;
                    }
                }
            }
            0x0A => {
                self.shadow_40a = v;
                self.mailbox = true;
            }
            0x0C => self.shadow_40c = v,
            0x0D => self.shadow_40d = v,
            0x10 => self.irq_enable = v,
            0x20 => self.sampler.addr = v,
            0x22 => self.sampler.write(v),
            _ => {}
        }
    }
}

impl Bus6502 for SoundBus {
    fn get_byte(&mut self, address: u16) -> u8 {
        if address >> 8 == 0x04 {
            self.io_read(address as u8)
        } else {
            self.ram[address as usize & 0xFFFF]
        }
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        if crate::debug::on("snd") && (0x300..0x302).contains(&address) {
            eprintln!("65 W {address:04X} = {value:02X}");
        }
        if address >> 8 == 0x04 {
            self.io_write(address as u8, value);
        } else {
            self.ram[address as usize & 0xFFFF] = value;
        }
    }

    fn nmi_pending(&mut self) -> bool {
        self.nmi
    }

    fn irq_pending(&mut self) -> bool {
        self.irq_live() & self.irq_enable != 0
    }
}

// ---- the sampler -------------------------------------------------------------

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
struct Channel {
    pitch: u16,
    start: u16,
    length: u16,
    current: u16,
    end: u16,
    fraction: u16,
    dma: u8,
    vol_l: u8,
    vol_r: u8,
    one_shot: bool,
    active: bool,
    releasing: bool,
    attacking: bool,
    decaying: bool,
    level: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Sampler {
    #[serde(with = "crate::state::bytes")]
    regs: [u8; 256],
    ch: [Channel; 16],
    /// Register number latched through $0420.
    pub addr: u8,
    timer_running: bool,
    timer_left: u32,
    timer_phase: bool,
    pub irq_timer: bool,
    pub irq_dma: bool,
    sample_cnt: u32,
}

impl Sampler {
    fn new() -> Self {
        Self {
            regs: [0; 256],
            ch: [Channel::default(); 16],
            addr: 0,
            timer_running: false,
            timer_left: 0,
            timer_phase: false,
            irq_timer: false,
            irq_dma: false,
            sample_cnt: 0,
        }
    }

    fn timer_period(&self) -> u32 {
        let n = u16::from_le_bytes([self.regs[0x11], self.regs[0x12]]) as u32;
        5 * (0x1_0000 - n)
    }

    /// $0422 read.
    fn read(&mut self) -> u8 {
        match self.addr {
            0x14 => {
                self.irq_timer = false;
                (self.regs[0x14] & 0xBF) | (self.timer_phase as u8) << 6
            }
            0x16 => {
                self.irq_dma = false;
                let busy = self.ch.iter().any(|c| c.active && c.dma != 0);
                (self.regs[0x16] & !0x40) | (busy as u8) << 6
            }
            a => self.regs[a as usize],
        }
    }

    /// $0422 write.
    fn write(&mut self, v: u8) {
        let a = self.addr;
        self.regs[a as usize] = v;
        let c = (a & 0xF) as usize;
        match a >> 4 {
            0x1 if a == 0x14 => {
                if v & 0xC0 == 0 {
                    self.timer_running = false;
                    self.irq_timer = false;
                } else if !self.timer_running {
                    self.timer_running = true;
                    self.timer_left = self.timer_period();
                }
            }
            0x1 if a == 0x17 => self.key(v),
            0x2 => self.ch[c].pitch = (self.ch[c].pitch & 0xFF00) | v as u16,
            0x3 => self.ch[c].pitch = (self.ch[c].pitch & 0x00FF) | (v as u16) << 8,
            0x5 => {
                self.ch[c].length = 0x40 << ((v >> 1) & 7);
                self.ch[c].one_shot = v & 1 != 0;
            }
            0x6 => self.ch[c].start = (self.ch[c].start & 0x00FF) | (v as u16) << 8,
            0x7 => self.ch[c].start = (self.ch[c].start & 0xFF00) | v as u16,
            0x9 => self.ch[c].dma = v,
            0xE => {
                self.ch[c].vol_l = v & 0xF0;
                self.ch[c].vol_r = v << 4;
            }
            _ => {}
        }
    }

    /// $17: key on (bits 7-4 non-zero) or off for voice bits 3-0.
    fn key(&mut self, v: u8) {
        let c = (v & 0xF) as usize;
        let ctrl = self.regs[0x40 | c];
        let attack = self.regs[0xA0 | c] & 0x1F;
        let ch = &mut self.ch[c];
        if v >> 4 != 0 {
            if ctrl & 2 == 0 || ch.dma != 0 || attack == 31 {
                ch.level = u32::MAX;
            } else if !ch.active {
                ch.level = 0;
            }
            ch.attacking = attack != 31;
            ch.decaying = true;
            ch.releasing = false;
            ch.current = ch.start << 6;
            ch.end = (ch.start << 6).wrapping_add(ch.length);
            ch.fraction = 0;
            ch.active = true;
        } else if ctrl & 2 != 0 && ch.dma == 0 {
            // Key-off releases the loop; streams keep their stop path.
            ch.releasing = true;
        } else {
            ch.active = false;
        }
    }

    /// Advances `n` sound-CPU cycles, appending output samples (stereo f32).
    fn tick(&mut self, n: u32, ram: &[u8], out: &mut Vec<f32>) {
        for _ in 0..n {
            if self.timer_running {
                if self.timer_left <= 1 {
                    self.timer_left = self.timer_period();
                    self.timer_phase = !self.timer_phase;
                    self.irq_timer = true;
                } else {
                    self.timer_left -= 1;
                }
            }
            self.sample_cnt += 1;
            if self.sample_cnt == CYCLES_PER_SAMPLE {
                self.sample_cnt = 0;
                let (l, r) = self.mix(ram);
                out.push(l as f32 / 32768.0);
                out.push(r as f32 / 32768.0);
            }
        }
    }

    /// One output sample: every active voice in turn.
    fn mix(&mut self, ram: &[u8]) -> (i16, i16) {
        let (mut acc_l, mut acc_r) = (0i32, 0i32);
        for c in 0..16 {
            if !self.ch[c].active {
                continue;
            }
            let ctrl = self.regs[0x40 | c];
            let attack_rate = (self.regs[0xA0 | c] & 0x1F) as u32;
            let decay_rate = (self.regs[0xB0 | c] & 0x1F) as u32;
            let sustain_rate = (self.regs[0xC0 | c] & 0x1F) as u32;
            let rel = self.regs[0xD0 | c];
            let ch = &mut self.ch[c];
            let fetched = ram[ch.current as usize & 0xFFFF];
            if fetched == 0 && ch.dma != 0 && ch.one_shot {
                // A streamed sound ends on a zero byte, never a real sample.
                ch.active = false;
                self.irq_dma = true;
                continue;
            }
            let envelope = ctrl & 2 != 0 && ch.dma == 0;
            let sustain_level = u32::MAX - ((rel as u32 >> 4) << 28);
            let gap = if ch.attacking {
                !ch.level
            } else { ch.level.saturating_sub(sustain_level) };
            let (mut eff_l, mut eff_r) = (ch.vol_l as i32, ch.vol_r as i32);
            let mut held_step = None;
            if ch.releasing || envelope {
                let lvl = (ch.level >> 24) as i32;
                eff_l = (ch.vol_l as i32 * lvl + ch.vol_l as i32) >> 8;
                eff_r = (ch.vol_r as i32 * lvl + ch.vol_r as i32) >> 8;
                let shift = if ch.attacking { 18 - (attack_rate >> 1) } else { 20 - (decay_rate >> 1) };
                held_step = Some(gap >> shift);
            }
            let pcm = fetched as i32 - 128;
            acc_l += pcm * eff_l;
            acc_r += pcm * eff_r;

            // Envelope step.
            if ch.releasing {
                let rr = rel & 0xF;
                let step = if rr == 0 { u32::MAX } else { 857 << rr };
                if ch.level <= step {
                    ch.level = 0;
                    ch.active = false;
                } else {
                    ch.level -= step;
                }
            } else if let (true, Some(step)) = (envelope, held_step) {
                if ch.attacking {
                    if attack_rate != 0 {
                        if attack_rate == 31 || gap <= 0x01FF_FFFF {
                            ch.level = u32::MAX;
                            ch.attacking = false;
                        } else {
                            ch.level = ch.level.wrapping_add(step);
                        }
                    }
                } else if ch.decaying {
                    if decay_rate != 0 {
                        if gap <= 0x00FF_FFFF {
                            ch.level = sustain_level;
                            ch.decaying = false;
                        } else {
                            ch.level = ch.level.wrapping_sub(step);
                        }
                    }
                } else {
                    let sustain_step = if sustain_rate == 0 { 0 } else { 857u32 << (sustain_rate >> 1) };
                    if sustain_step != 0 {
                        if ch.level <= sustain_step {
                            ch.level = 0;
                            ch.active = false;
                        } else {
                            ch.level -= sustain_step;
                        }
                    }
                }
            }

            // Phase: 16.16 accumulator, increment pitch << 6.
            let step_sum = ch.fraction as u32 + ((ch.pitch as u32) << 6);
            ch.fraction = step_sum as u16;
            let next = ch.current.wrapping_add((step_sum >> 16) as u16);
            ch.current = next;
            if next >= ch.end {
                if ch.dma != 0 {
                    // Stream: ask for the next buffer and restart, keeping the phase.
                    self.irq_dma = true;
                    ch.current = ch.start << 6;
                    ch.end = (ch.start << 6).wrapping_add(ch.length);
                } else if ch.one_shot {
                    ch.active = false;
                } else if ch.length != 0 {
                    ch.current = next.wrapping_sub(ch.length);
                }
            }
        }
        let sat = |v: i32| (v >> 4).clamp(-32768, 32767) as i16;
        (sat(acc_l), sat(acc_r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Loads a 65C02 program at $0200 with the reset vector pointing at it.
    fn with_program(code: &[u8]) -> Sound {
        let mut snd = Sound::new(None);
        let ram = &mut snd.bus().ram;
        ram[0x200..0x200 + code.len()].copy_from_slice(code);
        ram[0xFFFC] = 0x00;
        ram[0xFFFD] = 0x02;
        snd.set_running(true);
        snd
    }

    /// Round trip through the save state form.
    fn reload(snd: &Sound) -> Sound {
        let state: SoundState = snd.clone().into();
        state.into()
    }

    #[test]
    fn save_state_keeps_wai_and_the_nmi_edge() {
        // NMI handler at $0300: INC $10, then spin. Main program: WAI.
        let mut snd = with_program(&[0xCB, 0x80, 0xFD]); // WAI; BRA back to WAI
        let ram = &mut snd.bus().ram;
        ram[0x300..0x305].copy_from_slice(&[0xE6, 0x10, 0x80, 0xFE, 0x00]); // INC $10; BRA *
        ram[0xFFFA] = 0x00;
        ram[0xFFFB] = 0x03;
        snd.run(100, &mut Vec::new());
        assert_eq!(snd.cpu.wait_state(), WaitState::WaitingForInterrupt);
        let mut copy = reload(&snd);
        assert_eq!(copy.cpu.wait_state(), WaitState::WaitingForInterrupt);
        assert_eq!(copy.cpu.registers, snd.cpu.registers);

        // The NMI wakes it once; a reloaded copy must not take it again
        // while the line stays high (it is edge triggered).
        copy.vblank_nmi();
        copy.run(100, &mut Vec::new());
        assert_eq!(copy.bus().ram[0x10], 1);
        let mut again = reload(&copy);
        again.run(1000, &mut Vec::new());
        assert_eq!(again.bus().ram[0x10], 1);
        assert_eq!(again.cpu.wait_state(), WaitState::Running);
    }

    #[test]
    fn pad_is_read_msb_first_through_the_io_page() {
        // Latch pad 1 (bit 0 falls), shift 8 bits (bit 2 falls each time),
        // then store $0402 at $0300.
        #[rustfmt::skip]
        let code = [
            0xA9, 0x3F, 0x8D, 0x07, 0x04, // LDA #$3F; STA $0407
            0xA9, 0x3E, 0x8D, 0x07, 0x04, // bit 0 falls: latch
            0xA2, 0x08,                   // LDX #8
            0xA9, 0x3E, 0x8D, 0x07, 0x04, // loop: bit 2 high
            0xA9, 0x3A, 0x8D, 0x07, 0x04, //       bit 2 falls: shift
            0xCA, 0xD0, 0xF2,             //       DEX; BNE loop
            0xAD, 0x02, 0x04, 0x8D, 0x00, 0x03, // LDA $0402; STA $0300
            0x80, 0xFE,                   // BRA *
        ];
        let mut snd = with_program(&code);
        // A and Start held: bits 15 and 13.
        snd.bus().pads[0] = 0xA000;
        snd.run(2000, &mut Vec::new());
        // Active low: A (bit 7 of the first byte) and Start (bit 5) read 0.
        assert_eq!(snd.bus().ram[0x300], 0b0101_1111);
    }

    #[test]
    fn mailbox_and_latches() {
        // Write $040A (mailbox to the 68000), then copy latch $0404 to $0301.
        #[rustfmt::skip]
        let code = [
            0xA9, 0x55, 0x8D, 0x0A, 0x04, // LDA #$55; STA $040A
            0xAD, 0x04, 0x04, 0x8D, 0x01, 0x03, // LDA $0404; STA $0301
            0x80, 0xFE,
        ];
        let mut snd = with_program(&code);
        snd.bus().host_write(0x404, 0x42);
        snd.run(100, &mut Vec::new());
        let b = snd.bus();
        assert!(b.mailbox);
        assert_eq!(b.shadow_40a, 0x55);
        assert_eq!(b.ram[0x301], 0x42);
        // Empty latch reads $CD.
        assert_eq!(b.io_read(0x04), 0xCD);
    }

    #[test]
    fn sampler_plays_a_looping_voice() {
        let mut s = Sampler::new();
        let mut ram = vec![0x80u8; 0x1_0000];
        // A square wave at $1000 (start = $1000 >> 6 = $40), 64 bytes.
        for i in 0..64 {
            ram[0x1000 + i] = if i < 32 { 0xC0 } else { 0x40 };
        }
        let mut w = |a: u8, v: u8| {
            s.addr = a;
            s.write(v);
        };
        w(0x70, 0x40); // start low
        w(0x60, 0x00); // start high
        w(0x50, 0x00); // length $40, looping
        w(0x30, 0x04); // pitch $0400: one byte per sample
        w(0xE0, 0xFF); // full volume both sides
        w(0x17, 0x10); // key on voice 0
        let mut out = Vec::new();
        s.tick(80 * 128, &ram, &mut out);
        assert_eq!(out.len(), 256);
        let peak = out.iter().fold(0f32, |m, v| m.max(v.abs()));
        // One full-volume voice peaks at 127 × 240 >> 4 = 1905 of 32768.
        assert!(peak > 0.025, "voice is silent");
        assert!(s.ch[0].active, "looping voice stopped");
    }
}
