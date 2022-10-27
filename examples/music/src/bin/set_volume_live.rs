use std::io::Write;

use soloud::*;
use rand::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = Soloud::default()?;

    let mut wav = audio::Wav::default();
    let mut filt = filter::EchoFilter::default();
    // let mut filt = filter::
    let mut filt = filter::BassboostFilter::default();
    filt.set_params(1.0)?;

    let song_bytes = include_bytes!("../static/audio/skeler-telaviv.mp3");
    // WARNING: DONT RUN THIS; doesnt work; it plays gibberish and destroys my ears
    // unsafe { wav.load_raw_wav_8(song_bytes)?; }

    wav.load(&std::path::Path::new("static/audio/skeler-telaviv.mp3"))?;
    wav.set_filter(0, Some(&filt));
    let handle = sl.play(&wav);


    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut thread_rng = thread_rng();
    while sl.voice_count() > 0 {
        // let random_float = thread_rng.gen::<f32>() * 3.0;
        // dbg!(random_float);

        let mut line = String::from("");
        print!("your volume please > ");
        stdout.flush().unwrap();
        stdin.read_line(&mut line);
        if line == "\n" {
            continue;
        }
        line.remove(line.len() - 1);

        let volume_value = line.parse::<f32>().unwrap();

        sl.set_volume(handle, volume_value);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
