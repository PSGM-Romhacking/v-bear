//! UM6618 line renderer: four tilemaps, the ROZ plane, two windows, the
//! sprites and the mixer that composes them in palette-index space.
//!
//! Each part follows its MiSTer module (um6618_tilemap/roz/window/sprite/
//! mixer.sv); see docs/hardware.md §4.

use crate::video::Video;

const MAX_W: usize = 320;

/// One layer's output for a line, as the mixer sees it.
#[derive(Clone, Copy)]
struct Layer {
    index: [u8; MAX_W],
    opaque: [bool; MAX_W],
    tie: [bool; MAX_W],
    priority: u8,
    add: bool,
}

impl Default for Layer {
    fn default() -> Self {
        Self { index: [0; MAX_W], opaque: [false; MAX_W], tie: [false; MAX_W], priority: 7, add: false }
    }
}

/// The sprite line buffers: S (index, priority, drawn by a mask-1 sprite),
/// B (what a mask-1 sprite covered) and K (a mask-only sprite covered the dot).
struct Sprites {
    index: [u8; MAX_W],
    priority: [u8; MAX_W],
    backup_mode: [bool; MAX_W],
    backup: [u8; MAX_W],
    masked: [bool; MAX_W],
    /// A mask-only sprite drew somewhere on this line.
    any_mask: bool,
}

/// What the renderer hands back for one line.
pub struct LineOut {
    pub index: [u8; MAX_W],
    pub mask_drawn: bool,
}

const MOSAIC: [(i32, i32); 8] = [(1, 1), (2, 2), (4, 2), (4, 4), (4, 8), (8, 8), (16, 8), (16, 16)];

fn map_size(code: u16) -> (u32, u32) {
    match code & 0xF {
        0x2 => (4, 4),
        0x4 => (5, 5),
        0x6 => (6, 5),
        0xA => (7, 5),
        0xC => (6, 6),
        _ => (5, 5),
    }
}

fn sext(v: u32, bits: u32) -> i32 {
    let shift = 32 - bits;
    ((v as i32) << shift) >> shift
}

pub fn line(v: &Video, y: usize, width: usize) -> LineOut {
    let flags = v.reg(0x08);
    // Order matters to the mixer: BG0..3, ROZ, window 0, window 1.
    let mut layers = [Layer::default(); 7];
    #[allow(clippy::needless_range_loop)] // g is also the layer number
    for g in 0..4 {
        if flags & (0x80 >> g) != 0 {
            tilemap(v, g, y, width, &mut layers[g]);
        }
    }
    if flags & 0x04 != 0 {
        roz(v, y, width, &mut layers[4]);
    }
    window(v, 0, flags & 0x02 != 0, y, width, &mut layers[5]);
    window(v, 1, flags & 0x01 != 0 && v.reg(0x1D8) != 0, y, width, &mut layers[6]);

    let sprites = if flags & 0x08 != 0 { Some(sprites(v, y, width)) } else { None };
    let mut out = LineOut { index: [0; MAX_W], mask_drawn: false };
    for x in 0..width {
        out.index[x] = mix(&layers, sprites.as_ref(), v.sprite_masked_frame, x);
    }
    out.mask_drawn = sprites.is_some_and(|s| s.any_mask);
    out
}

// ---- tilemaps ----------------------------------------------------------------

fn tilemap(v: &Video, g: usize, y: usize, width: usize, out: &mut Layer) {
    let b = 0x100 + 0x20 * g as u32;
    let flags = v.reg(b);
    let tile_mode = v.reg(b + 2);
    let map_base = v.reg(b + 8) as usize;
    let mode = v.reg(b + 0xA);

    // Depth: $F001F0 bits 2-0 for layers 0 and 1; 2 and 3 are always 2bpp.
    // region 0 = 8bpp, 1 = 4bpp, 2 = 2bpp.
    let region: usize = match (g, v.reg(0x1F0) & 7) {
        (0 | 1, 0) => 2,
        (0 | 1, 1 | 3) => 1,
        (0, 2) => 0,
        (1, 2) => 1,
        (0, _) => 0,
        _ => 2,
    };
    let (xs_log, ys_log) = map_size(flags >> 8);
    let x_mask: i32 = (1 << (xs_log + 3)) - 1;
    let y_mask: i32 = (1 << (ys_log + 3)) - 1;
    // X scroll 11 bits signed, Y 12; a layer flip XORs the scroll.
    let mut sx = sext(v.reg(b + 4) as u32, 11);
    let mut sy = sext(v.reg(b + 6) as u32, 12);
    if flags & 2 != 0 {
        sx ^= x_mask;
    }
    if flags & 1 != 0 {
        sy ^= y_mask;
    }
    let (mos_x, mos_y) = MOSAIC[(flags as usize >> 2) & 7];
    let wrap = flags & 0x20 != 0;
    let pal_hi = tile_mode & 0x0200 != 0;

    let y_check = y as i32 + sy;
    if !wrap && (y_check < 0 || y_check > y_mask) {
        return;
    }
    let mut real_y = ((y as i32 & !(mos_y - 1)) + sy) & y_mask;
    if tile_mode & 0x0800 != 0 {
        let sel = v.vram_word(((v.reg(b + 0xE) as usize & 0x7FFF) << 1) + y) as i16 as i32;
        real_y = (sel + sy) & y_mask;
    }
    let mut line_scroll = sx;
    if tile_mode & 0x4000 != 0 {
        let off = if tile_mode & 0x0100 != 0 { y } else { 0 };
        line_scroll += v.vram_word(((v.reg(b + 0xC) as usize & 0x7FFF) << 1) + off) as i16 as i32;
    }

    out.priority = (flags >> 13) as u8;
    out.add = flags & 0x40 != 0;
    let bank = (((mode >> 12) & 7) as usize) << (8 + region);
    let mut cached = usize::MAX;
    let mut row = [0u8; 8];
    let (mut flip_x, mut pal, mut tie) = (0, 0u8, false);

    for px in 0..width {
        let pos = px as i32 + line_scroll;
        if !wrap && (pos < 0 || pos > x_mask) {
            continue;
        }
        let real_x = ((px as i32 & !(mos_x - 1)) + line_scroll) & x_mask;
        let cell = ((real_y as usize >> 3) << xs_log) | (real_x as usize >> 3);
        if cell != cached {
            cached = cell;
            let entry = v.vram_word(((map_base & 0x7FFF) << 1) + cell);
            let tile_no = (entry as usize & 0x3FF) + bank;
            let py = (real_y as usize & 7) ^ if entry & 0x0400 != 0 { 7 } else { 0 };
            let row_byte = (tile_no << (6 - region)) + (py << (3 - region));
            for (i, b) in row.iter_mut().take(8 >> region).enumerate() {
                *b = v.vram[(row_byte + i) & 0x1_FFFF];
            }
            flip_x = if entry & 0x0800 != 0 { 7 } else { 0 };
            pal = (entry >> 12) as u8 | if pal_hi { 8 } else { 0 };
            tie = pal_hi && entry & 0x8000 == 0;
        }
        let s = (real_x as usize & 7) ^ flip_x;
        let pix = match region {
            0 => row[s],
            1 => (row[s >> 1] >> if s & 1 == 0 { 4 } else { 0 }) & 0xF,
            _ => (row[s >> 2] >> ((3 - (s & 3)) * 2)) & 3,
        };
        if pix != 0 {
            out.index[px] = if region == 0 { pix } else { pal << 4 | pix };
            out.opaque[px] = true;
            out.tie[px] = tie;
        }
    }
}

// ---- ROZ ---------------------------------------------------------------------

fn roz(v: &Video, y: usize, width: usize, out: &mut Layer) {
    let r = |o: u32| v.reg(0x180 + o);
    let mode = r(0x0);
    let tile_mode = r(0x2);
    let (xs_log, ys_log) = map_size(mode >> 8);
    let x_limit = 1i32 << (xs_log + 3);
    let y_limit = 1i32 << (ys_log + 3);
    // region: 0 = 8bpp, 1 = 4bpp, 2 = 2bpp, 4 = the BIOS logo's 1bpp layout.
    let region = match mode & 3 {
        0 => 4,
        1 => 2,
        2 => 1,
        _ => 0,
    };
    let bitmap = region == 0 && (v.reg(0x1F0) >> 3) & 3 == 1;
    let need_map = region != 4 && !bitmap;
    let (mos_x, mos_y) = MOSAIC[(mode as usize >> 2) & 7];
    let wrap = mode & 0x20 != 0;
    let use_lines = tile_mode & 0x4000 != 0;
    // Bit 15 alone turns on the per-line source row (Super Dragon Force's menus set 0x8500, with
    // bit 14 clear, and rely on it to hide the frames they do not use); bit 14 adds the X tables.
    let use_y_lines = tile_mode & 0x8000 != 0;
    let x_per_line = tile_mode & 0x0100 != 0 || bitmap || mode & 3 == 0;

    out.priority = (mode >> 13) as u8;
    out.add = mode & 0x40 != 0;

    // Origins: 24 signed bits with 8 fractional; coefficients 8.8.
    let scroll_x_reg = sext(((r(0x4) as u32 & 0xFF) << 16) | r(0x6) as u32, 24);
    let scroll_y_reg = sext(((r(0x8) as u32 & 0xFF) << 16) | r(0xA) as u32, 24);
    let coef_b = r(0xE) as i16 as i32;
    let coef_c = r(0x10) as i16 as i32;
    let coef_d = r(0x12) as i16 as i32;
    let table = |reg: u32, off: usize| v.vram_word(((r(reg) as usize & 0x7FFF) << 1) + off);

    // y·B and y·D at the mosaic block origin line.
    let blk_y = y as i32 & !(mos_y - 1);
    let yblk_b = blk_y * coef_b;
    let yblk_d = blk_y * coef_d;

    let mut line_a = r(0xC) as i16 as i32;
    let mut scroll_x = if use_lines { v.roz_frame_x } else { scroll_x_reg };
    let mut scroll_y = scroll_y_reg;
    if use_y_lines && !use_lines {
        let hi = table(0x1E, y * 2) as u32;
        let lo = table(0x1E, y * 2 + 1) as u32;
        scroll_y = scroll_y.wrapping_add((hi << 16 | lo) as i32);
    }
    if use_lines {
        if tile_mode & 0x0040 != 0 {
            // A zero in the first table turns the line off.
            let t0 = table(0x18, y);
            if t0 == 0 {
                return;
            }
            line_a = t0 as i16 as i32;
        }
        if x_per_line {
            let hi = table(0x1A, y * 2) as u32;
            let lo = table(0x1A, y * 2 + 1) as u32;
            scroll_x = scroll_x.wrapping_add((hi << 16 | lo) as i32);
        } else {
            scroll_x += (table(0x1A, 0) as i16 as i32) << 8;
        }
        if use_y_lines {
            let hi = table(0x1E, y * 2) as u32;
            let lo = table(0x1E, y * 2 + 1) as u32;
            scroll_y = scroll_y.wrapping_add((hi << 16 | lo) as i32);
        }
    }
    let row_y = if use_y_lines { scroll_y } else { scroll_y + yblk_d };
    let (mut accx, mut accy) = (scroll_x + yblk_b, row_y);
    let (mut blkx, mut blky) = (accx, accy);

    let map_base = (r(0x14) as usize & 0x7FFF) << 1;
    let bank_reg = r(0x16) as usize;
    let tile_bank = match region {
        0 => ((bank_reg >> 12) & 0xF) << 8,
        1 => ((bank_reg >> 12) & 0xF) << 9,
        _ => ((bank_reg >> 12) & 0x7) << 10,
    };
    let pal_hi = tile_mode & 0x0200 != 0;

    for px in 0..width {
        let src_x = blkx >> 8;
        let src_y = blky >> 8;
        let outside = !wrap && (src_x < 0 || src_x >= x_limit || src_y < 0 || src_y >= y_limit);
        if !outside {
            let mx = (src_x & (x_limit - 1)) as usize;
            let my = (src_y & (y_limit - 1)) as usize;
            let entry = if need_map {
                v.vram_word(map_base + (((my >> 3) << xs_log) | (mx >> 3)))
            } else {
                0
            };
            let tile_no = (entry as usize & 0x3FF) + tile_bank;
            let tpx = (mx & 7) ^ if entry & 0x0800 != 0 { 7 } else { 0 };
            let tpy = (my & 7) ^ if entry & 0x0400 != 0 { 7 } else { 0 };
            let byte_addr = if region == 4 {
                logo_byte(mx, my)
            } else if bitmap {
                (bank_reg << 2) + (my << (xs_log + 3)) + mx
            } else {
                let sel = match region {
                    0 => tpx,
                    1 => tpx >> 1,
                    _ => tpx >> 2,
                };
                (tile_no << (6 - region)) + (tpy << (3 - region)) + sel
            };
            let byte = v.vram[byte_addr & 0x1_FFFF];
            let (raw, op) = match region {
                4 => {
                    let bit = (byte >> (7 - (mx & 7))) & 1;
                    (bit, bit != 0)
                }
                0 => (byte, byte != 0),
                1 => {
                    let p = if tpx & 1 != 0 { byte & 0xF } else { byte >> 4 };
                    (p, p != 0)
                }
                _ => {
                    let p = (byte >> ((3 - (tpx & 3)) * 2)) & 3;
                    (p, p != 0)
                }
            };
            if op {
                let pal = (entry >> 12) as u8 | if pal_hi { 8 } else { 0 };
                out.index[px] = if region == 0 || region == 4 || bitmap { raw } else { pal << 4 | raw };
                out.opaque[px] = true;
                out.tie[px] = need_map && pal_hi && entry & 0x8000 == 0;
            }
        }
        accx = accx.wrapping_add(line_a);
        accy = accy.wrapping_add(coef_c);
        if (px + 1) as i32 & (mos_x - 1) == 0 {
            blkx = accx;
            blky = accy;
        }
    }
}

/// The BIOS logo: fixed tile layout with the low address bits rotated, a map
/// thirty-two tiles wide.
fn logo_byte(mx: usize, my: usize) -> usize {
    let lcount = (((my >> 8) & 7) << 10 | ((my >> 3) & 0x1F) << 5) + (mx >> 3);
    let mut tile = 0x880 + ((lcount & 7) << 1);
    if lcount & 0x20 != 0 {
        tile ^= 1;
    }
    tile |= ((lcount >> 6) & 3) << 4;
    let raw = (tile << 3) + (my & 7);
    (raw & !0x7F) | ((raw & 0xF) << 3) | ((raw >> 4) & 7)
}

// ---- windows -----------------------------------------------------------------

fn window(v: &Video, n: u32, active: bool, y: usize, width: usize, out: &mut Layer) {
    if !active {
        return;
    }
    let b = 0x1D0 + 8 * n;
    let ctrl = v.reg(b);
    let scroll = sext(v.reg(b + 4) as u32, 10);
    // Line L is clipped by table entry L. MiSTer fetches the pair during line L-1 but its
    // `render_y` is already vpos + 1 (um6618.sv), so what it reads is entry L; MAME indexes by
    // the line itself. Using L-1 drew the shape one line too low (docs/compatibility.md,
    // 2026-10-30).
    let off = if ctrl & 0x0100 != 0 { y * 2 } else { 0 };
    let base = (v.reg(b + 2) as usize & 0x7FFF) << 1;
    let min = v.vram_word(base + off) as i16 as i32 + scroll;
    let max = v.vram_word(base + off + 1) as i16 as i32 + scroll;
    let invert = ctrl & 0x0800 != 0;
    out.priority = ((ctrl >> 13) & 3) as u8;
    for x in 0..width {
        let inside = ((x as i32) >= min && (x as i32) < max) ^ invert;
        if inside {
            out.index[x] = ctrl as u8;
            out.opaque[x] = true;
            out.tie[x] = true;
        }
    }
}

// ---- sprites -----------------------------------------------------------------

const SPRITE_MAX: usize = 128;
const YSIZE: [u32; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 16, 20, 22, 24, 26];

fn sprites(v: &Video, y: usize, width: usize) -> Sprites {
    let mut s = Sprites {
        index: [0; MAX_W],
        priority: [0; MAX_W],
        backup_mode: [false; MAX_W],
        backup: [0; MAX_W],
        masked: [false; MAX_W],
        any_mask: false,
    };
    let base = (v.reg(0x20) as usize & 0x7FFF) << 1;
    let count = ((v.reg(0x22) as usize & 0x7FFF) + 1).min(SPRITE_MAX);
    // $F00026 bit 0: 8bpp (set) or 4bpp.
    let four_bpp = v.reg(0x26) & 1 == 0;

    for i in 0..count {
        let e = base + i * 4;
        let w = [v.vram_word(e), v.vram_word(e + 1), v.vram_word(e + 2), v.vram_word(e + 3)];
        if w[3] == 0 {
            continue;
        }
        let ysize = YSIZE[(w[0] as usize >> 9) & 0xF];
        let xsize = 1u32 << (w[1] & 7);
        let is1x1 = xsize == 1 && ysize == 1;
        let direct = w[3] & 0x8000 != 0 || is1x1;
        let (nw, nh, xslog) = if direct { (8, 8, 0) } else { (xsize * 8, ysize * 8, w[1] as u32 & 7) };
        // Scaled size: width = (hscale + 6·nw) / (hscale + 1);
        // height = (vscale + 2·nh − 1) / vscale, or 3·nh with vscale 0.
        let hscale = (w[2] >> 11) as u32 & 0x1F;
        let vscale = (w[0] >> 13) as u32 & 7;
        let cap = |q: u32| q.clamp(1, 256);
        let width_s = cap((hscale + 6 * nw) / (hscale + 1));
        let height_s = (vscale + 2 * nh - 1).checked_div(vscale).map_or((nh * 3).min(256), cap);

        let pos = |p: u16| {
            let p = (p & 0x1FF) as i32;
            if p >= 0x180 { p - 512 } else { p }
        };
        let sx = pos(w[2]);
        let sy = pos(w[0]);
        let row = y as i32 - sy;
        if row == 0 && (width_s != nw || height_s != nh) && crate::debug::on("sprscale") {
            eprintln!("SPRSCALE #{i} {:04X} {:04X} {:04X} {:04X}: {nw}x{nh} drawn {width_s}x{height_s} at ({sx},{sy})", w[0], w[1], w[2], w[3]);
        }
        if row < 0 || row >= height_s as i32 {
            continue;
        }
        let (mos_x, mos_y) = MOSAIC[(w[1] as usize >> 3) & 7];
        // Source row: the Bresenham accumulator at the mosaic block origin.
        let blk_row = (row - row % mos_y) as u32;
        let src_y = blk_row * nh / height_s;
        let flip_x = w[1] & 0x0800 != 0;
        let flip_y = w[1] & 0x0400 != 0;
        let use_y = if flip_y { nh - 1 - src_y } else { src_y } as usize;
        let bank = (w[1] >> 12) as usize & 0xF;
        let mask_mode = (w[1] >> 8) & 3;
        let prio = ((w[2] >> 9) & 3) as u8;

        for d in 0..width_s {
            // The horizontal position wraps at 512 (MAME: "magipool rolls back at 512 edge for
            // the shot power menu"): a sprite that starts below -128 comes back in from the right
            // side of the 9-bit space and its far end lands on the left of the screen.
            let screen_x = (sx + d as i32) & 0x1FF;
            if screen_x >= width as i32 {
                continue;
            }
            let blk_d = d - d % mos_x as u32;
            let sxb = (blk_d * nw / width_s) as usize;
            let src_x = if flip_x { nw as usize - 1 - sxb } else { sxb };
            let entry = if direct {
                w[3]
            } else {
                let ent_addr = ((w[3] as usize & 0x7FFF) << 1) + ((use_y >> 3) << xslog) + (src_x >> 3);
                v.vram_word(ent_addr)
            };
            if entry == 0 && (four_bpp || bank == 0) {
                continue;
            }
            let tile_no = (entry as usize & 0x3FF) + (bank << if four_bpp { 9 } else { 8 });
            let tpx = (src_x & 7) ^ if entry & 0x0800 != 0 { 7 } else { 0 };
            let tpy = (use_y & 7) ^ if entry & 0x0400 != 0 { 7 } else { 0 };
            let (idx, on) = if four_bpp {
                let byte = v.vram[((tile_no << 5) + (tpy << 2) + (tpx >> 1)) & 0x1_FFFF];
                let p = if tpx & 1 != 0 { byte & 0xF } else { byte >> 4 };
                (((entry >> 12) as u8) << 4 | p, p != 0)
            } else {
                let byte = v.vram[((tile_no << 6) + (tpy << 3) + tpx) & 0x1_FFFF];
                (byte, byte != 0)
            };
            if !on {
                continue;
            }
            let x = screen_x as usize;
            match mask_mode {
                2 | 3 => {
                    // Mode 3 moves the covered sprite ahead without recolouring it.
                    if mask_mode == 3 {
                        s.priority[x] = prio;
                    }
                    s.masked[x] = true;
                    s.any_mask = true;
                }
                _ => {
                    if s.index[x] == 0 || prio <= s.priority[x] {
                        if mask_mode == 1 {
                            s.backup[x] = s.index[x];
                        }
                        s.index[x] = idx;
                        s.priority[x] = prio;
                        s.backup_mode[x] = mask_mode == 1;
                    }
                }
            }
        }
    }
    s
}

// ---- mixer -------------------------------------------------------------------

/// Priority and colour composition in palette-index space. Lower priority is
/// in front. At equal priority a later layer is nearer, but an ordinary layer
/// replaces an earlier one only when it raises the tie bit. Additive layers
/// sum their indices with whatever lies behind them.
fn mix(layers: &[Layer; 7], spr: Option<&Sprites>, frame_masked: bool, x: usize) -> u8 {
    const N: usize = 7;
    let valid = |i: usize| layers[i].opaque[x];
    let prio = |i: usize| layers[i].priority;
    let tie = |i: usize| layers[i].tie[x];
    // Is j nearer than i?
    let nearer = |i: usize, j: usize| prio(j) < prio(i) || (j > i && prio(j) == prio(i));

    let mut eligible = [false; N];
    #[allow(clippy::needless_range_loop)] // i is the layer number, compared with j
    for i in 0..N {
        let blocked = (0..i).any(|j| {
            valid(j)
                && prio(j) == prio(i)
                && if i < 4 { tie(j) || !tie(i) } else { i > 4 && j > 4 }
        });
        eligible[i] = valid(i) && !blocked;
    }

    let (s_index, s_prio) = spr.map_or((0, 0), |s| (s.index[x], s.priority[x]));
    let sprite_valid = s_index != 0;
    let ahead = |i: usize| eligible[i] && prio(i) < s_prio;
    let behind = |i: usize| eligible[i] && !ahead(i);
    let first_ahead = |i: usize| ahead(i) && !(0..N).any(|j| nearer(j, i) && ahead(j));
    let covered = |i: usize| {
        behind(i) && !(0..N).any(|j| nearer(i, j) && behind(j)) && prio(i) == s_prio && tie(i)
    };
    let sprite_base = sprite_valid && !(0..N).any(covered);
    let inject: [bool; N] = std::array::from_fn(|i| first_ahead(i) && layers[i].add && sprite_base);
    let barrier = |j: usize| (eligible[j] && !layers[j].add) || inject[j];

    let mut bg = 0u8;
    let mut injected = false;
    let (mut bg_prio, mut bg_tie) = (15u8, false);
    for i in 0..N {
        if !eligible[i] {
            continue;
        }
        if !(0..N).any(|j| nearer(i, j) && barrier(j)) {
            bg = bg.wrapping_add(layers[i].index[x]);
            injected |= inject[i];
        }
        if !(0..N).any(|j| nearer(i, j) && eligible[j]) {
            bg_prio = prio(i);
            bg_tie = tie(i);
        }
    }
    if injected {
        bg = bg.wrapping_add(s_index);
    }

    let Some(s) = spr else { return bg };
    let hit = sprite_valid && (s_prio < bg_prio || (s_prio == bg_prio && !bg_tie));
    if !hit {
        bg
    } else if !s.backup_mode[x] {
        s_index
    } else if frame_masked {
        if s.masked[x] { s_index } else { bg }
    } else {
        // A blend sprite keeps its palette row and adds only the pens, no carry.
        let under = if s.backup[x] != 0 { s.backup[x] } else { bg } & 0xF;
        (s_index & 0xF0) | (s_index.wrapping_add(under) & 0xF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put(v: &mut Video, word: usize, val: u16) {
        v.vram[word * 2..word * 2 + 2].copy_from_slice(&val.to_be_bytes());
    }

    /// One sprite, 32 tiles wide (256 pixels), at the 9-bit position `x`; every tile is tile 1,
    /// a solid 4bpp tile.
    fn wide_sprite(x: u16) -> Video {
        let mut v = Video::new();
        v.regs[0x20 >> 1] = 0x0100; // sprite table at word 0x200
        v.regs[0x22 >> 1] = 0; // one sprite
        put(&mut v, 0x200, 0); // y = 0, height 1 tile
        put(&mut v, 0x201, 5); // 1 << 5 = 32 tiles wide
        put(&mut v, 0x202, x);
        put(&mut v, 0x203, 0x1000); // tile map at word 0x2000
        for t in 0..32 {
            put(&mut v, 0x2000 + t, 1);
        }
        v.vram[32..64].fill(0x11); // tile 1: all pixels use pen 1
        v
    }

    #[test]
    fn sprite_position_wraps_at_512() {
        // 0x17F is 383, not negative: the sprite starts at 383 and 129 of its pixels land past
        // 511, which come back in at the left of the screen (Magical Pool's shot-power cue).
        let s = sprites(&wide_sprite(0x17F), 0, 320);
        assert!(s.index[..127].iter().all(|&i| i != 0), "the end of the sprite shows at the left");
        assert_eq!(s.index[127], 0);
        assert!(s.index[127..].iter().all(|&i| i == 0));
    }

    #[test]
    fn sprite_negative_position_still_clips() {
        // 0x1C0 is -64: its first 64 pixels are off the left edge, the rest is drawn from 0.
        let s = sprites(&wide_sprite(0x1C0), 0, 320);
        assert!(s.index[..192].iter().all(|&i| i != 0));
        assert!(s.index[192..].iter().all(|&i| i == 0));
    }
}
