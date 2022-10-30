use std::io::Write;
use std::time::Duration;
use std::sync::{
    Arc,
    Mutex,
    RwLock
};
use std::cell::RefCell;
use std::thread::{
    Builder as ThreadBuilder,
    JoinHandle
};

use colored::*;
use core_dev::datetime::datetime::get_current_datetime;
use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
    KeyEventState,
    KeyModifiers,
    MouseButton,
    MouseEvent,
    MouseEventKind
};
use crossterm::{
    cursor,
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture
    },
    execute,
    style,
    terminal::{
        self,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
    tty,
    Result as CrosstermResult
};

use super::GameState;
use crate::{
    MusicPlayer,
    MusicPlayerResult,
    MusicState
};
use super::AppState;
use super::errors::{
    IndexOutOfBoundsError,
    SpanError,
    TyperacerErrors,
    TyperacerResult
};
use crate::statics::{
    PLAY_CS16_SOUND,
    PROMPT_ARROW,
    SKELER_TELATIV_SONG,
    TERMINAL_CURSOR
};
use crate::terminal_screen::TerminalScreen;
use super::Stats;
use super::TyperacerUI;

pub enum LoopActions {
    TimeToBreak,
    TimeToContinue,
    GameFinished,
    LoopDoesntQuit,
    PauseGame,
    ContinueGame,
    QuitGame
}

#[derive(Debug)]
pub struct Typeracer<'a> {
    ui:    TyperacerUI<'a>,
    // IDEA: this pointer is valid as long as the struct is alive
    // so i dont need to clone every time i need to use it inside the struct
    // just clone when you move this self.state on the music thread
    // just use .lock() in the game logic; dont .clone() anymore
    state: Arc<Mutex<AppState>>
}

impl<'a> Typeracer<'a> {
    // pub fn default() -> TyperacerResult<Self> {
    //     let mut term = TerminalScreen::builder()
    //         .alternate(true)
    //         .capture_mouse(false)
    //         .build()?;

    //     term.enter_raw_terminal()?;
    //     term.set_panic_hook();

    //     let _self = Self::from_term(&mut term);

    //     Ok(_self)
    // }

    pub fn from_term(term: &'a mut TerminalScreen) -> Self {
        let ui = TyperacerUI::from_term(term);
        let state = Arc::new(Mutex::new(AppState::init()));
        Self {
            ui,
            state
        }
    }

    // TODO: this needs to be better
    // game logic
    pub fn handle_ctrl_backspace(
        &self,
        user_input_prompt: &mut String
    ) {
        // clear the user input prompt
        if let Some(last_space_index) = user_input_prompt.rfind(' ') {
            if let Some(last_char) =
                user_input_prompt.chars().nth(user_input_prompt.len())
            {
                if last_char == ' ' {
                    user_input_prompt.remove(user_input_prompt.len() - 1);
                }
            }
            user_input_prompt.replace_range(
                last_space_index..user_input_prompt.len(),
                ""
            )
        } else {
            user_input_prompt.clear();
        }
    }

    fn create_and_spawn_music_thread(
        &self
    ) -> Result<JoinHandle<MusicPlayerResult<()>>, std::io::Error> {
        let app_state_arc = self.state.clone();

        let music_thread = ThreadBuilder::new()
            .name("typeracer-music-thread".to_string())
            .spawn(move || -> MusicPlayerResult<()> {
                let mut mp = MusicPlayer::from_volume(0.5)?;

                // this read operating slows the startup
                // TODO: still after putting the songs inside binary
                // load_from_mem is slow too
                // i need to to this operation async, or on another thread
                mp.load_song_from_mem(PLAY_CS16_SOUND, "play")?;
                mp.load_song_from_mem(SKELER_TELATIV_SONG, "skeler")?;

                // there inside MusicPLayer I could have
                // a reference to AppState, to modifiy the Music state automatically
                // but we'll see
                mp.play_all_songs_one_by_one();
                {
                    let mut app_state_lock = app_state_arc.lock();

                    match app_state_lock {
                        Ok(app_state_mutex) => {
                            *app_state_mutex.music_state_ref_mut() =
                                MusicState::new_playing();
                        },
                        Err(_) => {}
                    }
                }

                // this loop will pause the current thread
                // cause player is running in background
                // and if this finishes, music player is done
                loop {
                    let mut app_state_lock = app_state_arc.try_lock();

                    match app_state_lock {
                        Ok(app_state_mutex) => {
                            let mut music_state =
                                app_state_mutex.music_state_ref_mut();

                            // TODO: 1. how about music_player.do_based_on_state()
                            // 2. how about the music_player to contain the state?

                            music_state.do_based_on_state(&mut mp);
                        },
                        Err(_) => {}
                    }

                    std::thread::sleep(std::time::Duration::from_millis(
                        10
                    ));
                }

                Ok(())
            })?;

        Ok(music_thread)
    }

    fn game_loop(mut self) -> TyperacerResult<LoopActions> {
        // IDEA: I could hold this arc pointer inside Self instead of state: AppState
        // if i cant hold the reference inside the mutex
        // let app_state_arc = Arc::new(Mutex::new(self.state));
        // clones the pointer here

        // arc pointer is in self as field
        let music_thread_handle = self.create_and_spawn_music_thread();
        let stopwatch_thread_handle =
            self.create_and_spawn_stopwatch_thread();

        // there is one thing i can do
        // make a thread for music only, and only share on that thread AppSgstate
        // and stop the music from that thread while playing using AppState

        loop {
            // render ui
            {
                let app_state_arc = self.state.clone();
                self.ui.draw(app_state_arc)?;
            }

            // if *self.state.game_finished_ref_mut() {
            //     self.ui
            //         .print(
            //             "Congratulations! <press any key to leave game>",
            //             19,
            //             0
            //         )?
            //         .flush_stdout()?;

            //     event::read()?;
            //     return Ok(LoopActions::TimeToBreak);
            // }

            // handle keyboard input
            if event::poll(Duration::from_millis(100))? {
                let event = event::read()?;
                let loop_action = {
                    let app_state_arc = self.state.clone();
                    self.handle_event(event, app_state_arc)?.1
                };
                // handle key particurarly
                match loop_action {
                    LoopActions::TimeToBreak => {
                        return Ok(LoopActions::TimeToBreak)
                    },
                    LoopActions::TimeToContinue => continue,
                    LoopActions::GameFinished => continue,
                    LoopActions::LoopDoesntQuit => {
                        // do nothing, just continues (not continue from programming)
                    },
                    LoopActions::QuitGame => {
                        return Ok(LoopActions::QuitGame)
                    },
                    _ => {
                        // the rest are not implemented
                    }
                }

                // logging
                // let app_state = format!("{:#?}", &self.state);
                // let mut handler = std::fs::File::options()
                //     // .append(true)
                //     .create(true)
                //     .truncate(true)
                //     .write(true)
                //     .open("logs/log-from-loop.text")?;
                // write!(handler, "{}\n\n", app_state)?;
            }
        }
    }

    pub fn mainloop(mut self) -> TyperacerResult<()> {
        self.game_loop()?;
        Ok(())
    }

    fn handle_key_event(
        &self,
        key_event: Event
    ) -> TyperacerResult<(&Self, LoopActions)> {
        let app_state = match self.state.lock() {
            Ok(app_state) => app_state,
            Err(error) => return Err(TyperacerErrors::PoisonError)
        };

        todo!()
    }

    fn handle_event(
        &self,
        event: Event,
        // TODO, remove this
        // the pointer is inside the struct
        app_state_arc: Arc<Mutex<AppState>>
    ) -> TyperacerResult<(&Self, LoopActions)> {
        let app_state = match app_state_arc.lock() {
            Ok(app_state) => app_state,
            Err(error) => return Err(TyperacerErrors::PoisonError)
        };

        // pointers to AppState fields
        let mut keyboard_input = app_state.keyboard_input_ref_mut();
        let mut index = app_state.index_ref_mut();
        let mut wrong_index = app_state.wrong_index_ref_mut();
        let mut typeracer_text = app_state.typeracer_text_ref_mut();
        let mut game_finished = app_state.game_finished_ref_mut();
        let mut user_input_prompt = app_state.user_input_prompt_ref_mut();
        let mut what_was_typed = app_state.what_was_typed_ref_mut();
        let mut user_input_prompt_x =
            app_state.user_input_prompt_x_ref_mut();
        let term_width = self.ui.term_width();

        let mut cursor_x = app_state.cursor_x_ref_mut();
        let mut cursor_y = app_state.cursor_y_ref_mut();
        let mut index_shadow = app_state.index_shadow_ref_mut();
        let mut wrong_index_shadow =
            app_state.wrong_index_shadow_ref_mut();
        let mut current_line = app_state.current_line_ref_mut();

        let mut music_state = app_state.music_state_ref_mut();

        let mut game_state = app_state.game_state_ref_mut();
        let elapsed = app_state.elapsed_time_ref_mut();

        match event {
            Event::FocusGained => {
                todo!("do something if terminal focus is gained")
            },
            Event::FocusLost => {
                todo!("do something if terminal focus is LOST")
            },
            Event::Paste(string_from_ctrl_v) => {},
            Event::Resize(y, x) => {},
            Event::Mouse(mevent) => {
                // dbg!(mevent);
                let mouse_kind = mevent.kind;
                match mouse_kind {
                    MouseEventKind::Down(MouseButton::Right) => {},
                    MouseEventKind::Up(MouseButton::Right) => {},
                    _ => {}
                }
            },
            // TODO: add `Self::handle_key_event()` here to make the code more modular and more readable
            Event::Key(kevent) => {
                let event_clone = format!("{:?}", kevent.code.clone());
                *keyboard_input = event_clone.yellow().to_string();

                match kevent {
                    // this will pause the game and also pause the music and the stopwatch
                    KeyEvent { code: KeyCode::Char(' '), modifiers: KeyModifiers::CONTROL, ..} => {
                        match *game_state {
                            GameState::Paused => {
                                *game_state = GameState::Playing;
                                match *music_state {
                                    MusicState::Stopped => {
                                        music_state.play();
                                    },
                                    MusicState::Paused => music_state.play(),
                                    _ => {}
                                    // MusicState::Playing => music_state.pause(),
                                }
                            }
                            GameState::Playing => {
                                *game_state = GameState::Paused;

                                match *music_state {
                                    // MusicState::Stopped => {
                                    //     music_state.play();
                                    // },
                                    // MusicState::Paused => music_state.play(),
                                    MusicState::Playing => music_state.pause(),
                                    _ => {}
                                }
                            },
                        }
                    },
                    // clear the entire user_input_bar
                    // and append the text to the text area
                    // enter or space into the user_input_prompt
                    KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        // here happens this at the end of the race


                        // [error]: IndexOutOfBounds
                        //     text: "Rust is blazingly fast and memory-efficient:
                        // with no runtime or garbage collector,
                        // it can power performance-critical services,
                        // run on embedded devices,
                        // and easily integrate with other languages.

                        // Rust's rich type system and ownership model
                        // guarantee memory-safety and thread-safety
                        // - enabling you to eliminate
                        // many classes of bugs at compile-time."
                        //     index: 347
                        //     text.len(): 347
                        //     span: [src/typeracer/typeracer.rs:351:79]

                        let error_span = SpanError::new(file!(), line!() + 1, column!());
                        let index_error = IndexOutOfBoundsError::new(
                            *index,
                            typeracer_text.to_string(),
                            error_span
                        );


                        //typeracer logic

                        if '\n' == typeracer_text.chars().nth(*index).ok_or_else(|| TyperacerErrors::IndexOutOfBounds(index_error.clone()))?
                            && *wrong_index == 0
                        {
                            *index += 1;
                            *cursor_x += 1;
                            *current_line += 1;
                            *cursor_y = 0;
                            *index_shadow = 0;
                            *wrong_index_shadow = 0;

                            if *index == typeracer_text.len() {
                                *game_finished = true;
                            }
                        } else if *index + *wrong_index < typeracer_text.len() {
                            let current_index = *index + *wrong_index;

                            // if the cursor is at the end of the line
                            // but everything is wrong
                            // you cannot continue to next line
                            if '\n' == typeracer_text.chars().nth(current_index).ok_or_else(|| {TyperacerErrors::IndexOutOfBounds(index_error.clone())})? {
                                return Ok((self, LoopActions::TimeToContinue))
                            }
                            // dbg!("herer");
                            *wrong_index += 1;
                            *wrong_index_shadow += 1;
                        }

                        // TODO: recomment this
                        // // ui logic
                        // let time_to_continue = self.handle_enter_key(
                        //     &mut what_was_typed,
                        //     &mut user_input_prompt,
                        //     user_input_prompt_x.clone())?;

                        // if time_to_continue {
                        //     return Ok((self, LoopActions::TimeToContinue))
                        // }
                    },
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: event::KeyModifiers::CONTROL,
                        ..
                    }
                    | KeyEvent {
                        code: KeyCode::Char('d'),
                        modifiers: event::KeyModifiers::CONTROL,
                        ..
                    } => {
                        return Ok((self, LoopActions::QuitGame))
                    },
                    // backspace
                    // delete one char backward
                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        let error_span = SpanError::new(file!(), line!() + 1, column!());
                        let index_error = IndexOutOfBoundsError::new(
                            *index,
                            typeracer_text.to_string(),
                            error_span
                        );

                        // if you are at the begginning of a line
                        // but that line is not the first line
                        // you cannot go back on the previous
                        // this behavious also happens in typing.io
                        if *current_line > 0 {
                            // one char backwards
                            let current_index = *index + *wrong_index - 1;
                            if '\n' == typeracer_text.chars().nth(current_index).ok_or_else(|| {TyperacerErrors::IndexOutOfBounds(index_error.clone())
                            })?
                            {
                                return Ok((self, LoopActions::TimeToContinue))
                            }
                        }

                        // ui logic
                        let _ = user_input_prompt.pop();

                        // logic for the typeracer game
                        if *wrong_index > 0 {
                            *wrong_index -= 1;
                            if *wrong_index_shadow > 0 {
                                *wrong_index_shadow -= 1;
                            }
                        } else if *index > 0 {
                            *index -= 1;
                            if *index_shadow > 0 {
                                *index_shadow -= 1;

                            }
                        }

                    },
                    // ctrl + backspace, doesnt work, cuz terminal stuff, i am guessing
                    // but ctrl + h works, cuz linux
                    //
                    // delete the entire word backwards
                    KeyEvent {
                        code: KeyCode::Char('h'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }
                    // and also for the same branch alt + backspace
                    | KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: KeyModifiers::ALT,
                        ..
                        // kind: KeyEventKind::Repeat | KeyEventKind::Release,
                        // state: KeyEventState::NONE
                    } => {
                        self.handle_ctrl_backspace(&mut user_input_prompt)
                    },
                    KeyEvent {
                        code: KeyCode::Char('s'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        match *music_state {
                            MusicState::Stopped => {
                                music_state.play();
                            },
                            MusicState::Paused => music_state.play(),
                            MusicState::Playing => music_state.pause(),
                        }
                    },
                    // user pressed a char key on keyboard
                    // append it to the prompt
                    KeyEvent {
                        code: KeyCode::Char(character),
                        modifiers: event::KeyModifiers::NONE | event::KeyModifiers::SHIFT,
                        ..
                    } => {
                        let error_span = SpanError::new(file!(), line!() + 1, column!());
                        let index_error = IndexOutOfBoundsError::new(
                            *index,
                            typeracer_text.to_string(),
                            error_span
                        );

                        let current_index = *index + *wrong_index;
                        if '\n' == typeracer_text.chars().nth(current_index)
                            .ok_or_else( || {
                            TyperacerErrors::IndexOutOfBounds(index_error.clone())
                         })? {
                            return Ok((self, LoopActions::TimeToContinue))
                        }

                        if character == ' ' {
                            what_was_typed.push_str(&user_input_prompt);
                            // what_was_typed.push(' ');
                            if what_was_typed.len() >= term_width - 6 {
                                what_was_typed.clear();
                            }
                            user_input_prompt.clear();
                        }

                        self.handle_any_character(&mut what_was_typed, &mut user_input_prompt, character);

                        if *index == typeracer_text.len() - 1 {
                            *index += 1;
                            *index_shadow += 1;

                            *game_finished = true;
                            return Ok((self, LoopActions::GameFinished))
                        }

                        // typeracer game logic
                        if character == typeracer_text.chars().nth(*index).ok_or(TyperacerErrors::IndexOutOfBounds(index_error))?
                            && *wrong_index == 0
                        {
                            *index += 1;
                            *index_shadow += 1;

                            if *index == typeracer_text.len() {
                                *game_finished = true;
                            }
                        } else if *index + *wrong_index < typeracer_text.len() {
                            *wrong_index += 1;
                            *wrong_index_shadow += 1;
                        }
                    },
                    _ => {},
                }
            },
            _ => {}
        }

        *cursor_y = *index_shadow + *wrong_index_shadow;

        Ok((self, LoopActions::LoopDoesntQuit))
    }

    fn handle_enter_key(
        &self,
        what_was_typed: &mut String,
        user_input_prompt: &mut String,
        user_input_prompt_x: usize
    ) -> TyperacerResult<bool> {
        let term_height = self.ui.term_height();
        let term_width = self.ui.term_width();

        if user_input_prompt.is_empty() {
            return Ok(true);
        }

        what_was_typed.push_str(user_input_prompt);
        what_was_typed.push(' ');

        if what_was_typed.len() >= term_width - 6 {
            what_was_typed.clear();
        }
        user_input_prompt.clear();

        Ok(false)
    }

    fn handle_any_character(
        &self,
        what_was_typed: &mut String,
        user_input_prompt: &mut String,
        character: char
    ) {
        user_input_prompt.push(character);
        // 2 from my red cursor _
        // 1 from char `_`
        // 1 from the ansi red color
        if user_input_prompt.len() == self.ui.term_width() - 11 {
            user_input_prompt.clear();
        }
    }

    fn create_and_spawn_stopwatch_thread(
        &self
    ) -> Result<JoinHandle<MusicPlayerResult<()>>, std::io::Error> {
        let app_state_arc = self.state.clone();

        let stopwatch_thread = ThreadBuilder::new()
            .name("stopwatch-thread".to_string())
            .spawn(move || {
                // let _ = 123;
                loop {
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

                    std::thread::sleep(std::time::Duration::from_millis(
                        1000
                    ));
                }
            })?;

        Ok(stopwatch_thread)
    }
}
