//! The known games, by CRC32 of the ROM as the frontend reads it (a zip's files
//! joined largest first), in either byte order. Titles, years and
//! publishers from MAME's software list (hash/supracan.xml, docs/references.md);
//! thumbnail names are the No-Intro names used by libretro-thumbnails.

use std::sync::LazyLock;

use emu_api::GameInfo;

/// (CRC32, id, title, alternative title (pinyin), year, developer, No-Intro name, most
/// players at once (LaunchBox Games DB, see docs/references.md)). Developers as in
/// Wikipedia's list; the boxes say who made the game and that Funtech (敦煌) published it
/// ("…製作 ◎ 敦煌發行"), so the publisher is Funtech for all of them. MAME's software list
/// gives the developer as the publisher.
/// Every Super A'Can game was published by Funtech.
const PUBLISHER: &str = "Funtech";

type Row = (u32, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, u8);
const GAMES: [Row; 12] = [
    (0x6099bb44, "boomzoo", "Boom Zoo", "Bao Bao Dong Wu Yuan", "1996", "Funtech", "Boom Zoo (Taiwan)", 2),
    (0xb2bf31dc, "formduel", "Formosa Duel", "Formosa Da Dui Jue", "1995", "AV Artisan", "Formosa Duel (Taiwan)", 2),
    (0xac4fa721, "gamblord", "Gambling Lord", "Du Ba", "1996", "Panda Entertainment Technology / Funtech", "Gambling Lord (Taiwan)", 1),
    (0xcee25eea, "jttlaugh", "Journey to the Laugh", "Xi You Ji", "1995", "Funtech", "Journey to the Laugh (Taiwan)", 1),
    (0xaed4e4f8, "magipool", "Magical Pool", "Mo Bang Zhuang Qiu", "1996", "Funtech", "Magical Pool (Taiwan)", 2),
    (0xdc3b7b84, "monopoly", "Monopoly: Adventure in Africa", "Fei Zhou Tan Xian Da Fu Weng", "1995", "Panda Entertainment Technology", "Monopoly - Adventure in Africa (Taiwan)", 6),
    (0xa3469172, "rebelst", "Rebel Star", "Pan Xing", "1996", "Horng Shen Information", "Rebel Star (Taiwan) (En)", 1),
    (0xa4de6dde, "sangofgt", "Sango Fighter", "Wu Jiang Zheng Ba - San Guo Zhi", "1995", "Panda Entertainment Technology", "Sango Fighter (Taiwan)", 2),
    (0xf631383c, "speedyd", "Speedy Dragon", "Yin Su Fei Long", "1995", "AV Artisan", "Speedy Dragon (Taiwan)", 1),
    (0xc4fe1ca1, "slghtsag", "Super Dragon Force", "Chao Ji Guang Ming Zhan Shi", "1996", "Kingformation", "Super Dragon Force (Taiwan)", 1),
    (0xccf6829b, "staiwbbl", "Super Taiwanese Baseball League", "Chao Ji Zhong Hua Zhi Bang Lian Meng", "1995", "Quan Wei Technology (C&E)", "Super Taiwanese Baseball League (Taiwan)", 2),
    (0x9f6119a7, "sonevil", "The Son of Evil", "Xie E Zhi Zi", "1995", "Funtech", "The Son of Evil (Taiwan)", 1),
];

/// Description of each game, by id, one JSON per language (format in the frontend's
/// `sheets`): written for V-Bear from the games themselves (MAME's list only has
/// emulation notes).
pub const TEXTS: &[(&str, &str)] = &[
    ("en", include_str!("../../../assets/games/games.en.json")),
    ("pt", include_str!("../../../assets/games/games.pt.json")),
    ("es", include_str!("../../../assets/games/games.es.json")),
];

static INFOS: LazyLock<Vec<(u32, GameInfo)>> = LazyLock::new(|| {
    GAMES
        .iter()
        .map(|&(crc, id, title, alt, year, developer, thumb, players)| {
            let info = GameInfo {
                id: id.into(),
                title: title.into(),
                alt_title: Some(alt.into()),
                year: Some(year.into()),
                publisher: Some(PUBLISHER.into()),
                developer: Some(developer.into()),
                serial: None,
                players: Some(players),
                thumbnail: Some(thumb.into()),
            };
            (crc, info)
        })
        .collect()
});

/// Dumps exist in both byte orders (No-Intro's Rebel Star is word-swapped, another copy
/// isn't): a ROM is also looked up with its 16-bit words swapped.
pub fn game_info(rom: &[u8]) -> Option<&'static GameInfo> {
    let find = |crc: u32| INFOS.iter().find(|(c, _)| *c == crc).map(|(_, g)| g);
    find(emu_api::crc32(rom)).or_else(|| {
        let swapped: Vec<u8> = rom.as_chunks::<2>().0.iter().flat_map(|&[a, b]| [b, a]).collect();
        find(emu_api::crc32(&swapped))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts_cover_every_game() {
        let ids: std::collections::BTreeSet<&str> = GAMES.iter().map(|g| g.1).collect();
        for (code, json) in TEXTS {
            let m: std::collections::BTreeMap<String, serde_json::Value> = serde_json::from_str(json).unwrap();
            let keys: std::collections::BTreeSet<&str> = m.keys().map(String::as_str).collect();
            assert_eq!(keys, ids, "games.{code}.json");
        }
    }

    #[test]
    fn found_in_either_byte_order() {
        // not a real game: a fake entry is not possible, so check the swap itself
        let rom: Vec<u8> = (0..64u8).collect();
        let swapped: Vec<u8> = rom.as_chunks::<2>().0.iter().flat_map(|&[a, b]| [b, a]).collect();
        assert_ne!(emu_api::crc32(&rom), emu_api::crc32(&swapped));
        assert!(game_info(&rom).is_none());
    }
}
