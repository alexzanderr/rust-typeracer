
#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use,
    non_upper_case_globals,
    non_camel_case_types,
    semicolon_in_expressions_from_macros,
    redundant_semicolons,
    unused_macros,
)]

mod common;
use common::{
    PLAY_CS16_SOUND,
    SKELER_TELAVIV_SONG
};

use typeracer as game;
use game::{MusicPlayer, MusicPlayerResult};
use std::time::Instant;


use std::sync::{
    Arc,
    Mutex
};

pub type GenericResult<T> =
core::result::Result<T, Box<dyn std::error::Error>>;


// https://doc.rust-lang.org/nomicon/send-and-sync.html
pub struct WavWrapper(Wav);

// to fix the stupid error about *mut *mut c_void cannot be sent between threads
// even when i use arc mutex or even when im not moving any Wav between threads
unsafe impl Send for WavWrapper {
}

#[cfg(test)]
mod load_from_mem {
    use super::*;

    // #[test]
    // fn load_1() -> MusicPlayerResult<()> {
    //     let mut mp = MusicPlayer::new()?;
    //
    //     let now = Instant::now();
    //
    //     mp.load_song_from_mem(SKELER_TELAVIV_SONG, "skeler");
    //     drop(mp);
    //
    //     let elapsed = now.elapsed().as_secs_f32();
    //
    //     println!("execution time for 1 load: {elapsed}");
    //
    //     Ok(())
    // }

    /// execution time for 2 loads: 2.9101417 seconds
    /// a lot, really bad
    #[test]
    fn load_lots() -> MusicPlayerResult<()> {
        let mut mp = MusicPlayer::new()?;

        let now = Instant::now();

        mp.load_song_from_mem(SKELER_TELAVIV_SONG, "skeler");
        mp.load_song_from_mem(SKELER_TELAVIV_SONG, "skeler");
        mp.load_song_from_mem(SKELER_TELAVIV_SONG, "skeler");
        mp.load_song_from_mem(SKELER_TELAVIV_SONG, "skeler");
        mp.load_song_from_mem(SKELER_TELAVIV_SONG, "skeler");
        mp.load_song_from_mem(SKELER_TELAVIV_SONG, "skeler");
        mp.load_song_from_mem(SKELER_TELAVIV_SONG, "skeler");
        mp.load_song_from_mem(SKELER_TELAVIV_SONG, "skeler");
        mp.load_song_from_mem(SKELER_TELAVIV_SONG, "skeler");
        mp.load_song_from_mem(SKELER_TELAVIV_SONG, "skeler");
        mp.load_song_from_mem(SKELER_TELAVIV_SONG, "skeler");
        mp.load_song_from_mem(PLAY_CS16_SOUND, "cs16");
        mp.load_song_from_mem(PLAY_CS16_SOUND, "cs16");
        mp.load_song_from_mem(PLAY_CS16_SOUND, "cs16");
        mp.load_song_from_mem(PLAY_CS16_SOUND, "cs16");
        mp.load_song_from_mem(PLAY_CS16_SOUND, "cs16");
        mp.load_song_from_mem(PLAY_CS16_SOUND, "cs16");
        mp.load_song_from_mem(PLAY_CS16_SOUND, "cs16");
        mp.load_song_from_mem(PLAY_CS16_SOUND, "cs16");
        mp.load_song_from_mem(PLAY_CS16_SOUND, "cs16");
        mp.load_song_from_mem(PLAY_CS16_SOUND, "cs16");
        mp.load_song_from_mem(PLAY_CS16_SOUND, "cs16");
        mp.load_song_from_mem(PLAY_CS16_SOUND, "cs16");

        let elapsed = now.elapsed().as_secs_f32();

        println!("execution time for 2 loads: {elapsed}");
        Ok(())
    }


    #[test]
    fn main() -> GenericResult<()> {
        let songs_arc: Arc<Mutex<Vec<WavWrapper>>> =
            Arc::new(Mutex::new(Vec::with_capacity(15)));

        let mut thread_handles = Vec::with_capacity(15);

        for _ in 0..15 {
            let songs_arc = songs_arc.clone();

            let thread_handle = std::thread::spawn(move || {
                // 1
                let mut w1 = Wav::default();
                w1.load_mem(PLAY_CS16_SOUND);

                // 2
                let mut w2 = Wav::default();
                w2.load_mem(SKELER_TELATIV_SONG);

                let w1 = WavWrapper(w1);
                let w2 = WavWrapper(w2);
                (w1, w2)
            });

            thread_handles.push(thread_handle);
        }

        for h in thread_handles {
            let result = h.join().unwrap();
        }

        Ok(())
    }

}
