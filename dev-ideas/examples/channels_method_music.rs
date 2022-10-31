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

use std::time::Instant;
// use getset::{
//     CopyGetters,
//     Getters,
//     MutGetters,
//     Setters
// };
use std::rc::Rc;
use std::cell::{
    RefCell,
    RefMut
};
use std::thread::sleep;

use crossbeam_channel::unbounded as unbounded_channel;
use crossbeam_channel::bounded as bounded_channel;

#[derive(Debug, Copy, Clone)]
pub enum MusicState {
    Stopped,
    Paused,
    Playing
}

impl MusicState {
    #[inline]
    pub fn new_playing() -> Self {
        Self::Playing
    }

    #[inline]
    pub fn new_stopped() -> Self {
        Self::Stopped
    }

    #[inline]
    pub fn new_paused() -> Self {
        Self::Paused
    }

    pub fn play(&mut self) {
        match *self {
            Self::Stopped => {
                *self = MusicState::Playing;
            },
            Self::Paused => *self = MusicState::Playing,
            Self::Playing => {}
        }
    }

    pub fn pause(&mut self) {
        match *self {
            Self::Stopped => {
                *self = MusicState::Paused;
            },
            Self::Playing => {
                *self = MusicState::Paused;
            },
            Self::Paused => {}
        }
    }

    pub fn stop(&mut self) {
        match *self {
            Self::Stopped => {},
            Self::Paused => *self = MusicState::Stopped,
            Self::Playing => {
                *self = MusicState::Stopped;
            }
        }
    }

    pub fn do_based_on_state(
        &mut self,
        music_player: &mut MusicPlayer
    ) {
        let mp = music_player;

        match *self {
            MusicState::Stopped => {
                mp.stop_all();
            },
            MusicState::Playing => mp.continue_playing(),
            MusicState::Paused => mp.pause_playing_by_song("skeler")
        }
    }
}

#[derive(Debug)]
// #[derive(Getters, MutGetters, Debug)]
// #[getset(get = "pub", get_mut = "pub")]
pub struct AppState {
    text:        RefCell<String>,
    index:       RefCell<usize>,
    music_state: RefCell<MusicState>
}

use typeracer::{
    MusicPlayer,
    MusicPlayerResult
};

impl AppState {
    pub fn text_ref_mut(&self) -> RefMut<'_, String> {
        self.text.borrow_mut()
    }

    pub fn index_ref_mut(&self) -> RefMut<'_, usize> {
        self.index.borrow_mut()
    }

    pub fn music_state_ref_mut(&self) -> RefMut<'_, MusicState> {
        self.music_state.borrow_mut()
    }

    pub fn init() -> Self {
        let text = RefCell::new(String::from("rust"));
        let index = RefCell::new(0);
        let is_music_playing = RefCell::new(MusicState::new_stopped());
        Self {
            text,
            index,
            music_state: is_music_playing
        }
    }
}
use std::time::Duration;
use std::sync::{
    Arc,
    Mutex,
    RwLock
};
use std::thread::Builder as ThreadBuilder;

use rand::*;

fn main() {
    let app_state = AppState::init();
    let (sender, receiver) = bounded_channel::<MusicState>(1);

    let main_thread_sender = sender.clone();
    let music_thread_receiver = receiver.clone();

    let music_thread = ThreadBuilder::new()
        .name("music-thread".to_string())
        .spawn(move || {
            // waiting to get started
            let mut mp = MusicPlayer::from_volume(0.5).unwrap();

            mp.load_song_from_path(
                "static/audio/skeler-telaviv.mp3",
                "skeler"
            )
            .unwrap();

            loop {
                let music_state = music_thread_receiver.recv();
                if let Ok(MusicState::Playing) = music_state {
                    mp.play_all_songs_one_by_one();
                    break;
                }
                sleep(Duration::from_millis(10));
            }

            // this is blocking
            // write this instead of loop
            for mut music_state in music_thread_receiver.iter() {
                music_state.do_based_on_state(&mut mp);
            }
        })
        .unwrap();

    // time to play
    // it needs to get started somehow
    println!("started music");
    main_thread_sender.send(MusicState::Playing).unwrap();
    println!(
        "sleeping 5 seconds ... (the music is running on background)"
    );
    sleep(Duration::from_secs(5));

    let mut thread_rng = thread_rng();
    loop {
        let random_int = thread_rng.gen_range(0..4);
        println!("random int: {:?}", random_int);

        if random_int == 0 {
            println!("pause music!");
            let ms = MusicState::Paused;
            *app_state.music_state_ref_mut() = ms;
            main_thread_sender.send(ms).unwrap();
        }

        if random_int == 1 {
            println!("continue playing music");
            let ms = MusicState::Playing;
            *app_state.music_state_ref_mut() = ms;
            main_thread_sender.send(ms).unwrap();
        }

        sleep(std::time::Duration::from_millis(500));
    }
}
