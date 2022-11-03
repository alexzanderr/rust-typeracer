use std::io::Write;
use std::time::Instant;

use soloud::*;
use rand::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = Soloud::default()?;

    let mut wav = audio::Wav::default();

    // let song_bytes = include_bytes!("../static/audio/skeler-telaviv.mp3");
    // WARNING: DONT RUN THIS; doesnt work; it plays gibberish and destroys my ears
    // unsafe { wav.load_raw_wav_8(song_bytes)?; }

    wav.load(&std::path::Path::new("static/audio/skeler-telaviv.mp3"))?;

    let handle = sl.play(&wav);
    let bc = sl.backend_channels();
    sl.set_global_volume(0.5);

    dbg!(bc);

    let time_handle = Instant::now();
    while sl.voice_count() > 0 {
        let elapsed = time_handle.elapsed().as_secs();
        dbg!(elapsed);

        // 3 seconds passed
        if elapsed == 2 {
            sl.stop_all();
            // sl.stop(voice_handle)
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let handle = sl.play(&wav);
    // sl.set_volume(handle, 0.5);

    let time_handle = Instant::now();
    while sl.voice_count() > 0 {
        let elapsed = time_handle.elapsed().as_secs();
        dbg!(elapsed);

        // 3 seconds passed
        if elapsed == 2 {
            sl.stop_all();
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
