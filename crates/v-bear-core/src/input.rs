//! Controller state.
//!
//! The pad is a 16-bit serial shift register (SNES-like, active low, MSB
//! first), read by the sound 6502. See docs/hardware.md §6.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Button {
    A = 15,
    B = 14,
    Start = 13,
    Select = 12,
    Up = 11,
    Down = 10,
    Left = 9,
    Right = 8,
    X = 7,
    Y = 6,
    L = 5,
    R = 4,
}

impl Button {
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name.to_ascii_lowercase().as_str() {
            "a" => Button::A,
            "b" => Button::B,
            "x" => Button::X,
            "y" => Button::Y,
            "l" => Button::L,
            "r" => Button::R,
            "start" => Button::Start,
            "select" => Button::Select,
            "up" => Button::Up,
            "down" => Button::Down,
            "left" => Button::Left,
            "right" => Button::Right,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Pad {
    bits: u16,
}

impl Pad {
    pub fn set(&mut self, button: Button, pressed: bool) {
        let mask = 1 << button as u16;
        if pressed {
            self.bits |= mask;
        } else {
            self.bits &= !mask;
        }
    }

    /// Adds buttons held elsewhere (e.g. a gamepad), same bit layout.
    pub fn press_bits(&mut self, bits: u16) {
        self.bits |= bits;
    }

    /// Buttons held, one bit per button (set = pressed).
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
