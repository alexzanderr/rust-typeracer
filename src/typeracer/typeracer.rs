use std::io::Write;
use std::time::Duration;

use colored::*;
use core_dev::datetime::datetime::get_current_datetime;
use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
    KeyEventState,
    KeyModifiers
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

use super::AppState;
use super::errors::{
    IndexOutOfBoundsError,
    SpanError,
    TyperacerErrors,
    TyperacerResult
};
use crate::statics::{
    PROMPT_ARROW,
    TERMINAL_CURSOR
};
use crate::terminal_screen::TerminalScreen;
use super::Stats;
use super::TyperacerUI;

pub enum LoopActions {
    TimeToBreak,
    TimeToContinue,
    GameFinished,
    LoopDoesntQuit
}

#[derive(Debug)]
pub struct Typeracer<'a> {
    ui:    TyperacerUI<'a>,
    state: AppState
}

impl<'a> Typeracer<'a> {
    pub fn from_term(term: &'a mut TerminalScreen) -> Self {
        let ui = TyperacerUI::from_term(term);
        let state = AppState::init();
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

    pub fn mainloop(mut self) -> TyperacerResult<()> {
        loop {
            // render ui
            self.ui.draw_from_app_state(&self.state)?;

            if *self.state.game_finished_ref_mut() {
                self.ui
                    .print(
                        "Congratulations! <press any key to leave game>",
                        19,
                        0
                    )?
                    .flush_stdout()?;

                event::read()?;
                break;
            }

            // handle keyboard input
            if event::poll(Duration::from_millis(100))? {
                let event = event::read()?;

                // handle key particurarly
                let loop_action = self.handle_event(event, &self.state)?.1;
                match loop_action {
                    LoopActions::TimeToBreak => break,
                    LoopActions::TimeToContinue => continue,
                    LoopActions::GameFinished => continue,
                    LoopActions::LoopDoesntQuit => {
                        // do nothing, just continues (not continue from programming)
                    }
                }

                // logging
                let app_state = format!("{:#?}", &self.state);
                let mut handler = std::fs::File::options()
                    // .append(true)
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open("log-from-loop.text")?;
                write!(handler, "{}\n\n", app_state)?;
            }
        }

        Ok(())
    }

    fn handle_event(
        &self,
        event: Event,
        app_state: &AppState
    ) -> TyperacerResult<(&Self, LoopActions)> {
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
                    event::MouseEventKind::Down(
                        event::MouseButton::Right
                    ) => {},
                    _ => {}
                }
            },
            Event::Key(kevent) => {
                let event_clone = format!("{:?}", kevent.code.clone());
                *keyboard_input = event_clone.yellow().to_string();

                match kevent {
                    // clear the entire user_input_bar
                    // and append the text to the text area
                    // enter or space into the user_input_prompt
                    KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        // return Ok((self, LoopActions::TimeToContinue));
                        //typeracer logic

                        let error_span = SpanError::new(file!(), line!() + 1, column!());
                        let index_error = IndexOutOfBoundsError::new(
                            index.clone(),
                            typeracer_text.to_string(),
                            error_span
                        );
                        if '\n' == typeracer_text.chars().nth(*index).ok_or(TyperacerErrors::IndexOutOfBounds(index_error))?
                            && *wrong_index == 0
                        {
                            *index += 1;
                            // cursor_x += 1;
                            // // let move_to = cursor::
                            // execute!(
                            //     self.ui.term_buffer_ref_mut(),
                            //     show_cursor,
                            //     cursor_shape,
                            //     cursor_blink_off
                            // )?;
                            if *index == typeracer_text.len() {
                                *game_finished = true;
                            }
                        } else if *index + *wrong_index < typeracer_text.len() {
                            *wrong_index += 1;
                        }

                        // // ui logic
                        let time_to_continue = self.handle_enter_key(
                            &mut what_was_typed,
                            &mut user_input_prompt,
                            user_input_prompt_x.clone())?;

                        if time_to_continue {
                            return Ok((self, LoopActions::TimeToContinue))
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
                        return Ok((self, LoopActions::TimeToBreak))
                    },
                    // backspace
                    // delete one char backward
                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {

                        // ui logic
                        let _ = user_input_prompt.pop();

                        // logic for the typeracer game
                        if *wrong_index > 0 {
                            *wrong_index -= 1
                        } else {
                            if *index > 0 {
                                *index -= 1;
                            }
                        }
                    },
                    // ctrl + backspace, doesnt work, cuz terminal stuff, i am guessing
                    // but ctrl + h works, cuz linux
                    //
                    // delete the entire word backwards
                    KeyEvent {
                        code: KeyCode::Char('h'),
                        modifiers: event::KeyModifiers::CONTROL,
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
                    // user pressed a char key on keyboard
                    // append it to the prompt
                    KeyEvent {
                        code: KeyCode::Char(character),
                        modifiers: event::KeyModifiers::NONE | event::KeyModifiers::SHIFT,
                        ..
                    } => {

                        self.handle_any_character(&mut what_was_typed, &mut user_input_prompt, character);

                        if character == ' ' {
                            what_was_typed.push_str(&user_input_prompt);
                            // what_was_typed.push(' ');
                            if what_was_typed.len() >= term_width - 6 {
                                what_was_typed.clear();
                            }
                            user_input_prompt.clear();
                        }

                        if *index == typeracer_text.len() - 1 {
                            *index += 1;
                            *game_finished = true;
                            return Ok((self, LoopActions::GameFinished))
                        }

                        // typeracer game logic

                        let error_span = SpanError::new(file!(), line!() + 1, column!());
                        let index_error = IndexOutOfBoundsError::new(
                            index.clone(),
                            typeracer_text.to_string(),
                            error_span
                        );
                        if character == typeracer_text.chars().nth(*index).ok_or(TyperacerErrors::IndexOutOfBounds(index_error))?
                            && *wrong_index == 0
                        {
                            *index += 1;
                            if *index == typeracer_text.len() {
                                *game_finished = true;
                            }
                        } else if *index + *wrong_index < typeracer_text.len() {
                            *wrong_index += 1;
                        }
                    },
                    _ => {},
                }
            },
            _ => {}
        }

        // logger
        let app_state = format!("{app_state:#?}");
        let mut handler = std::fs::File::options()
            // .append(true)
            .create(true)
            .truncate(true)
            .write(true)
            .open("log-method.text")?;
        write!(handler, "{}\n\n", app_state)?;

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

        what_was_typed.push_str(&user_input_prompt);
        what_was_typed.push(' ');

        if what_was_typed.len() >= term_width - 6 {
            what_was_typed.clear();
        }

        // text_area.push(user_input_prompt.clone());

        // if text_area.len() == term_height - text_area_x - 1 {
        //     text_area.clear();
        //     text_area.push(user_input_prompt.clone());

        //     let clear_current_line =
        //         terminal::Clear(terminal::ClearType::CurrentLine);

        //     let hide_cursor = cursor::Hide;
        //     for index in text_area_x..term_height {
        //         let move_to = cursor::MoveTo(0, index as u16);
        //         write!(
        //             self.term.stdout_ref(),
        //             "{}{}{}",
        //             move_to,
        //             clear_current_line,
        //             hide_cursor
        //         )?;
        //     }
        //     self.term.refresh()?;
        // }
        user_input_prompt.clear();

        // maybe this one makes flickering
        // era de la clear problema cu flickering
        // self.term.clear()?;
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
}
