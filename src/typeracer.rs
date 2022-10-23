use std::io::Write;
use std::time::Duration;

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

pub use crate::statics::{
    PROMPT_ARROW,
    TERMINAL_CURSOR
};
use crate::terminal_screen::TerminalScreen;

#[derive(Debug)]
pub struct Stats<'a> {
    keyboard_input: &'a str,
    index:          usize,
    wrong_index:    usize,
    text_len:       usize
}

impl<'a> Stats<'a> {
    pub fn new(
        keyboard_input: &'a str,
        index: usize,
        wrong_index: usize,
        text_len: usize
    ) -> Self {
        Self {
            keyboard_input,
            index,
            wrong_index,
            text_len
        }
    }
}

impl<'a> std::fmt::Display for Stats<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        let keyboard_input = self.keyboard_input;
        let index = self.index;
        let wrong_index = self.wrong_index;
        let text_length = self.text_len;
        let text_length_minus_one = self.text_len - 1;
        let index_plus_wrong_index = self.index + self.wrong_index;

        let stats_to_string = format!(
            r#"Keyboard input: '{keyboard_input}'
Index: {index}
Wrong: {wrong_index}
Index + Wrong: {index_plus_wrong_index}
text.len(): {text_length}
text.len() - 1: {text_length_minus_one}"#
        );

        write!(f, "{stats_to_string}")
    }
}

#[derive(Debug)]
pub struct Typeracer<'a> {
    term: &'a mut TerminalScreen
}

use colored::*;

use crate::errors::TyperacerResult;

impl<'a> Typeracer<'a> {
    pub fn from_term(screen: &'a mut TerminalScreen) -> Self {
        Self {
            term: screen
        }
    }

    pub fn draw_user_input_prompt(
        &mut self,
        user_input: &str,
        x: usize,
        y: usize
    ) -> TyperacerResult<&mut Self> {
        let input_icon = PROMPT_ARROW.yellow().bold();
        // let red_cursor = "_".red().bold();
        // let red_cursor = "│".red().bold();
        let red_cursor = TERMINAL_CURSOR.red().bold();
        let text = format!("{input_icon}  {user_input}{red_cursor}");
        let text = text.as_str();
        self.term
            .rectangle()
            .text(text)
            .xy(x, y)
            .screens_width(true)
            .align_center(false)
            .build()?
            .draw()?;

        self.term.refresh()?;

        Ok(self)
    }

    // TODO: this needs to be better
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

    pub fn draw_stats(
        &mut self,
        game_stats: Stats
    ) -> TyperacerResult<()> {
        // 6 + 2 (the borders)
        let total_lines_of_stats = 8;
        let x = self.term.height() - total_lines_of_stats;

        let text = game_stats.to_string();

        self.term
            .rectangle()
            .screens_width(true)
            .align_center(false)
            .text(text)
            .xy(x, 0)
            .build()?
            .draw()?;

        self.term.refresh()?;
        Ok(())
    }

    pub fn get_text_colored(
        &self,
        text: &str,
        index: usize,
        wrong_index: usize,
        cursor_on: bool
    ) -> String {
        if cursor_on {
            // let cursor = "▏".red();
            // let cursor = "│".red();
            // let cursor = "|".red();
            // let cursor = "｜".red();
            let green =
                text[..index].green().to_string().replace(" ", "_");
            let red = text[index..index + wrong_index]
                .red()
                .to_string()
                .replace(" ", "_");
            let rest = &text[index + wrong_index..];

            let cursor = "❘".yellow();
            let cursor = "|".yellow();
            let cursor = "⏐".yellow();
            let cursor = "𑗅".yellow();
            // let cursor = "r𑗅ust".red();
            // format!("'𑗅asd'")
            format!("{green}{red}{cursor}{rest}")
        } else {
            let green =
                text[..index].green().to_string().replace(" ", "_");
            let red = text[index..index + wrong_index]
                .red()
                .to_string()
                .replace(" ", "_");
            let rest = &text[index + wrong_index..];
            format!("{green}{red}{rest}")
        }
    }

    pub fn draw_ui(
        &mut self,
        current_time: std::time::Instant,
        typeracer_text: &str,
        typeracer_text_x: usize,
        what_was_typed: &str,
        what_was_typed_x: usize,
        user_input_prompt: &mut String,
        user_input_prompt_x: usize,
        keyboard_input: &mut String,
        index: usize,
        wrong_index: usize
    ) -> TyperacerResult<()> {
        let current_date_time = get_current_datetime();
        let yellow_left_bracket = "[".yellow();
        let yellow_right_bracket = "]".yellow();
        let lb = yellow_left_bracket;
        let rb = yellow_right_bracket;

        let elapsed_time = current_time.elapsed();

        let header_x = 0usize;
        // let elapsed_repr = format!("{:.2?}", elapsed_time);
        let header = format!(
                "{lb}Date-time: {current_date_time}{rb} {lb}Elapsed-time: {elapsed_time:.2?}{rb}",
            );

        self.term
            .rectangle()
            .screens_width(true)
            // TODO: ansi parser algo doesnt work in align_center == true
            .align_center(false)
            .text(header)
            .xy(header_x, 0)
            .build()?
            .draw()?;

        // if dont refresh the terminal every time i draw something
        // i have flickering
        self.term.refresh()?;

        let typeracer_text_colored = self.get_text_colored(
            typeracer_text,
            index,
            wrong_index,
            false
        );
        // let typeracer_text_colored = format!(
        //     "{} {} {}",
        //     "hello".green(),
        //     "wrong".red(),
        //     "nortmrl teast"
        // );

        self.term
            .rectangle()
            .screens_width(true)
            .align_center(false)
            .xy(typeracer_text_x, 0)
            .text(typeracer_text_colored.as_str())
            .build()?
            .draw()?;
        self.term.refresh()?;

        self.term
            .rectangle()
            .screens_width(true)
            .align_center(false)
            .xy(what_was_typed_x, 0)
            .text(what_was_typed)
            .build()?
            .draw()?;
        self.term.refresh()?;

        // if !text_area.is_empty() {
        //     self.term
        //         .rectangle()
        //         .screens_width(true)
        //         .align_center(false)
        //         .xy(text_area_x, 0)
        //         .text(text_area.as_slice())
        //         .build()?
        //         .draw()?;
        //     self.term.refresh()?;
        // }

        self.draw_user_input_prompt(
            &user_input_prompt,
            user_input_prompt_x,
            0
        )?;
        self.term.refresh()?;

        {
            let stats = Stats::new(
                &keyboard_input,
                index,
                wrong_index,
                typeracer_text.len()
            );
            self.draw_stats(stats)?;
            self.term.refresh()?;
        }

        let x = typeracer_text_x as u16 + 1;
        let y = (index + wrong_index) as u16 + 3;
        let move_to = cursor::MoveTo(y, x);

        let show_cursor = cursor::Show;
        let cursor_shape =
            cursor::SetCursorShape(cursor::CursorShape::Line);
        let cursor_blink_off = cursor::DisableBlinking;

        // if i show the cursor is blinking really fast
        // meaning the cursor is flickering
        write!(
            self.term.stdout_ref(),
            "{}{}{}{}",
            move_to,
            show_cursor,
            cursor_shape,
            cursor_blink_off
        )?;
        self.term.refresh()?;
        // still flickering by putting this above the poll
        Ok(())
    }

    pub fn mainloop(mut self) -> TyperacerResult<()> {
        let typeracer_text_x = 6;
        let typeracer_text =
            "rust is the best language ever and the hardest";

        let mut what_was_typed = String::from("");
        let mut what_was_typed_x = 9;

        let show_cursor = cursor::Show;

        let cursor_shape =
            cursor::SetCursorShape(cursor::CursorShape::UnderScore);
        let cursor_blink_off = cursor::DisableBlinking;

        execute!(
            self.term.stdout_ref(),
            show_cursor,
            cursor_shape,
            cursor_blink_off
        )?;
        self.term.refresh()?;

        // let input_bar_x = self.term.height() - 3;
        let user_input_prompt_x = 3;
        self.draw_user_input_prompt("", user_input_prompt_x, 0)?;

        // self.term.refresh()?;

        let mut user_input_prompt = String::from("");
        let mut total_spaces = 0usize;

        let text_area_x = 6;
        let mut text_area = Vec::new();

        let current_time = std::time::Instant::now();
        let mut keyboard_input = String::from("");
        let term_height = self.term.height();
        let term_width = self.term.width();

        let mut index = 0;
        let mut wrong_index = 0;
        let mut time_to_break = false;
        let mut game_finished = false;

        // get_text_colored(&text, index, wrong_index)

        loop {
            self.draw_ui(
                current_time,
                typeracer_text,
                typeracer_text_x,
                &mut what_was_typed,
                what_was_typed_x,
                &mut user_input_prompt,
                user_input_prompt_x,
                &mut keyboard_input,
                index,
                wrong_index
            )?;

            if time_to_break {
                if game_finished {
                    self.term
                .print(
                    "Congratulations! <press any key to leave game>",
                    19,
                    0
                )?
                .refresh()?;

                    event::read()?;
                }

                break;
            }

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
                        self.term.clear()?;
                        self.term.refresh()?;
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

                        self.term.refresh()?;

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

                                if character == ' ' {
                                    user_input_prompt.clear();
                                }

                                // typracer game logic
                                if index == typeracer_text.len() - 1 {
                                    index += 1;
                                    time_to_break = true;
                                    game_finished = true;
                                    continue;
                                }
                                if character == typeracer_text.chars().nth(index).unwrap()
                                    && wrong_index == 0
                                {
                                    index += 1;
                                } else if index + wrong_index < typeracer_text.len() {
                                    wrong_index += 1;
                                }

                                self.handle_any_character(&mut what_was_typed, &mut user_input_prompt, character);
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

            self.term.refresh()?;
        } // end of loop

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
        let term_height = self.term.height();
        let term_width = self.term.width();

        if user_input_prompt.is_empty() {
            return Ok(true);
        }

        what_was_typed.push_str(&user_input_prompt);
        what_was_typed.push(' ');

        if what_was_typed.len() >= term_width - 6 {
            what_was_typed.clear();
        }

        text_area.push(user_input_prompt.clone());

        let term_height = self.term.height();
        if text_area.len() == term_height - text_area_x - 1 {
            text_area.clear();
            text_area.push(user_input_prompt.clone());

            let clear_current_line =
                terminal::Clear(terminal::ClearType::CurrentLine);

            let hide_cursor = cursor::Hide;
            for index in text_area_x..term_height {
                let move_to = cursor::MoveTo(0, index as u16);
                write!(
                    self.term.stdout_ref(),
                    "{}{}{}",
                    move_to,
                    clear_current_line,
                    hide_cursor
                )?;
            }
            self.term.refresh()?;
        }
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
        if user_input_prompt.len() == self.term.width() - 11 {
            user_input_prompt.clear();
        }
    }
}
