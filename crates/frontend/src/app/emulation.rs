//! The emulation loop: frame pacing, input, rewind, fast forward, sound and the texture
//! of the current frame.

#[allow(unused_imports)]
use super::*;

impl App {
    /// Smooth sound: 1-pole low-pass (~4 kHz) + DC blocker.
    pub(super) fn soften(&mut self, samples: &mut [f32]) {
        let a = 1.0 - (-2.0 * std::f32::consts::PI * 4000.0 / self.audio.rate as f32).exp();
        // interleaved stereo: one filter per channel
        for (i, x) in samples.iter_mut().enumerate() {
            let c = i & 1;
            self.lp[c] += a * (*x - self.lp[c]);
            let hp = self.lp[c] - self.dc_x[c] + 0.995 * self.dc_y[c];
            self.dc_x[c] = self.lp[c];
            self.dc_y[c] = hp;
            *x = hp;
        }
    }


    pub(super) fn emulate(&mut self, ctx: &egui::Context) {
        let now = self.now();
        if std::mem::take(&mut self.dirty_frame) {
            self.upload(ctx);
        }
        let focused = ctx.input(|i| i.focused);
        // with the debugger open the focus goes to its window: don't pause for that
        self.focus_paused = self.loaded && self.pause_unfocused && !focused && !self.debugger.open && !self.cheats.open;
        // FPS/speed: frames emulated per wall-clock second
        if now - self.fps_t0 >= 1.0 {
            self.fps_value = self.fps_frames as f64 / (now - self.fps_t0);
            self.fps_frames = 0;
            self.fps_t0 = now;
        }
        if self.loaded && self.paused && std::mem::take(&mut self.step_frame) {
            // Debugger › Advance 1 frame: one frame, no input, no sound
            self.cheats.apply(self.core.as_mut());
            self.core.run_frame(&vec![0; self.sys.players]);
            self.core.take_audio(&mut Vec::new());
            self.frame_no += 1;
            self.upload(ctx);
        }
        if !self.loaded || self.paused || self.focus_paused || self.menu_open || self.dialog != Dialog::None || self.nav.quick.is_some() {
            self.next_frame = now;
            self.session_tick = now;
            return;
        }
        // play time: only while the game runs (a long gap, e.g. the window hidden, counts little)
        if let Some(s) = &mut self.session {
            s.2 += (now - self.session_tick).clamp(0.0, 0.25);
        }
        self.session_tick = now;
        let devs = self.pads.snapshot();
        let hold = |f: Func| focused && (self.kb_level(f) || self.pad_level(&devs, f));
        let (ff_held, rw_held) = (hold(Func::FastForward), hold(Func::Rewind));
        // 1st controller = player 1, 2nd = player 2…; "P2" actions from any
        // controller go to player 2
        let players = self.sys.players;
        let mut input = vec![0u16; players];
        if focused {
            input = self.kb_bits();
            for (n, d) in devs.iter().enumerate() {
                for (p, b) in gamepad::bits(&self.mapping_for(d), d, players).into_iter().enumerate() {
                    let to = if p == 0 { n } else { p };
                    if let Some(v) = input.get_mut(to) {
                        *v |= b;
                    }
                }
            }
        }
        self.filter_input(&mut input);

        // rewind: one saved state per displayed frame (since they are saved every
        // 2 frames, it goes back at ~2x); the frame is re-run from the state
        if self.rewind_enabled && rw_held {
            self.speed = Speed::Rewind;
            if now >= self.next_frame {
                if let Some(st) = self.history.pop_back() {
                    let _ = self.core.load_state(&st);
                    self.cheats.apply(self.core.as_mut());
                    self.core.run_frame(&input);
                    self.fps_frames += 1;
                    self.core.take_audio(&mut Vec::new());
                    self.upload(ctx);
                }
                self.next_frame = now + self.frame_seconds();
            }
            return;
        }

        let fast = self.ff_toggle || ff_held;
        self.speed = if fast { Speed::Fast } else { Speed::Normal };
        let mult = if fast { self.ff_speed } else { 1 };
        let mut ran = 0;
        // Frames emulated in this tick and not shown by themselves (see `upload_blended`).
        let mut skipped: Vec<Vec<u32>> = Vec::new();
        while now >= self.next_frame && ran < 3 * mult {
            if ran > 0 && mult == 1 {
                skipped.push(packed(&self.core.video()));
            }
            if self.rewind_enabled && self.frame_no.is_multiple_of(REWIND_EVERY) {
                if self.history.len() >= REWIND_MAX {
                    self.history.pop_front();
                }
                self.history.push_back(self.core.save_state());
            }
            self.cheats.apply(self.core.as_mut());
            self.core.run_frame(&input);
            self.cheevos.frame(&*self.core);
            self.frame_no += 1;
            self.fps_frames += 1;
            let mut samples = Vec::new();
            self.core.take_audio(&mut samples);
            if self.soft_audio {
                self.soften(&mut samples);
            }
            if let Some(r) = &mut self.recorder {
                r.frame(&packed(&self.core.video()));
                r.audio(&samples); // records sound even with sound off
            }
            if !self.muted && !fast {
                self.audio.push(&samples, self.volume as f32 / 100.0); // muted while fast forwarding
            }
            self.next_frame += self.frame_seconds() / mult as f64;
            ran += 1;
        }
        if now > self.next_frame + 0.25 {
            self.next_frame = now; // too far behind: don't try to catch up
        }
        if ran > 0 {
            self.upload_blended(ctx, &skipped);
        }
    }

    pub(super) fn upload(&mut self, ctx: &egui::Context) {
        self.upload_blended(ctx, &[]);
    }

    /// Shows the current frame. When the window could not keep up and several frames were
    /// emulated before one could be shown, `skipped` has the earlier ones, and the picture is
    /// their average with the current one: games that flicker a sprite every other frame (Journey
    /// to the Laugh when the hero is hit) would otherwise show only the frames where it is hidden,
    /// and the sprite would vanish; averaged, it shows half-transparent, as on a CRT.
    pub(super) fn upload_blended(&mut self, ctx: &egui::Context, skipped: &[Vec<u32>]) {
        let v = self.core.video();
        let rgb = rgb_blended(&v, skipped);
        let img = egui::ColorImage::from_rgb([v.width, v.height], &rgb);
        let opts = if self.filter == Filter::Crt { egui::TextureOptions::LINEAR } else { egui::TextureOptions::NEAREST };
        match &mut self.tex {
            Some(t) => t.set(img, opts),
            None => self.tex = Some(ctx.load_texture("screen", img, opts)),
        }
    }
}

/// RGB bytes of `v`, averaged with the earlier frames in `skipped` (packed, `width` per row).
fn rgb_blended(v: &emu_api::Frame, skipped: &[Vec<u32>]) -> Vec<u8> {
    let mut rgb = Vec::with_capacity(v.width * v.height * 3);
    let n = skipped.len() as u32 + 1;
    for y in 0..v.height {
        for (x, &p) in v.pixels[y * v.pitch..y * v.pitch + v.width].iter().enumerate() {
            let (mut r, mut g, mut b) = ((p >> 16) & 0xFF, (p >> 8) & 0xFF, p & 0xFF);
            for f in skipped {
                let q = f.get(y * v.width + x).copied().unwrap_or(p);
                r += (q >> 16) & 0xFF;
                g += (q >> 8) & 0xFF;
                b += q & 0xFF;
            }
            rgb.extend_from_slice(&[(r / n) as u8, (g / n) as u8, (b / n) as u8]);
        }
    }
    rgb
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(pixels: &[u32], w: usize, h: usize, pitch: usize) -> emu_api::Frame<'_> {
        emu_api::Frame { pixels, width: w, height: h, pitch, aspect: 1.0 }
    }

    #[test]
    fn one_frame_is_shown_as_is() {
        let px = [0x0010_2030; 3];
        let f = frame(&px, 2, 1, 3);
        assert_eq!(rgb_blended(&f, &[]), vec![0x10, 0x20, 0x30, 0x10, 0x20, 0x30]);
    }

    #[test]
    fn a_sprite_that_flickers_every_other_frame_does_not_vanish() {
        // Frame A has the sprite (white), B hides it (black): showing only B loses it.
        let px = [0x0000_0000];
        let shown = frame(&px, 1, 1, 1);
        let skipped = vec![vec![0x00FF_FFFF]];
        assert_eq!(rgb_blended(&shown, &skipped), vec![127, 127, 127]);
    }
}
