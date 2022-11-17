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
    unused_macros
)]

use std::sync::{
    Arc,
    Mutex
};

pub type GenericResult<T> =
    core::result::Result<T, Box<dyn std::error::Error>>;

use soloud::*;
use typeracer::MusicPlayer;

pub const SKELER_TELATIV_SONG: &'static [u8] =
    include_bytes!("../../static/audio/skeler-telaviv.mp3");
pub const PLAY_CS16_SOUND: &'static [u8] =
    include_bytes!("../../static/audio/play_cs16.wav");

// https://doc.rust-lang.org/nomicon/send-and-sync.html
pub struct WavWrapper(Wav);

// to fix the stupid error about *mut *mut c_void cannot be sent between threads
// even when i use arc mutex or even when im not moving any Wav between threads
unsafe impl Send for WavWrapper {
}

/// ./target/debug/examples/wav_on_threads  4.96s user 0.60s system 642% cpu 0.866 total
fn main() -> GenericResult<()> {
    let songs_arc: Arc<Mutex<Vec<WavWrapper>>> =
        Arc::new(Mutex::new(Vec::with_capacity(15)));

    let mut thread_handles = Vec::with_capacity(15);

    // Soloud can be sent between threads
    let sol = Soloud::default().unwrap();
    let sol_arc = Arc::new(Mutex::new(sol));
    for _ in 0..15 {
        let songs_arc = songs_arc.clone();
        let sol_arc_clone = sol_arc.clone();

        let thread_handle = std::thread::spawn(move || {
            let sol = sol_arc_clone;
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
