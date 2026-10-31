//! The core as the frontend drives it, through `emu_api::Core`, with a real game.
//! Skipped unless V_BEAR_TEST_ROM (a game .zip) and V_BEAR_TEST_BIOS (a folder with
//! the four BIOS files) are set: ROMs and BIOS are never in the repository.

use std::io::Read;

use emu_api::{Core, PadButton};

/// The game's files joined largest first, as the frontend reads a multi-file zip.
fn read_zip(path: &std::path::Path) -> Vec<u8> {
    let mut z = zip::ZipArchive::new(std::fs::File::open(path).unwrap()).unwrap();
    let mut parts: Vec<Vec<u8>> = (0..z.len())
        .map(|i| {
            let mut v = Vec::new();
            z.by_index(i).unwrap().read_to_end(&mut v).unwrap();
            v
        })
        .collect();
    parts.sort_by_key(|p| std::cmp::Reverse(p.len()));
    parts.concat()
}

#[test]
fn runs_a_game_through_the_contract() {
    let (Some(rom), Some(bios)) = (std::env::var_os("V_BEAR_TEST_ROM"), std::env::var_os("V_BEAR_TEST_BIOS")) else {
        eprintln!("V_BEAR_TEST_ROM / V_BEAR_TEST_BIOS not set: test skipped");
        return;
    };
    let info = &v_bear_core::INFO;
    let files: Vec<Vec<u8>> = std::fs::read_dir(bios).unwrap().flatten().map(|e| std::fs::read(e.path()).unwrap()).collect();
    let slots: Vec<Option<Vec<u8>>> =
        info.bios.iter().map(|b| files.iter().find(|f| emu_api::crc32(f) == b.crc32).cloned()).collect();
    assert!(slots.iter().all(Option::is_some), "BIOS files missing");

    let rom = read_zip(std::path::Path::new(&rom));
    assert!((info.game_info)(&rom).is_some(), "game not in the database");
    let mut core = (info.create)(48_000);
    core.set_bios(&slots);
    core.load(&rom).unwrap();
    let mut audio = Vec::new();
    let mut pictures = 0;
    for f in 0..900u32 {
        let start = if (600..608).contains(&f) { PadButton::Start.bit() } else { 0 };
        core.run_frame(&[start, 0]);
        core.take_audio(&mut audio);
        let frame = core.video();
        assert_eq!((frame.width, frame.height), (info.width, info.height));
        pictures += frame.pixels.iter().any(|&p| p != frame.pixels[0]) as u32;
    }
    // intros fade in and out: just ask for a picture most of the time
    assert!(pictures > 300, "only {pictures} frames with a picture");
    // 900 frames at ~59.92 fps, 48 kHz stereo: about 15 s of sound
    let expected = 900.0 * 48_000.0 * 2.0 * info.fps_den as f64 / info.fps_num as f64;
    assert!((audio.len() as f64 - expected).abs() < expected * 0.01, "{} samples", audio.len());
    assert!(audio.iter().any(|&s| s != 0.0), "silence");

    // save state: the next 120 frames are the same after loading it
    let state = core.save_state();
    let run = |core: &mut Box<dyn Core>| {
        for _ in 0..120 {
            core.run_frame(&[0, 0]);
        }
        core.video().pixels.to_vec()
    };
    let first = run(&mut core);
    core.load_state(&state).unwrap();
    assert_eq!(run(&mut core), first, "save state not exact");
    assert_eq!(core.battery_ram().map(<[u8]>::len), Some(32 * 1024));
}
