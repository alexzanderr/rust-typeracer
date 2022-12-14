use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::path::Path;
use std::collections::HashMap;

use soloud::*;

use super::{
    MusicPlayerErrors,
    MusicPlayerResult
};

lazy_static::lazy_static! {
    static ref INITIALIZED_MUSIC_PLAYER: AtomicBool = AtomicBool::new(false);
}

#[derive(Debug)]
// https://github.com/MoAlyousef/soloud-rs/issues/24
pub struct MusicPlayer {
    player:  Soloud,
    songs:   Option<HashMap<String, Wav>>,
    handles: Option<HashMap<String, Handle>>
}

impl MusicPlayer {
    pub fn stop_all(&mut self) {
        self.player.stop_all();
    }

    // BUG: this plays all the songs at the same time
    pub fn play_all_songs_one_by_one(&mut self) {
        if let Some(ref mut songs) = self.songs {
            for pair in songs.iter() {
                let (song_alias, song_wav) = pair;

                let handle = self.player.play(song_wav);
                let mut handles = HashMap::with_capacity(1);
                handles.insert(song_alias.clone(), handle);

                if let Some(ref mut self_handles) = self.handles {
                    self_handles.extend(handles);
                } else {
                    self.handles = Some(handles);
                }
                // cant put loop here because will block the main thread
            }
        }
    }

    pub fn load_songs_from_paths<P: AsRef<Path>>(
        &mut self,
        songs_aliases: &[&str],
        songs_paths: &[P]
    ) -> MusicPlayerResult<()> {
        debug_assert!(songs_aliases.len() == songs_paths.len());

        let mut songs = HashMap::with_capacity(songs_aliases.len());

        for pair in songs_paths.iter().zip(songs_aliases) {
            let (song_path, song_alias) = pair;
            let song_path = song_path.as_ref();

            let mut wav = Wav::default();
            wav.load(song_path)?;

            songs.insert(song_alias.to_string(), wav);
        }

        if let Some(ref mut self_songs) = self.songs {
            self_songs.extend(songs);
        } else {
            self.songs = Some(songs);
        }

        Ok(())
    }

    pub fn load_song_from_mem(
        &mut self,
        song_bytes: &[u8],
        song_alias: &str
    ) -> MusicPlayerResult<()> {
        let mut songs = HashMap::with_capacity(1);

        let mut wav = Wav::default();
        wav.load_mem(song_bytes)?;

        songs.insert(song_alias.to_string(), wav);

        if let Some(ref mut self_songs) = self.songs {
            self_songs.extend(songs);
        } else {
            self.songs = Some(songs);
        }

        Ok(())
    }

    pub fn load_songs_from_mem(
        &mut self,
        songs_aliases: &[&str],
        songs_bytes_matrix: &[&[u8]]
    ) -> MusicPlayerResult<()> {
        debug_assert!(songs_aliases.len() == songs_bytes_matrix.len());
        let mut songs = HashMap::with_capacity(songs_bytes_matrix.len());

        for pair in songs_bytes_matrix.iter().zip(songs_aliases) {
            let (song_bytes, song_alias) = pair;
            let mut wav = Wav::default();
            wav.load_mem(song_bytes)?;
            songs.insert(song_alias.to_string(), wav);
        }

        if let Some(ref mut self_songs) = self.songs {
            self_songs.extend(songs);
        } else {
            self.songs = Some(songs);
        }
        Ok(())
    }

    pub fn from_volume(global_volume: f32) -> MusicPlayerResult<Self> {
        if INITIALIZED_MUSIC_PLAYER.load(Ordering::Relaxed) {
            Err(MusicPlayerErrors::MusicPLayerAlreadyInitializedError)
        } else {
            INITIALIZED_MUSIC_PLAYER.store(true, Ordering::Relaxed);

            let mut _soloud = Soloud::default()?;
            _soloud.set_global_volume(global_volume);

            let mut wav = Wav::default();

            let _self = MusicPlayer {
                player:  _soloud,
                songs:   None,
                handles: None
            };

            Ok(_self)
        }
    }

    // pub fn new() -> Result<Self, SoloudError> {
    //     let mut sl = Soloud::default()?;
    //     // sl.set_global_volume(2.0);

    //     let mut wav = Wav::default();
    //     Ok(Self {
    //         player: sl,
    //         wav,
    //         playing: false,
    //         handle: None,
    //     })
    // }

    pub fn pause_playing_by_song(
        &mut self,
        song_alias: &str
    ) {
        if let Some(ref mut handles) = self.handles {
            if let Some((song_alias, song_handle)) =
                handles.get_key_value(song_alias)
            {
                self.player.set_pause(*song_handle, true);
            }
        }
    }

    pub fn pause_playing(&mut self) {
        if let Some(ref mut handles) = self.handles {
            for (song_alias, song_handle) in handles.iter() {
                self.player.set_pause(*song_handle, true);
            }
        }
    }

    pub fn continue_playing_by_song(
        &mut self,
        song_alias: &str
    ) {
        if let Some(ref mut handles) = self.handles {
            if let Some((song_alias, song_handle)) =
                handles.get_key_value(song_alias)
            {
                self.player.set_pause(*song_handle, false);
            }
        }
    }

    pub fn continue_playing(&mut self) {
        if let Some(ref mut handles) = self.handles {
            for (song_alias, song_handle) in handles.iter() {
                self.player.set_pause(*song_handle, false);
            }
        }
    }

    pub fn is_done_playing(&self) -> bool {
        // ?
        // self.player.is_voice_group_empty(voice_group_handle)
        self.player.voice_count() == 0
    }

    pub fn is_playing(&self) -> bool {
        self.player.voice_count() > 0
    }

    pub fn play_music_in_background(&mut self) {
        if let Some(ref mut songs) = self.songs {
            if songs.len() == 1 {
                for (song_alias, song_wav) in songs.iter() {
                    let handle = self.player.play(song_wav);

                    if let Some(ref mut handles) = self.handles {
                        handles.insert(song_alias.clone(), handle);
                    } else {
                        let mut hm = HashMap::new();
                        hm.insert(song_alias.clone(), handle);
                        self.handles = Some(hm);
                    }
                }
            }
        }
    }

    pub fn play_music_by_song_in_background(
        &mut self,
        song_alias: &str
    ) {
        if let Some(songs) = &self.songs {
            if let Some((song_alias, song_wav)) =
                songs.get_key_value(song_alias)
            {
                let handle = self.player.play(song_wav);
                if let Some(ref mut handles) = self.handles {
                    handles.insert(song_alias.clone(), handle);
                } else {
                    let mut hm = HashMap::new();
                    hm.insert(song_alias.clone(), handle);
                    self.handles = Some(hm);
                }
            }
        }
    }

    pub fn load_song_from_path<P: AsRef<Path>>(
        &mut self,
        song_path: P,
        song_alias: &str
    ) -> MusicPlayerResult<()> {
        let mut songs = HashMap::with_capacity(1);

        let mut wav = Wav::default();
        wav.load(song_path.as_ref())?;

        songs.insert(song_alias.to_string(), wav);

        if let Some(ref mut self_songs) = self.songs {
            self_songs.extend(songs);
        } else {
            self.songs = Some(songs);
        }

        Ok(())
    }

    fn play_song_by_alias(
        &self,
        alias: &str
    ) -> MusicPlayerResult<()> {
        todo!();

        Ok(())
    }

    fn stop_song_by_alias(
        &self,
        arg: &str
    ) -> MusicPlayerResult<()> {
        todo!();

        Ok(())
    }
}

#[test]
fn design_music_player() -> MusicPlayerResult<()> {
    let player = MusicPlayer::from_volume(0.5)?;

    // wav.load(&std::path::Path::new("static/audio/skeler-telaviv.mp3"))?;
    // let handle = sl.play(&wav);

    player.load_song_from_path(
        std::path::Path::new("static/audio/davai_hardbass.wav"),
        "davai-hardbass"
    )?;

    player.load_song_from_path(
        std::path::Path::new("static/audio/skeler-telaviv.mp3"),
        "skeler"
    )?;

    player.play_song_by_alias("skeler")?;
    player.stop_song_by_alias("skeler")?;

    Ok(())
}
