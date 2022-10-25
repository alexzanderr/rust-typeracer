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

#[derive(Debug)]
pub struct Typeracer<'a> {
    ui: TyperacerUI<'a>
}

impl<'a> Typeracer<'a> {
    pub fn from_term(term: &'a mut TerminalScreen) -> Self {
        let ui = TyperacerUI::from_term(term);
        Self {
            ui
        }
    }

    // TODO: this needs to be better
    // game logic
    pub fn handle_ctrl_backspace(
        &mut self,
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
        // let typeracer_text =
        //     "rust is the best language ever and the hardest\n\
        //      rust is the best language ever and the hardest";

        let typeracer_text_lines = "rust best asd\n\
             rust best\n\
             second one long";
        // let typeracer_text = "asd|";
        // let typeracer_text = "what | is this ?|";
        // let typeracer_text = "what|";
        let current_time = std::time::Instant::now();

        let typeracer_lines =
            typeracer_text_lines.split("\n").collect::<Vec<&str>>();

        let mut typeracer_visual_lines = typeracer_lines
            .clone()
            .into_iter()
            .map(|item| item[..5].green().to_string() + &item[5..])
            .collect::<Vec<String>>();

        // for pair in typeracer_lines.into_iter().enumerate() {
        // let (row_index, line) = pair;
        // game

        // let typeracer_text = line;
        let typeracer_text_x = 6;
        let typeracer_text =
            "rust is the best language ever and the hardest";

        let mut what_was_typed = String::from("");
        let mut what_was_typed_x = 9;

        let user_input_prompt_x = 3;

        let mut user_input_prompt = String::from("");
        let mut total_spaces = 0usize;

        let text_area_x = 6;
        let mut text_area = Vec::new();

        let mut keyboard_input = String::from("");
        let term_height = self.ui.term_height();
        let term_width = self.ui.term_width();

        let mut index = 0;
        let mut wrong_index = 0;
        let mut time_to_break = false;
        let mut game_finished = false;
        let mut cursor_x = typeracer_text_x + 1;

        loop {
            // self.ui.draw(&mut app_state)?;

            self.ui.draw(
                current_time,
                typeracer_text,
                typeracer_text_x,
                &mut what_was_typed,
                what_was_typed_x,
                &mut user_input_prompt,
                user_input_prompt_x,
                &mut keyboard_input,
                index,
                wrong_index,
                cursor_x
            )?;

            if game_finished {
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

            // let got_event = event::poll(Duration::from_millis(100))?;
            // if got_event {
            //     let e = event::read()?;
            // }

            if event::poll(Duration::from_millis(100))? {
                let e = event::read()?;

                match e {
                    Event::FocusGained => {
                        todo!("do something if terminal focus is gained")
                    },
                    Event::FocusLost => {
                        todo!("do something if terminal focus is LOST")
                    },

                    Event::Paste(string_from_ctrl_v) => {},
                    Event::Resize(y, x) => {
                        // dbg!(y, x);
                        // self.term.clear()?;
                        // self.term.refresh()?;
                    },
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
                        keyboard_input =
                            format!("{:?}", kevent.code.clone());
                        keyboard_input =
                            keyboard_input.yellow().to_string();

                        // debug the current key in a separate rectangle
                        // self.term
                        //     .rectangle()
                        //     .screens_width(true)
                        //     .align_center(false)
                        //     .xy(term_height - 8 - 3, 0)
                        //     .text(keyboard_input.as_str())
                        //     .build()?
                        //     .draw()?;

                        // self.term.refresh()?;

                        match kevent {
                                KeyEvent {
                                    code: KeyCode::Enter,
                                    modifiers: event::KeyModifiers::CONTROL,
                                    ..
                                } => {
                                },
                                // clear the entire user_input_bar
                                // and append the text to the text area
                                // enter or space into the user_input_prompt
                                KeyEvent {
                                    code: KeyCode::Enter,
                                    modifiers: event::KeyModifiers::NONE,
                                    ..
                                } => {
                                    //typeracer logic
                                    let error_span = SpanError::new(file!(), line!() + 1, column!());
                                    let index_error = IndexOutOfBoundsError::new(
                                        index,
                                        typeracer_text.to_string(),
                                        error_span
                                    );
                                    if '\n' == typeracer_text.chars().nth(index).ok_or(TyperacerErrors::IndexOutOfBounds(index_error))?
                                        && wrong_index == 0
                                    {
                                        index += 1;
                                        // cursor_x += 1;
                                        // // let move_to = cursor::
                                        // execute!(
                                        //     self.ui.term_buffer_ref_mut(),
                                        //     show_cursor,
                                        //     cursor_shape,
                                        //     cursor_blink_off
                                        // )?;
                                        if index == typeracer_text.len() {
                                            game_finished = true;
                                        }
                                    } else if index + wrong_index < typeracer_text.len() {
                                        wrong_index += 1;
                                    }


                                    // ui logic
                                    let time_to_continue = self.handle_enter_key(
                                        user_input_prompt_x,
                                        text_area_x,
                                        &mut text_area,
                                        &mut what_was_typed,
                                        &mut user_input_prompt)?;

                                    if time_to_continue {
                                        continue
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
                                } => break,
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
                                    if wrong_index > 0 {
                                        wrong_index -= 1
                                    } else {
                                        if index > 0 {
                                            index -= 1;
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

                                    // if index == typeracer_text.len() - 1 {
                                    //     // index += 1;
                                    //     // if
                                    //     time_to_break = true;
                                    //     game_finished = true;
                                    //     continue;
                                    // }
                                    // typeracer game logic
                                    // let err_msg = format!("index out of bounds;\ntyperacer_text: {typeracer_text}\nindex: {index}");
                                    let error_span = SpanError::new(file!(), line!() + 1, column!());
                                    let index_error = IndexOutOfBoundsError::new(
                                        index,
                                        typeracer_text.to_string(),
                                        error_span
                                    );
                                    if character == typeracer_text.chars().nth(index).ok_or(TyperacerErrors::IndexOutOfBounds(index_error))?
                                        && wrong_index == 0
                                    {
                                        index += 1;
                                        // if index == typeracer_text.len() {
                                        //     game_finished = true;
                                        // }
                                    } else if index + wrong_index < typeracer_text.len() {
                                        wrong_index += 1;
                                    }

                                },
                                _ => {}
                            } // end of key events
                    } // end of key events from match
                } // end of match
            }
            // end of poll
            else {
            }

            text_area.clear();
        } // end of loop
          // }

        Ok(())
    }

    fn handle_enter_key(
        &mut self,
        input_bar_x: usize,
        text_area_x: usize,
        text_area: &mut Vec<String>,
        what_was_typed: &mut String,
        user_input_prompt: &mut String
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
        &mut self,
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
