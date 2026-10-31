//! Cartridge ROM loading.
//!
//! Accepts a raw .bin or a .zip. Dumps come in either byte order (most are
//! word-swapped), so the order is detected from the 68000 reset vectors.
//! A zip holding several files (one per mask ROM, e.g. Super Dragon Force)
//! is joined largest first: the 16 Mbit ROM sits at 0, the second at A21.

use std::io::Read;
use std::path::Path;
use std::{fmt, fs, io};

/// Two 16 Mbit mask ROMs at most.
const MAX_SIZE: usize = 4 * 1024 * 1024;

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Zip(zip::result::ZipError),
    Empty,
    TooLarge(usize),
    OddSize(usize),
    NoVectors,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "{e}"),
            LoadError::Zip(e) => write!(f, "zip: {e}"),
            LoadError::Empty => write!(f, "no ROM data"),
            LoadError::TooLarge(n) => write!(f, "too large ({n} bytes, max 4 MiB)"),
            LoadError::OddSize(n) => write!(f, "odd size ({n} bytes), expected 16-bit words"),
            LoadError::NoVectors => write!(f, "no valid 68000 reset vectors in either byte order"),
        }
    }
}

#[derive(Clone, Default)]
pub struct Cartridge {
    /// ROM as the 68000 sees it: big-endian 16-bit words.
    pub rom: Vec<u8>,
    pub name: String,
    /// The file was word-swapped and has been put in big-endian order.
    pub swapped: bool,
}

impl Cartridge {
    pub fn load(path: &Path) -> Result<Self, LoadError> {
        let is_zip = path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("zip"));
        let rom = if is_zip { read_zip(path)? } else { fs::read(path).map_err(LoadError::Io)? };
        let name = path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        Self::from_bytes(rom, name)
    }

    /// A ROM already in memory (several mask ROMs already joined, largest first).
    pub fn from_bytes(mut rom: Vec<u8>, name: String) -> Result<Self, LoadError> {
        match rom.len() {
            0 => return Err(LoadError::Empty),
            n if n > MAX_SIZE => return Err(LoadError::TooLarge(n)),
            n if n % 2 != 0 => return Err(LoadError::OddSize(n)),
            _ => {}
        }
        let swapped = if vectors_ok(&rom) {
            false
        } else {
            swap_words(&mut rom);
            if !vectors_ok(&rom) {
                return Err(LoadError::NoVectors);
            }
            true
        };
        Ok(Self { rom, name, swapped })
    }

    pub fn read_u32(&self, addr: usize) -> u32 {
        read_u32(&self.rom, addr)
    }

    pub fn summary(&self) -> String {
        format!(
            "{}: {} KiB{}, SSP={:08X} PC={:08X}",
            self.name,
            self.rom.len() / 1024,
            if self.swapped { " (word-swapped dump)" } else { "" },
            self.read_u32(0),
            self.read_u32(4)
        )
    }
}

/// All files in the zip, largest first, concatenated.
fn read_zip(path: &Path) -> Result<Vec<u8>, LoadError> {
    let file = fs::File::open(path).map_err(LoadError::Io)?;
    let mut zip = zip::ZipArchive::new(file).map_err(LoadError::Zip)?;
    let mut parts = Vec::new();
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(LoadError::Zip)?;
        if entry.is_dir() {
            continue;
        }
        let mut data = Vec::with_capacity(entry.size() as usize);
        entry.read_to_end(&mut data).map_err(LoadError::Io)?;
        parts.push(data);
    }
    parts.sort_by_key(|p| std::cmp::Reverse(p.len()));
    Ok(parts.concat())
}

fn read_u32(rom: &[u8], addr: usize) -> u32 {
    rom.get(addr..addr + 4)
        .map_or(0, |b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

/// Stack in work RAM ($FC0000-$FFFFFF, or $1000000 for a stack at the top)
/// and an even entry point inside the cartridge space.
fn vectors_ok(rom: &[u8]) -> bool {
    let ssp = read_u32(rom, 0);
    let pc = read_u32(rom, 4);
    (0xFC_0000..=0x100_0000).contains(&ssp) && pc < 0x40_0000 && pc.is_multiple_of(2)
}

fn swap_words(rom: &mut [u8]) {
    for w in rom.as_chunks_mut::<2>().0 {
        w.swap(0, 1);
    }
}
