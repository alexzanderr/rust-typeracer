use std::sync::MutexGuard;
use std::io::Write;
use std::time::{
    Duration,
    Instant,
};
use std::sync::{
    Arc,
    Mutex,
    RwLock,
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
    Noop,
    PauseGame,
    ContinueGame,
    ForceQuitGame
}

use crate::{
    ConfigResult,
    TyperacerConfig
};

#[derive(Debug)]
/// Typeracer main handle to the game
pub struct Typeracer<'a> {
    /// separate UI with specific methods
    ui: TyperacerUI<'a>,
    /// entire app state
    /// similar to `context` in other "contexts"
    app_state: Arc<Mutex<AppState>>,
    /// configuration for the game
    // NOTE: I suppose you will need to make arc<mutex<>> from this one
    config: TyperacerConfig,
}

impl<'a> Typeracer<'a> {
    // IDEA: implement Drop trait to disable raw terminal mode when
    // this entire struct goes away
    // to simplify the code
    // to not initialize a terminal every time
    //
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

    /// typeracer/src/typeracer/typeracer.rs
    pub fn from_term(term: &'a mut TerminalScreen) -> Self {
        let ui = TyperacerUI::from_term(term);
        let app_state = Arc::new(Mutex::new(AppState::init()));
        // this is ugly; i dont want this in the future
        // let config = TyperacerConfig::load_default_path().unwrap();
        let config = TyperacerConfig::load_from_str(include_str!(
            "../../config.toml"
        ))
            .unwrap();

        Self {
            ui,
            app_state,
            config,
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

    #[cfg(feature = "music")]
    fn create_and_spawn_music_thread(
        &self
    ) -> Result<JoinHandle<MusicPlayerResult<()>>, std::io::Error> {
        let app_state_arc = self.app_state.clone();

        let music_thread = ThreadBuilder::new()
            .name("typeracer-music-thread".to_string())
            .spawn(move || -> MusicPlayerResult<()> {
                let mut mp = MusicPlayer::from_volume(0.5)?;

                mp.load_song_from_mem(PLAY_CS16_SOUND, "play")?;
                mp.load_song_from_mem(SKELER_TELATIV_SONG, "skeler")?;

                // there inside MusicPLayer I could have
                // a reference to AppState, to modifiy the Music state automatically
                // but we'll see
                mp.play_all_songs_in_order();
                {
                    if let Ok(app_state_mutex) = app_state_arc.lock() {
                        *app_state_mutex.music_state_ref_mut() =
                            MusicState::new_playing();
                    } else {
                        panic!("here")
                    }
                }

                // this loop will pause the current thread
                // cause player is running in background
                // and if this finishes, music player is done
                loop {
                    // try lock is non-blocking
                    // doesnt need to be

                    if let Ok(app_state_mutex) = app_state_arc.try_lock() {
                        let mut music_state =
                            app_state_mutex.music_state_ref_mut();

                        mp.react_to_state(&music_state);
                    }

                    std::thread::sleep(std::time::Duration::from_millis(
                        10
                    ));
                }

                Ok(())
            })?;

        Ok(music_thread)
    }

    fn calculate_wpm(&mut self) {
        // def calculate_wpm(self):
        //     try:
        //     time_diff = abs(time() - self.start_time)
        // time_diff_2 = fixed_set_precision_float(time_diff, 2)
        //
        // if time_diff_2 == 0:
        // return round(time_diff / 1000)
        //
        // time_diff = float(time_diff)
        //
        // time_diff = current_time - start_time
        // self.wpm = (
        //     60 * len(self.total_correct_typed_chars) / 5) / time_diff
        // return round(self.wpm)
        if let Ok(mut app_state_mutex) = self.app_state.lock() {
            let mut game_state = app_state_mutex.game_state_ref_mut();

            match *game_state {
                GameState::Playing => {
                    let mut time_diff =
                        app_state_mutex.elapsed_time_ref_mut();
                    let total_correct_typed_chars = app_state_mutex
                        .total_correct_typed_chars_ref_mut();

                    let wpm = (60.0 * (*total_correct_typed_chars as f64)
                        / 5.0)
                        / *time_diff;
                    let wpm = wpm as u16;

                    let mut wpm_ref_mut = app_state_mutex.wpm_ref_mut();
                    *wpm_ref_mut = Some(wpm);
                },
                GameState::Paused => {
                    // do nothing if the game hasnt started
                }
            }
        } else {
            panic!("asd")
        }
    }

    fn game_loop(mut self) -> TyperacerResult<()> {
        let sleep_ms = u64::from(self.config.sleep_ms());

        loop {
            // calculate WPM always
            self.calculate_wpm();

            // render ui
            {
                let app_state_arc = self.app_state.clone();
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
            if event::poll(Duration::from_millis(sleep_ms))? {
                let event = event::read()?;

                let loop_action = self.handle_event(event)?.1;

                // handle key particurarly
                match loop_action {
                    LoopActions::TimeToBreak => break,
                    LoopActions::TimeToContinue => continue,
                    // here we need to render one last time UI before ending the game
                    LoopActions::GameFinished => break,
                    LoopActions::Noop => {
                        // do nothing, just continues the typeracer game
                        // (not continue from programming)
                    },
                    LoopActions::ForceQuitGame => break,
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

        Ok(())
    }

    /// the main game that is only played once
    pub fn mainloop(mut self) -> TyperacerResult<()> {
        // arc pointer is in self as field
        #[cfg(feature = "music")]
            // this only exists if the music feature is ON
            let music_thread_handle = self.create_and_spawn_music_thread()?;

        let stopwatch_thread_handle =
            self.create_and_spawn_stopwatch_thread()?;

        self.game_loop()?;
        Ok(())
    }

    fn handle_key_event(
        &self,
        key_event: KeyEvent,
        app_state_mutex_ref: &MutexGuard<AppState>
    ) -> TyperacerResult<(&Self, LoopActions)> {
        let app_state = app_state_mutex_ref;
        // at first keyboard press, the game has started
        let mut game_state = app_state.game_state_ref_mut();

        #[cfg(feature = "music")]
            let mut music_state = app_state.music_state_ref_mut();

        let mut has_game_started = app_state.game_started_ref_mut();

        /// initialize the game state flag
        {
            let control_space =
                KeyEvent::new(KeyCode::Char(' '), KeyModifiers::CONTROL);
            let control_s =
                KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL);

            // if the user pressed anything other than the game keybindings for menu actions
            if key_event != control_space {
                if !*has_game_started {
                    *game_state = GameState::Playing;
                    *has_game_started = true;
                } else if *game_state == GameState::Paused {
                    *game_state = GameState::Playing;
                    *music_state = MusicState::Playing;
                }
            }
        }

        let mut game_finished = app_state.game_finished_ref_mut();
        let mut typeracer_text = app_state.typeracer_text_ref_mut();
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

        let elapsed = app_state.elapsed_time_ref_mut();
        let mut keyboard_input = app_state.keyboard_input_ref_mut();

        let mut index = app_state.index_ref_mut();
        let mut wrong_index = app_state.wrong_index_ref_mut();

        let mut total_correct_typed_chars =
            app_state.total_correct_typed_chars_ref_mut();

        let key_event_clone = format!("{:?}", key_event.code.clone());
        *keyboard_input = key_event_clone.yellow().to_string();

        match key_event {
            // this needs to be merged with enter and Char(character)
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                // its working, nice, but this is just a prototype
                if '\t' == typeracer_text.chars().nth(*index).unwrap() {
                    *index += 1;
                    *index_shadow += 1;
                }
            },
            // this will pause the game and also pause the music and the stopwatch
            KeyEvent {
                code: KeyCode::Char(' '),
                modifiers: KeyModifiers::CONTROL,
                ..
                // Release doesnt work, just Press
                // kind: KeyEventKind::Press,
                // TODO: file an issue about this
                // state: KeyEventState::NONE
            } => {
                if *has_game_started {
                    match *game_state {
                        GameState::Paused => {
                            *game_state = GameState::Playing;

                            #[cfg(feature = "music")]
                            {
                                *music_state = MusicState::Playing;
                            }
                        }
                        GameState::Playing => {
                            *game_state = GameState::Paused;

                            #[cfg(feature = "music")]
                            {
                                *music_state = MusicState::Paused;
                            }
                        },
                    }
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

                // TODO: recomment this
                // // ui logic
                let time_to_continue = self.handle_enter_key(
                    &mut what_was_typed,
                    &mut user_input_prompt,
                    *user_input_prompt_x)?;

                // if enter is pressed and the prompt is empty
                // just continue the loop
                if time_to_continue {
                    return Ok((self, LoopActions::TimeToContinue))
                }

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
                    if '\n' == typeracer_text.chars().nth(current_index).ok_or_else(|| { TyperacerErrors::IndexOutOfBounds(index_error.clone()) })? {
                        return Ok((self, LoopActions::TimeToContinue))
                    }
                    // dbg!("herer");
                    *wrong_index += 1;
                    *wrong_index_shadow += 1;
                }

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
                return Ok((self, LoopActions::ForceQuitGame))
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
                    if '\n' == typeracer_text.chars().nth(current_index).ok_or_else(|| {
                        TyperacerErrors::IndexOutOfBounds(index_error.clone())
                    })?
                    {
                        return Ok((self, LoopActions::TimeToContinue))
                    }
                }

                // you cannot go back if all the text is green behind the cursor
                if *wrong_index == 0 {
                    return Ok((self, LoopActions::TimeToContinue))
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
            #[cfg(feature = "music")]
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
                    .ok_or_else(|| {
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

                    *total_correct_typed_chars += 1;
                } else if *index + *wrong_index < typeracer_text.len() {
                    *wrong_index += 1;
                    *wrong_index_shadow += 1;
                }
            },
            _ => {},
        }
        *cursor_y = *index_shadow + *wrong_index_shadow;

        Ok((self, LoopActions::Noop))
    }

    fn handle_event(
        &mut self,
        event: Event
    ) -> TyperacerResult<(&Self, LoopActions)> {
        let app_state = match self.app_state.lock() {
            Ok(app_state) => app_state,
            Err(error) => return Err(TyperacerErrors::PoisonError)
        };

        match event {
            Event::FocusGained => {
                todo!("do something if terminal focus is gained")
            },
            Event::FocusLost => {
                todo!("do something if terminal focus is LOST")
            },
            Event::Paste(string_from_ctrl_v) => {},
            Event::Resize(width, height) => {
                // the opposite compared to the one from mathematical graphs
                // y is always the width (total cols in the term)
                // which in math should be the `x`
                // same as `tput cols`
                let y = width as u16;
                // x is always the height (total rows in the term)
                // which in math should be the `y`
                // same as `tput lines`
                let x = height as u16;
                self.ui.set_term_height(x);
                self.ui.set_term_width(y);
            },
            Event::Mouse(mevent) => {
                // dbg!(mevent);
                let mouse_kind = mevent.kind;
                match mouse_kind {
                    MouseEventKind::Down(MouseButton::Right) => {},
                    MouseEventKind::Up(MouseButton::Right) => {},
                    _ => {}
                }
            },
            Event::Key(kevent) => {
                let loop_actions =
                    self.handle_key_event(kevent, &app_state)?.1;
                match loop_actions {
                    LoopActions::Noop => {
                        // do nothing if everything is from `Self::handle_key_event`
                    },
                    _ => return Ok((self, loop_actions))
                }
            },
            _ => {}
        }

        Ok((self, LoopActions::Noop))
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
        let app_state_arc = self.app_state.clone();

        let stopwatch_thread = ThreadBuilder::new()
            .name("stopwatch-thread".to_string())
            .spawn(move || {
                // this comment helps protect this from rustfmt to
                // make a single block loop: `move || loop {}`
                // i really dont like this, to reformat everytime
                // what if i want to put some lines before the loop
                loop {
                    if let Ok(mut app_state_mutex) = app_state_arc.lock() {
                        let mut game_state =
                            app_state_mutex.game_state_ref_mut();

                        match *game_state {
                            GameState::Paused => {},
                            GameState::Playing => {
                                let mut elapsed =
                                    app_state_mutex.elapsed_time_ref_mut();
                                *elapsed += 0.01;
                            }
                        }
                    }

                    std::thread::sleep(
                        std::time::Duration::from_secs_f32(0.0095)
                    );
                }
            })?;

        Ok(stopwatch_thread)
    }
}
