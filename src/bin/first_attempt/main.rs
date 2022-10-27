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
#![allow(clippy::all)]

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

    if text.contains("\n") {
        for pair in text.split("\n").enumerate() {
            let (index, line) = pair;
            let index = index as u16;

            let (move_to_y_x, clear_current_line, hide_cursor) = (
                crossterm::cursor::MoveTo(y, x + index),
                crossterm::terminal::Clear(
                    crossterm::terminal::ClearType::CurrentLine
                ),
                crossterm::cursor::Show
            );
            write!(
                stdout,
                "{}{}",
                // you need to hide the cursor
                // otherwise it will appear alone without any invocations
                move_to_y_x,
                // clear_current_line,
                line,
                // hide_cursor
            )
            .unwrap();
        }
    } else {
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
            "{}{}",
            // you need to hide the cursor
            // otherwise it will appear alone without any invocations
            move_to_y_x,
            // clear_current_line,
            text,
            // hide_cursor
        )
        .unwrap();
    }
}

fn print_stats(
    mut stdout: &mut Stdout,
    text: &str,
    index: usize,
    wrong_index: usize,
    display_index: usize,
    skip: bool,
    character: char,
    cursor_x: usize,
    cursor_y: usize
) {
    term_print(
        &mut stdout,
        &get_text_colored(&text, index, wrong_index),
        display_index,
        1
    );

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
    let cursor_x = cursor_x as u16;
    let cursor_y = cursor_y as u16;
    let move_to = cursor::MoveTo(cursor_y, cursor_x);
    execute!(stdout, move_to).unwrap();
}

fn main() -> CrosstermResult<()> {
    let mut stdout = std::io::stdout();
    enter_raw_terminal(&mut stdout)?;

    std::panic::set_hook(Box::new(move |panic_info| {
        // if i dont manually bring back the cursor here,
        // the cursor wont come back
        let mut stdout = std::io::stdout();
        let _ = write!(stdout, "{}", crossterm::cursor::Show);

        // ? operator is converting from an error to another
        let _ = execute!(stdout, LeaveAlternateScreen);
        let _ = execute!(stdout, DisableMouseCapture);
        let _ = terminal::disable_raw_mode();

        eprintln!();
        eprintln!("program panicked with this:");
        eprintln!("{panic_info:#?}");
    }));

    let (w, h) = terminal::size().unwrap();

    let mut index = 0;
    let mut wrong_index = 0;

    let display_index = 5;

    let text = String::from("first line\nsecond line\nthird line");
    // let text = String::from("hello");
    term_print(&mut stdout, &text, display_index, 1);

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
    let mut cursor_x = display_index - 1;
    // let y = index + wrong_index;

    let mut index_shadow = index;
    let mut wrong_index_shadow = wrong_index;

    let mut cursor_y = index_shadow + wrong_index_shadow;
    let total_lines = text.split("\n").collect::<Vec<_>>().len();
    let mut current_line = 0usize;
    'mainloop: loop {
        cursor_y = index_shadow + wrong_index_shadow;

        print_stats(
            &mut stdout,
            &text,
            index,
            wrong_index,
            display_index,
            skip,
            last_char,
            cursor_x,
            cursor_y
        );

        if time_to_break {
            break;
        }

        if event::poll(Duration::from_millis(100))? {
            let e = event::read()?;
            if let Event::Key(kevent) = e {
                match kevent {
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: event::KeyModifiers::CONTROL,
                        ..
                    } => {
                        time_to_break = true;
                        continue 'mainloop;
                    },
                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        if 0 < current_line {
                            let current_index = index + wrong_index - 1;
                            if text.chars().nth(current_index).unwrap()
                                == '\n'
                            {
                                dbg!("da");
                                continue 'mainloop;
                            }
                        }

                        if wrong_index > 0 {
                            wrong_index -= 1;
                            wrong_index_shadow -= 1;
                        } else {
                            if index > 0 {
                                index -= 1;
                                if index_shadow > 0 {
                                    index_shadow -= 1;
                                }
                            }
                        }
                    },
                    KeyEvent {
                        code,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => match code {
                        KeyCode::Char(character) => {
                            if wrong_index > 0 {
                                let current_index = index + wrong_index;
                                if text.chars().nth(current_index).unwrap()
                                    == '\n'
                                {
                                    continue 'mainloop;
                                }
                            }
                            last_char = character;

                            term_print(
                                &mut stdout,
                                character.to_string().as_str(),
                                25,
                                1
                            );

                            if index == text.len() - 1 {
                                index += 1;
                                index_shadow += 1;
                                time_to_break = true;
                                continue;
                            }
                            if character == text.get_char(index).unwrap()
                                && wrong_index == 0
                            {
                                index += 1;
                                index_shadow += 1;
                            } else {
                                if index + wrong_index < text.len() {
                                    wrong_index += 1;
                                    wrong_index_shadow += 1;
                                }
                            }
                        },
                        KeyCode::Enter => {
                            last_char = '\n';
                            cursor_x += 1;
                            current_line += 1;

                            if index == text.len() - 1 {
                                index += 1;
                                time_to_break = true;
                                continue;
                            }
                            if '\n' == text.get_char(index).unwrap()
                                && wrong_index == 0
                            {
                                index_shadow = 0;
                                wrong_index_shadow = 0;
                                index += 1;
                            } else {
                                if index + wrong_index < text.len() {
                                    wrong_index += 1;
                                }
                            }
                        },
                        _ => {}
                    },
                    _ => {}
                }
            } // end of match event
              // end of pol
        }
    }

    leave_raw_terminal(&mut stdout)
}

pub fn enter_raw_terminal(stdout: &mut Stdout) -> CrosstermResult<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    execute!(stdout, crossterm::cursor::Show)?;

    Ok(())
}

pub fn leave_raw_terminal(stdout: &mut Stdout) -> CrosstermResult<()> {
    // if i dont manually bring back the cursor here,
    // the cursor wont come back
    write!(stdout, "{}", crossterm::cursor::Show).unwrap();

    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
