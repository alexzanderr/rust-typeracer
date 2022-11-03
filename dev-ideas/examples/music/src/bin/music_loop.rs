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

    wav.load(&std::path::Path::new("static/audio/davai_hardbass.wav"))?;

    let handle = sl.play(&wav);
    sl.set_global_volume(0.5);

    let bc = sl.backend_channels();
    dbg!(bc);

    // works
    sl.set_looping(handle, true);

    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
