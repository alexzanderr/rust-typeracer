use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
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

use crate::terminal_screen::TerminalScreen;
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

    pub fn draw_input_bar(
        &mut self,
        user_input: &str,
        x: usize,
        y: usize
    ) -> TyperacerResult<&mut Self> {
        let input_icon = "❱".yellow().bold();
        let text = format!("{input_icon}  {user_input}");
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

    pub fn mainloop(mut self) -> TyperacerResult<()> {
        let input_bar_x = self.term.get_height() - 3;
        self.draw_input_bar("", input_bar_x, 0)?;
        let mut user_input = String::from("");
        let mut total_spaces = 0usize;

        let mut text_area = Vec::new();

        loop {
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
                Event::Key(kevent) => match kevent {
                    KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: event::KeyModifiers::CONTROL,
                        ..
                    } => {
                        self.term
                            .print("you pressed control + enter", 10, 0)?
                            .refresh()?;
                    },
                    KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    }
                    | KeyEvent {
                        code: KeyCode::Char(' '),
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        if user_input.is_empty() {
                            continue;
                        }
                        text_area.push(user_input.clone());
                        if text_area.len() == input_bar_x - 3 {
                            text_area.clear();
                            text_area.push(user_input.clone());
                        }
                        user_input.clear();
                        self.term.clear()?;
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
                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        let _ = user_input.pop();
                    },
                    // ctrl + backspace, doesnt work, cuz terminal stuff
                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: event::KeyModifiers::ALT,
                        ..
                    } => {
                        user_input.clear();
                    },
                    KeyEvent {
                        code: KeyCode::Char(character),
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        user_input.push(character);
                        if user_input.len() == self.term.get_width() - 9 {
                            user_input.clear();
                        }
                    },

                    _ => {}
                } // end of key events
            } // end of match
            self.term
                .rectangle()
                .screens_width(true)
                .align_center(false)
                .xy(0, 0)
                .text(text_area.as_slice())
                .build()?
                .draw()?;

            self.draw_input_bar(&user_input, input_bar_x, 0)?;
            self.term.refresh()?;
        } // end of loop
        Ok(())
    }
}
