#[allow(unused, dead_code)]
use std::time::Instant;
use std::rc::Rc;
use std::cell::{
    RefCell,
    RefMut
};
use std::sync::{
    Arc,
    Mutex,
    RwLock
};
use std::thread::Builder as ThreadBuilder;
use std::time::Duration;
use std::thread::sleep;
use std::io::stdout;
use std::io::Write;

use rand::*;

#[derive(Debug)]
pub enum GameState {
    Paused,
    Playing
}

#[derive(Debug)]
pub struct AppState {
    text:         RefCell<String>,
    index:        RefCell<usize>,
    elapsed_time: RefCell<usize>,
    game_state:   RefCell<GameState>
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

    pub fn elapsed_time_ref_mut(&self) -> RefMut<'_, usize> {
        self.elapsed_time.borrow_mut()
    }

    pub fn game_state_ref_mut(&self) -> RefMut<'_, GameState> {
        self.game_state.borrow_mut()
    }

    pub fn init() -> Self {
        let text = RefCell::new(String::from("rust"));
        let index = RefCell::new(0);
        let elapsed_time = RefCell::new(0);
        let game_state = RefCell::new(GameState::Playing);
        Self {
            text,
            index,
            elapsed_time,
            game_state
        }
    }
}

fn main() {
    let app_state = AppState::init();
    let app_state_arc = Arc::new(Mutex::new(app_state));

    let app_state_arc_clone = app_state_arc.clone();

    let music_thread = ThreadBuilder::new()
        .name("music-thread".to_string())
        .spawn(move || {
            let app_state_arc = app_state_arc_clone;
            loop {
                // println!("trying to get lock on app state ...");
                if let Ok(mut app_state_mutex) = app_state_arc.lock() {
                    let mut game_state =
                        app_state_mutex.game_state_ref_mut();
                    match *game_state {
                        GameState::Paused => {},
                        GameState::Playing => {
                            let mut elapsed =
                                app_state_mutex.elapsed_time_ref_mut();
                            *elapsed += 1;
                        }
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
        })
        .unwrap();

    let mut thread_rng = thread_rng();
    let mut stdout = stdout();
    'mainloop: loop {
        print!("enter game state > ");
        stdout.flush().unwrap();
        let mut buf = String::from("");
        let _ = std::io::stdin().read_line(&mut buf);
        buf = buf.replace('\n', "");
        buf = buf.trim().to_string();
        println!("{:?}", buf);
        {
            let app_state_mutex = app_state_arc.lock().unwrap();
            let mut game_state = app_state_mutex.game_state_ref_mut();

            if buf == "p" {
                dbg!("paused");
                *game_state = GameState::Paused;
            } else if buf == "c" {
                *game_state = GameState::Playing;
            }
            let elapsed = app_state_mutex.elapsed_time_ref_mut();
            println!("elapsed from main: {}", *elapsed);
        }

        // sleep(Duration::from_millis(100));
    }
}
