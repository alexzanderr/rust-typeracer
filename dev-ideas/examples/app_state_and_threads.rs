#[allow(unused, dead_code)]
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

#[derive(Debug)]
pub enum MusicState {
    Stopped,
    Paused,
    Playing
}

#[derive(Debug)]
// #[derive(Getters, MutGetters, Debug)]
// #[getset(get = "pub", get_mut = "pub")]
pub struct AppState {
    text:        RefCell<String>,
    index:       RefCell<usize>,
    music_state: RefCell<MusicState>
}

use typeracer::MusicPlayer;

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
        let is_music_playing = RefCell::new(MusicState::Stopped);
        Self {
            text,
            index,
            music_state: is_music_playing
        }
    }
}

fn second_func(app_state: &AppState) {
    let mut text = app_state.text_ref_mut();
    let mut index = app_state.index_ref_mut();

    {
        *index = 123;
    }
    {
        *text = String::from("asd");
    }
    dbg!(&app_state);
}

fn applogic(app_state: &AppState) {
    dbg!(&app_state);
    second_func(&app_state);
}

use std::sync::{
    Arc,
    Mutex,
    RwLock
};
use std::thread::Builder as ThreadBuilder;

use rand::*;

fn main() {
    let app_state = AppState::init();
    let app_state_arc = Arc::new(Mutex::new(app_state));

    let app_state_arc_clone = app_state_arc.clone();

    let music_thread = ThreadBuilder::new()
        .name("music-thread".to_string())
        .spawn(move || {
            // TODO add the music player here
            let x = 123;
            let mut mp = MusicPlayer::from_volume(0.5).unwrap();

            mp.load_song_from_path(
                "static/audio/skeler-telaviv.mp3",
                "skeler"
            )
            .unwrap();

            // there inside MusicPLayer I could have
            // a reference to AppState, to modifiy the Music state automatically
            // but we'll see
            mp.play_all_songs_one_by_one();
            {
                let mut app_state_lock =
                    app_state_arc_clone.lock().unwrap();
                *app_state_lock.music_state_ref_mut() =
                    MusicState::Playing;
            }

            // this loop will pause the current thread
            // cause player is running in background
            // and if this finishes, music player is done
            loop {
                // println!("trying to get lock on app state ...");
                let mut app_state_lock = app_state_arc_clone.try_lock();

                match app_state_lock {
                    Ok(app_state_lock) => {
                        // here you will be trying to determine the music state
                        // using this enum
                        //
                        // pub enum MusicState {
                        //     Stopped,
                        //     Paused,
                        //     Playing
                        // }
                        // and do this accordingly
                        let mut music_state =
                            app_state_lock.music_state_ref_mut();

                        match *music_state {
                            MusicState::Stopped => {
                                mp.stop_all();
                            },
                            MusicState::Playing => mp.continue_playing(),
                            MusicState::Paused => {
                                mp.pause_playing_by_song("skeler")
                            },
                        }
                    },
                    Err(_) => {}
                }

                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        })
        .unwrap();

    let mut thread_rng = thread_rng();
    loop {
        let random_int = thread_rng.gen_range(0..4);
        println!("random int: {:?}", random_int);

        if random_int == 0 {
            println!("pause music!");

            // if the user pressed ctrl-m, for example ( music menu )
            // and for example selects to pause the music
            // this will immediately change the state of music
            // and the music thread will try lock at every 10 millis
            // and it will be updated instantly and the music will pause
            let app_state_lock = app_state_arc.lock().unwrap();
            let mut music_state = app_state_lock.music_state_ref_mut();

            *music_state = MusicState::Paused;
            // println!("music stopped playing: {:?}", music_state);
        }

        if random_int == 1 {
            println!("continue playing music");

            // if the user pressed ctrl-m, for example ( music menu )
            // and for example selects to pause the music
            // this will immediately change the state of music
            // and the music thread will try lock at every 10 millis
            // and it will be updated instantly and the music will pause
            let app_state_lock = app_state_arc.lock().unwrap();
            let mut music_state = app_state_lock.music_state_ref_mut();

            *music_state = MusicState::Playing;
            // println!("music stopped playing: {:?}", music_state);
        }

        // if random_int == 2 {
        //     println!("exiting ...");
        //     std::thread::sleep(std::time::Duration::from_millis(3000));
        //     break;
        //     // let app_state_lock = app_state_arc.lock().unwrap();
        //     // let mut is_music_playing =
        //     //     app_state_lock.music_state_ref_mut();

        //     // if *is_music_playing == false {
        //     //     println!("music is stopped: closing app");
        //     //     break;
        //     // } else {
        //     //     println!("cant stop the app: music is not stopped");
        //     // };
        // }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    // dbg!(&app_state);
    // applogic(&app_state);
    // dbg!(&app_state);
}
