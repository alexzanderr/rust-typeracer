#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use,
    non_upper_case_globals,
    non_camel_case_types,
    semicolon_in_expressions_from_macros,
    redundant_semicolons,
    unused_macros
)]

use std::time::Duration;
use std::io::Stdout;
use std::io::{
    stdin,
    stdout,
    Stdin,
    Write
};

use crossterm::event::{
    Event::{
        self,
        *
    },
    KeyCode,
    KeyEvent
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
use core_dev::datetime::datetime::get_current_datetime;
use ansi_term::{
    Color::{
        Green,
        Red
    },
    Colour,
    Style
};
use colored::*;
use core_dev::traits::StringExtended;
use rand::thread_rng;
use rand::Rng;

fn get_text_colored(
    text: &str,
    index: usize,
    wrong_index: usize
) -> String {
    let green = Green.paint(&text[..index]).to_string().replace(" ", "_");
    let red = Red
        .paint(&text[index..index + wrong_index])
        .to_string()
        .replace(" ", "_");
    let rest = &text[index + wrong_index..];
    green + &red + rest
}

fn term_print(
    stdout: &mut Stdout,
    text: &str,
    x: usize,
    y: usize
) {
    let x = x as u16;
    let x = if x > 0 { x - 1 } else { x };
    let y = y as u16;
    let y = if y > 0 { y - 1 } else { y };

    let (move_to_y_x, clear_current_line, hide_cursor) = (
        crossterm::cursor::MoveTo(y, x),
        crossterm::terminal::Clear(
            crossterm::terminal::ClearType::CurrentLine
        ),
        crossterm::cursor::Hide
    );
    // doesnt work with text, it must be a crossterm Command
    // execute!(&mut stdout, move_to, clear, text).unwrap();
    write!(
        stdout,
        "{}{}{}{}",
        // you need to hide the cursor
        // otherwise it will appear alone without any invocations
        move_to_y_x,
        clear_current_line,
        text,
        hide_cursor
    )
    .unwrap();
}

fn print_stats(
    mut stdout: &mut Stdout,
    text: &str,
    index: usize,
    wrong_index: usize,
    display_index: usize,
    skip: bool,
    character: char
) {
    term_print(
        &mut stdout,
        &get_text_colored(&text, index, wrong_index),
        display_index,
        1
    );
    if !skip {
        term_print(
            &mut stdout,
            "^",
            display_index + 1,
            index + wrong_index + 1
        );
        term_print(
            &mut stdout,
            "│",
            display_index + 2,
            index + wrong_index + 1
        );
    }

    term_print(
        &mut stdout,
        &format!("Keyboard input: '{}'", character),
        display_index + 3,
        1
    );
    term_print(
        &mut stdout,
        format!("Index: {index}").as_str(),
        display_index + 4,
        1
    );
    term_print(
        &mut stdout,
        format!("Wrong: {wrong_index}").as_str(),
        display_index + 5,
        1
    );
    term_print(
        &mut stdout,
        format!("Index + Wrong: {}", index + wrong_index).as_str(),
        display_index + 6,
        1
    );
    term_print(
        &mut stdout,
        format!("text len: {}", text.len()).as_str(),
        display_index + 7,
        1
    );
    term_print(
        &mut stdout,
        format!("text len-1: {}", text.len() - 1).as_str(),
        display_index + 8,
        1
    );
}

fn main() -> CrosstermResult<()> {
    let mut stdout = std::io::stdout();
    enter_raw_terminal(&mut stdout)?;

    let (w, h) = terminal::size().unwrap();

    let mut index = 0;
    let mut wrong_index = 0;

    let display_index = 5;

    let text =
        String::from("rust is the best language ever and the hardest");
    // let text = String::from("hello");
    term_print(&mut stdout, &text, display_index, 1);
    term_print(&mut stdout, "^", display_index + 1, 1);
    term_print(&mut stdout, "│", display_index + 2, 1);

    term_print(
        &mut stdout,
        format!("Index: {index}").as_str(),
        display_index + 4,
        1
    );
    term_print(
        &mut stdout,
        format!("Wrong: {wrong_index}").as_str(),
        display_index + 5,
        1
    );
    term_print(
        &mut stdout,
        format!("Index + Wrong: {}", index + wrong_index).as_str(),
        display_index + 6,
        1
    );
    let mut skip = false;
    term_print(
        &mut stdout,
        format!("text len: {}", text.len()).as_str(),
        display_index + 7,
        1
    );
    term_print(
        &mut stdout,
        format!("text len-1: {}", text.len() - 1).as_str(),
        display_index + 8,
        1
    );

    let mut time_to_break = false;

    let mut last_char = ' ';
    let mut total_right_clicks = 0usize;
    loop {
        print_stats(
            &mut stdout,
            &text,
            index,
            wrong_index,
            display_index,
            skip,
            last_char
        );

        if time_to_break {
            break;
        }

        let random_string =
            "Datetime: ".to_string() + &get_current_datetime();
        let header = format!("╭{}╮", "─".repeat(w as usize - 2))
            + &format!(
                "│{}{}│",
                random_string,
                " ".repeat(w as usize - 2 - random_string.len())
            )
            + &format!("╰{}╯", "─".repeat(w as usize - 2));
        term_print(&mut stdout, &header, 1, 1);

        if event::poll(Duration::from_millis(100))? {
            let e = event::read()?;
            match e {
                FocusGained | FocusLost | Paste(_) => todo!(),
                Event::Resize(y, x) => {
                    // dbg!(y, x);
                },
                Event::Mouse(mevent) => {
                    // dbg!(mevent);
                    let mouse_kind = mevent.kind;
                    match mouse_kind {
                        event::MouseEventKind::Down(
                            event::MouseButton::Right
                        ) => {
                            total_right_clicks += 1;
                            let msg = format!(
                                "you just pressed right click: {}",
                                total_right_clicks
                            );
                            term_print(&mut stdout, &msg, 20, 0);
                        },
                        _ => {}
                    }
                },
                Event::Key(kevent) => {
                    match kevent {
                        KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: event::KeyModifiers::CONTROL,
                            ..
                        } => {
                            time_to_break = true;
                            // so its prints on alternate screen
                            // sure, cuz stdout its on alternate screen
                            // so println! prints on stdout, ofc it works
                            println!("time to break");
                            continue;
                        },
                        KeyEvent {
                            code: KeyCode::Backspace,
                            modifiers: event::KeyModifiers::NONE,
                            ..
                        } => {
                            skip = false;
                            if wrong_index > 0 {
                                wrong_index -= 1
                            } else {
                                if index > 0 {
                                    index -= 1;
                                }
                            }
                        },
                        KeyEvent {
                            code: KeyCode::Char(character),
                            modifiers: event::KeyModifiers::NONE,
                            ..
                        } => {
                            skip = false;
                            last_char = character;

                            term_print(
                                &mut stdout,
                                character.to_string().as_str(),
                                25,
                                1
                            );

                            if index == text.len() - 1 {
                                index += 1;
                                time_to_break = true;
                                continue;
                            }
                            if character == text.get_char(index).unwrap()
                                && wrong_index == 0
                            {
                                index += 1;
                            } else {
                                if index + wrong_index < text.len() {
                                    wrong_index += 1;
                                } else {
                                    let x = 6;
                                    let y = index + wrong_index + 3;
                                    term_print(&mut stdout, "stop!", x, y);
                                    skip = true;
                                }
                            }
                            term_print(
                                &mut stdout,
                                character.to_string().as_str(),
                                5,
                                1
                            );
                        },
                        _ => {}
                    } // end of match key event
                } // end of Event
            } // end of match event
        } // end of poll
    }

    leave_raw_terminal(&mut stdout)
}

pub fn enter_raw_terminal(stdout: &mut Stdout) -> CrosstermResult<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
}

pub fn leave_raw_terminal(stdout: &mut Stdout) -> CrosstermResult<()> {
    // if i dont manually bring back the cursor here,
    // the cursor wont come back
    write!(stdout, "{}", crossterm::cursor::Show).unwrap();

    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    terminal::disable_raw_mode()
}
