//! The console's internal ROMs (not distributed; supplied by the user).
//!
//! Accepted: MAME's `supracan.zip`, a folder with the four files, or the
//! MiSTer `boot.rom` concatenation. Files are recognised by CRC32, so names
//! do not matter.

use std::io::Read;
use std::path::Path;
use std::{fs, io};

pub const IPL_CRC: u32 = 0x8d57_5662;
pub const SND1_CRC: u32 = 0xfc9f_b05f;
pub const SND2_CRC: u32 = 0xbf95_0ab7;
pub const KEY_CRC: u32 = 0x0ba7_8597;

#[derive(Clone)]
pub struct Bios {
    /// 68000 IPL, 4 KiB, big-endian words.
    pub ipl: Vec<u8>,
    /// 65C02 boot image, 16 KiB, copied to sound RAM $0000 at reset.
    pub sound: Vec<u8>,
    /// UM6650 lockout key, 16 bytes.
    pub key: Vec<u8>,
}

impl Bios {
    pub fn load(path: &Path) -> Result<Self, String> {
        let files = if path.is_dir() {
            read_dir(path)
        } else if path.extension().is_some_and(|e| e.eq_ignore_ascii_case("zip")) {
            read_zip(path)
        } else {
            fs::read(path).map(|d| split_boot_rom(&d)).map_err(|e| e.to_string())?
        };
        Self::from_files(&files.map_err(|e| e.to_string())?)
    }

    /// Picks the four files out of `files` by CRC32 (other files are ignored).
    pub fn from_files(files: &[Vec<u8>]) -> Result<Self, String> {
        let find = |crc: u32, name: &str| {
            files
                .iter()
                .find(|d| crc32(d) == crc)
                .cloned()
                .ok_or(format!("missing {name} (CRC32 {crc:08x})"))
        };
        let mut ipl = find(IPL_CRC, "internal_68k.bin")?;
        let mut sound = find(SND1_CRC, "internal_6502_1.bin")?;
        sound.extend(find(SND2_CRC, "internal_6502_2.bin")?);
        let key = find(KEY_CRC, "umc6650.bin")?;
        // The IPL dump is word-swapped; its reset vectors tell which order is right.
        if !ipl_vectors_ok(&ipl) {
            ipl.as_chunks_mut::<2>().0.iter_mut().for_each(|w| w.swap(0, 1));
        }
        Ok(Self { ipl, sound, key })
    }
}

/// Stack in work RAM, entry point inside the IPL or the cartridge.
fn ipl_vectors_ok(ipl: &[u8]) -> bool {
    let ssp = u32::from_be_bytes([ipl[0], ipl[1], ipl[2], ipl[3]]);
    (0xFC_0000..=0x100_0000).contains(&ssp)
}

fn read_dir(path: &Path) -> io::Result<Vec<Vec<u8>>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(path)? {
        let p = entry?.path();
        if p.is_file() && fs::metadata(&p)?.len() <= 0x6000 {
            out.push(fs::read(&p)?);
        }
    }
    Ok(out)
}

fn read_zip(path: &Path) -> io::Result<Vec<Vec<u8>>> {
    let mut zip = zip::ZipArchive::new(fs::File::open(path)?)?;
    let mut out = Vec::new();
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let mut data = Vec::new();
        entry.read_to_end(&mut data)?;
        out.push(data);
    }
    Ok(out)
}

/// MiSTer boot.rom: IPL (4 KiB) + two 8 KiB sound images + 16-byte key.
fn split_boot_rom(d: &[u8]) -> io::Result<Vec<Vec<u8>>> {
    if d.len() != 0x5010 {
        return Err(io::Error::other("not a BIOS image (expected 20496 bytes)"));
    }
    Ok(vec![
        d[..0x1000].to_vec(),
        d[0x1000..0x3000].to_vec(),
        d[0x3000..0x5000].to_vec(),
        d[0x5000..].to_vec(),
    ])
}

pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = !0u32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xEDB8_8320 & (crc & 1).wrapping_neg());
        }
    }
    !crc
}
