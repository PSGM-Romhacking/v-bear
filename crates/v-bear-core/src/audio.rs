//! Sound output: the sampler's 44.7 kHz stream resampled for the host device.

use crate::sound::SAMPLE_RATE as NATIVE_RATE;

/// Output rate used when the host device does not say (most run at 48 or 44.1 kHz).
pub const DEFAULT_RATE: u32 = 48_000;
/// The sampler's sum leaves a lot of headroom (one full voice peaks at
/// 1905/32768); raise it to a normal listening level, clipping at full scale.
const GAIN: f32 = 4.0;

pub struct Audio {
    /// Stereo samples (L, R interleaved) at the sampler's own rate, one frame's worth.
    pub samples: Vec<f32>,
    /// Resampler position between the last two input samples.
    phase: f64,
    last: [f32; 2],
}

impl Audio {
    pub fn new() -> Self {
        Self { samples: Vec::new(), phase: 0.0, last: [0.0; 2] }
    }

    /// The frame's samples at `rate` Hz (linear interpolation).
    pub fn resampled(&mut self, rate: u32) -> Vec<f32> {
        let step = NATIVE_RATE / rate as f64;
        let mut out = Vec::with_capacity(self.samples.len() * 2);
        for frame in self.samples.as_chunks::<2>().0 {
            let cur = [frame[0], frame[1]];
            while self.phase < 1.0 {
                let t = self.phase as f32;
                let l = self.last[0] + (cur[0] - self.last[0]) * t;
                let r = self.last[1] + (cur[1] - self.last[1]) * t;
                out.push((l * GAIN).clamp(-1.0, 1.0));
                out.push((r * GAIN).clamp(-1.0, 1.0));
                self.phase += step;
            }
            self.phase -= 1.0;
            self.last = cur;
        }
        out
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}
