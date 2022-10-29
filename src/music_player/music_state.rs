use std::time::Instant;
use std::rc::Rc;
use std::cell::{
    RefCell,
    RefMut
};

use super::MusicPlayer;

#[derive(Debug)]
pub enum MusicState {
    Stopped,
    Paused,
    Playing
    // IDEA
    // LoadSongFromPath(String)
    // PlaySong(String)
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
        // for simplity in the function usage
        // but for explicity in the parameter name
        let mp = music_player;

        match *self {
            MusicState::Stopped => {
                mp.stop_all();
            },
            MusicState::Playing => mp.continue_playing(),
            MusicState::Paused => mp.pause_playing()
        }
    }
}
