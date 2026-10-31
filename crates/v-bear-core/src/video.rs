//! UM6618 video chip: registers, palette, VRAM and (from stage 2) the
//! renderer. See docs/hardware.md §4.

/// Largest visible area: H320 × 240 lines (H256 and 224 lines also exist).
pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;
/// Lines per frame; vertical retrace starts at line 240.
pub const LINES: u16 = 262;
/// Lines blanked at the top and bottom in the 224-line mode.
const OVERSCAN_CROP: usize = 8;

/// A sprite DMA command, run by the UM6619 on the 68000 bus.
pub struct SpriteDma {
    pub src: u32,
    pub dst: u32,
    pub count: u16,
    pub src_inc: u16,
    pub dst_inc: u16,
    pub fill: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Video {
    /// $F00000-$F003FF: 256 register words, then 256 palette words (xBGR555).
    pub regs: Vec<u16>,
    pub vram: Vec<u8>,
    pub vpos: u16,
    pub hpos: u16,
    pub odd_frame: bool,
    /// 0RGB8888, one u32 per pixel, row by row. Output only, left out of
    /// save states.
    #[serde(skip)]
    pub frame: Vec<u32>,
    /// ROZ X origin latched at line 0, used while line tables are on.
    pub roz_frame_x: i32,
    /// A mask-only sprite was drawn during the last frame / this frame.
    pub sprite_masked_frame: bool,
    sprite_masked_acc: bool,
}

impl Video {
    pub fn new() -> Self {
        Self {
            regs: vec![0; 512],
            vram: vec![0; 0x2_0000],
            vpos: 0,
            hpos: 0,
            odd_frame: false,
            frame: vec![0; WIDTH * HEIGHT],
            roz_frame_x: 0,
            sprite_masked_frame: false,
            sprite_masked_acc: false,
        }
    }

    pub fn reg(&self, offset: u32) -> u16 {
        self.regs[(offset as usize >> 1) & 0x1FF]
    }

    pub fn read_reg(&self, addr: u32) -> u16 {
        match addr & 0x3FE {
            0x00 => {
                let mut v = if self.vpos >= HEIGHT as u16 { 0x8000 } else { 0 };
                if self.odd_frame {
                    v |= 2;
                }
                v
            }
            0x02 => self.vpos,
            0x04 => self.hpos,
            a => self.reg(a),
        }
    }

    /// Returns a sprite DMA command when $F0001E is written with bit 15 set.
    pub fn write_reg(&mut self, addr: u32, data: u16) -> Option<SpriteDma> {
        let a = addr & 0x3FE;
        if crate::debug::on("vregs") && a < 0x200 && !(0x10..0x20).contains(&a) {
            eprintln!("VREG {a:03X} = {data:04X} line {}", self.vpos);
        }
        // palette: entry, value and line, to compare with the MiSTer simulation (reference/)
        if crate::debug::on("pal") && a >= 0x200 {
            eprintln!("PAL {:02X} = {data:04X} line {}", (a - 0x200) >> 1, self.vpos);
        }
        self.regs[a as usize >> 1] = data;
        if a != 0x1E || data & 0x8000 == 0 {
            return None;
        }
        let r = |o: u32| self.reg(o) as u32;
        let mut dst = r(0x12) << 16 | r(0x14);
        // Bits 14-13 force the destination into VRAM.
        if data & 0x6000 != 0 {
            dst |= 0xF4_0000;
        }
        Some(SpriteDma {
            count: self.reg(0x10),
            dst,
            dst_inc: self.reg(0x16),
            src: r(0x18) << 16 | r(0x1A),
            src_inc: self.reg(0x1C),
            fill: data & 0x0100 != 0,
        })
    }

    /// The chip hands the final pointers back to its registers.
    pub fn dma_done(&mut self, src: u32, dst: u32) {
        self.regs[0x12 >> 1] = (dst >> 16) as u16;
        self.regs[0x14 >> 1] = dst as u16;
        self.regs[0x18 >> 1] = (src >> 16) as u16;
        self.regs[0x1A >> 1] = src as u16;
    }

    pub fn read_vram(&self, addr: u32) -> u16 {
        let a = addr as usize & 0x1_FFFE;
        u16::from_be_bytes([self.vram[a], self.vram[a + 1]])
    }

    pub fn write_vram(&mut self, addr: u32, data: u16) {
        let a = addr as usize & 0x1_FFFE;
        if crate::debug::on("watch") && (0xE000..0xE004).contains(&a) {
            eprintln!("VRAM W {a:05X} = {data:04X} line {}", self.vpos);
        }
        self.vram[a..a + 2].copy_from_slice(&data.to_be_bytes());
    }

    /// Vertical retrace: publish the frame's mask history.
    pub fn end_frame(&mut self) {
        self.sprite_masked_frame = std::mem::take(&mut self.sprite_masked_acc);
    }

    pub fn vram_word(&self, word_addr: usize) -> u16 {
        let a = (word_addr & 0xFFFF) * 2;
        u16::from_be_bytes([self.vram[a], self.vram[a + 1]])
    }

    /// Renders visible line `y` into the frame buffer.
    pub fn render_line(&mut self, y: usize) {
        let width = if self.reg(0x08) & 0x0100 != 0 { 320 } else { 256 };
        if y == 0 {
            let hi = (self.reg(0x184) as u32 & 0xFF) << 16;
            self.roz_frame_x = (((hi | self.reg(0x186) as u32) << 8) as i32) >> 8;
        }
        let line = crate::render::line(self, y, width);
        self.sprite_masked_acc |= line.mask_drawn;
        let out = line.index;
        // $F00008 bit 9 crops the picture to a centred 224-line aperture
        // without changing the timing; the cropped lines are blanked.
        let cropped = self.reg(0x08) & 0x0200 != 0 && !(OVERSCAN_CROP..HEIGHT - OVERSCAN_CROP).contains(&y);
        let row = &mut self.frame[y * WIDTH..(y + 1) * WIDTH];
        if cropped {
            row.fill(0);
            return;
        }
        // H256 dots are wider: stretch the line across the frame.
        for (x, dst) in row.iter_mut().enumerate() {
            *dst = xbgr555(self.regs[256 + out[x * width / WIDTH] as usize]);
        }
    }
}

impl Default for Video {
    fn default() -> Self {
        Self::new()
    }
}

/// xBGR555 to 0RGB8888.
fn xbgr555(c: u16) -> u32 {
    let ch = |v: u16| {
        let v = (v & 0x1F) as u32;
        v << 3 | v >> 2
    };
    ch(c) << 16 | ch(c >> 5) << 8 | ch(c >> 10)
}
