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

    // but load all the files at the beginning of the game to make sure they are in memory
    // TODO: use wav.load_mem(&bytes)?; to load from memory in the middle of the game

    wav.load(&std::path::Path::new("static/audio/skeler-telaviv.mp3"))?;
    let handle = sl.play(&wav);

    let mut wav = audio::Wav::default();
    wav.load(&std::path::Path::new("static/audio/davai_hardbass.wav"))?;
    let handle2 = sl.play(&wav);
    sl.set_looping(handle2, true);

    let bc = sl.backend_channels();
    sl.set_global_volume(0.5);

    dbg!(bc);

    let time_handle = Instant::now();
    while sl.voice_count() > 0 {
        let elapsed = time_handle.elapsed().as_secs();
        dbg!(elapsed);
        let current_volume = sl.volume(handle);
        dbg!(current_volume);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    // 0.0 la final
    let stream_time = sl.stream_time(handle);
    dbg!(stream_time);

    Ok(())
}
