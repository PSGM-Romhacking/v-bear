//! V-Bear: the Funtech Super A'Can, behind the `emu-api` contract. It depends on no
//! frontend. The machine itself (68000, 65C02, UM6618 video, UM6619 sound and I/O)
//! is in the other modules; `core` adapts it to the contract.

pub mod audio;
pub mod bios;
pub mod bus;
pub mod cart;
pub mod console;
pub mod core;
pub mod debug;
mod games;
pub mod input;
mod render;
pub mod sound;
mod state;
pub mod video;

pub use core::{INFO, VBearCore};
