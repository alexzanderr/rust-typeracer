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
#![feature(test)]


// exactly like `extern crate proc_macro;`
// rust's built in crates
extern crate test;
use test::{Bencher, black_box};

use soloud::*;
use typeracer::MusicPlayer;

pub const SKELER_TELATIV_SONG: &'static [u8] =
    include_bytes!("../static/audio/skeler-telaviv.mp3");
pub const PLAY_CS16_SOUND: &'static [u8] =
    include_bytes!("../static/audio/play_cs16.wav");

// cant use this, doesnt work
/// btw dont run this code; it takes a lot of time: i dont know how many iterations is doing
/// test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 79.01s
#[bench]
fn bench_my_code(bencher: &mut Bencher) {
    /// to actually see how slow is this
    /// test bench_my_code ... bench: 263,774,891 ns/iter (+/- 48,287,064)
    /// 0.26 seconds just ot load a single wav, its huge, extremely slow
    bencher.iter(|| {
        let x = black_box(123);
        // let mut mp = Soloud::default().unwrap();
        let mut wav = Wav::default();
        wav.load_mem(SKELER_TELATIV_SONG).unwrap();
        // wav.load_mem(PLAY_CS16_SOUND).unwrap();
    })
}
