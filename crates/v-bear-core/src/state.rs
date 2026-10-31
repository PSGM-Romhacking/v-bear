//! Save state files: a small header, then the machine state in bincode.
//!
//! Header: "VBEARST" + format version (u8), CRC32 of the ROM (u32 LE). A
//! state loads only into the same game and the same format version.

use serde::Serialize;
use serde::de::DeserializeOwned;

const MAGIC: &[u8; 7] = b"VBEARST";
/// Bump when the saved structures change.
const VERSION: u8 = 1;
const HEADER: usize = MAGIC.len() + 1 + 4;

pub fn encode<T: Serialize>(rom_crc: u32, value: &T) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(1 << 20);
    out.extend_from_slice(MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&rom_crc.to_le_bytes());
    bincode::serialize_into(&mut out, value).map_err(|e| e.to_string())?;
    Ok(out)
}

pub fn decode<T: DeserializeOwned>(rom_crc: u32, data: &[u8]) -> Result<T, String> {
    if data.len() < HEADER || &data[..MAGIC.len()] != MAGIC {
        return Err("not a V-Bear save state".into());
    }
    if data[MAGIC.len()] != VERSION {
        return Err(format!("save state format {} (this version reads {VERSION})", data[MAGIC.len()]));
    }
    let crc = u32::from_le_bytes(data[MAGIC.len() + 1..HEADER].try_into().unwrap());
    if crc != rom_crc {
        return Err("save state from another game".into());
    }
    bincode::deserialize(&data[HEADER..]).map_err(|e| e.to_string())
}

/// Fixed-size byte arrays larger than serde's built-in 32.
pub mod bytes {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer, const N: usize>(a: &[u8; N], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_bytes(a)
    }

    pub fn deserialize<'de, D: Deserializer<'de>, const N: usize>(d: D) -> Result<[u8; N], D::Error> {
        let v: Vec<u8> = Deserialize::deserialize(d)?;
        v.try_into().map_err(|v: Vec<u8>| D::Error::invalid_length(v.len(), &"a fixed-size array"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, serde::Deserialize, PartialEq, Debug)]
    struct Big {
        #[serde(with = "bytes")]
        a: [u8; 256],
        n: u32,
    }

    #[test]
    fn round_trip_and_checks() {
        let v = Big { a: std::array::from_fn(|i| i as u8), n: 7 };
        let data = encode(0x1234, &v).unwrap();
        assert_eq!(decode::<Big>(0x1234, &data).unwrap(), v);
        assert!(decode::<Big>(0x9999, &data).unwrap_err().contains("another game"));
        let mut old = data.clone();
        old[MAGIC.len()] = 0;
        assert!(decode::<Big>(0x1234, &old).is_err());
        assert!(decode::<Big>(0x1234, b"junk").is_err());
    }
}
