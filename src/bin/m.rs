use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
};
use crossterm::{
    event,
    terminal,
};
use std::time::Duration; // add this line

use termion::color::Color;
use termion::event::Key;
use termion::event::MouseEvent;
use termion::input::TermRead;
use termion::input::MouseTerminal;

use termion::cursor::{
    self,
    DetectCursorPos,
};

use termion::event::*;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use core_dev::datetime::datetime::get_current_datetime;

use std::io::Stdout;
use std::io::{
    stdin,
    stdout,
    Stdin,
    Write,
};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

use ansi_term::{
    Style,
    Colour,
    Color::{
        Red,
        Green,
    },
};

use core_dev::traits::StringExtended;


use rand::thread_rng;
use rand::Rng;
// ╭ ─ ╮
// │ │
// │ │
// ╰ ─ ╯

fn get_text_colored(
    text: &str,
    index: usize,
    wrong_index: usize,
) -> String {
    let green = Green.paint(&text[..index]).to_string().replace(" ", "_");
    let red = Red
        .paint(&text[index..index + wrong_index])
        .to_string()
        .replace(" ", "_");
    let rest = &text[index + wrong_index..];
    green + &red + rest
}

fn term_print(stdout: &mut Stdout, text: &str, x: usize, y: usize) {
    write!(
        stdout,
        "{}{}{}{}",
        // Clear the screen.
        termion::cursor::Goto(y as u16, x as u16),
        termion::clear::CurrentLine,
        text,
        // Hide the cursor.
        termion::cursor::Hide,
    )
    .unwrap();
    stdout.flush().unwrap();
}

fn print_stats(
    mut stdout: &mut Stdout,
    text: &str,
    index: usize,
    wrong_index: usize,
    display_index: usize,
    skip: bool,
    character: char,
) {
    term_print(
        &mut stdout,
        &get_text_colored(&text, index, wrong_index),
        display_index,
        1,
    );
    if !skip {
        term_print(
            &mut stdout,
            "^",
            display_index + 1,
            index + wrong_index + 1,
        );
        term_print(
            &mut stdout,
            "│",
            display_index + 2,
            index + wrong_index + 1,
        );
    }

    term_print(
        &mut stdout,
        &format!("Keyboard input: '{}'", character),
        display_index + 3,
        1,
    );
    term_print(
        &mut stdout,
        format!("Index: {index}").as_str(),
        display_index + 4,
        1,
    );
    term_print(
        &mut stdout,
        format!("Wrong: {wrong_index}").as_str(),
        display_index + 5,
        1,
    );
    term_print(
        &mut stdout,
        format!("Index + Wrong: {}", index + wrong_index).as_str(),
        display_index + 6,
        1,
    );
    term_print(
        &mut stdout,
        format!("text len: {}", text.len()).as_str(),
        display_index + 7,
        1,
    );
    term_print(
        &mut stdout,
        format!("text len-1: {}", text.len() - 1).as_str(),
        display_index + 8,
        1,
    );
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?; // modify

    let mut stdout = std::io::stdout();
    write!(
        stdout,
        "{}{}{}",
        // Clear the screen.
        termion::clear::All,
        // Goto (1,1).
        termion::cursor::Goto(1, 1),
        // Hide the cursor.
        termion::cursor::Show
    )
    .unwrap();

    let (w, h) = termion::terminal_size().unwrap();

    let mut index = 0;
    let mut wrong_index = 0;

    let display_index = 5;

    let text = String::from(
        "rust is the best language ever and the hardest",
    );
    // let text = String::from("hello");
    term_print(&mut stdout, &text, display_index, 1);
    term_print(&mut stdout, "^", display_index + 1, 1);
    term_print(&mut stdout, "│", display_index + 2, 1);

    term_print(
        &mut stdout,
        format!("Index: {index}").as_str(),
        display_index + 4,
        1,
    );
    term_print(
        &mut stdout,
        format!("Wrong: {wrong_index}").as_str(),
        display_index + 5,
        1,
    );
    term_print(
        &mut stdout,
        format!("Index + Wrong: {}", index + wrong_index).as_str(),
        display_index + 6,
        1,
    );
    let mut skip = false;
    term_print(
        &mut stdout,
        format!("text len: {}", text.len()).as_str(),
        display_index + 7,
        1,
    );
    term_print(
        &mut stdout,
        format!("text len-1: {}", text.len() - 1).as_str(),
        display_index + 8,
        1,
    );

    let mut time_to_break = false;

    let mut last_char = ' ';
    loop {
        write!(
            stdout,
            "{}{}",
            // Clear the screen.
            // Goto (1,1).
            termion::cursor::Goto(
                index as u16 + wrong_index as u16,
                display_index as u16,
            ),
            // Hide the cursor.
            termion::cursor::Show
        )
        .unwrap();
        // let random_number = thread_rng().gen_range(10..10000);
        if time_to_break {
            print_stats(
                &mut stdout,
                &text,
                index,
                wrong_index,
                display_index,
                skip,
                last_char,
            );
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


        if event::poll(Duration::from_millis(100))?
        // modify
        {
            if let Event::Key(event) = event::read()?
            // modify
            {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: event::KeyModifiers::CONTROL,
                    } => {
                        time_to_break = true;
                        continue;
                    },
                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: event::KeyModifiers::NONE,
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
                    } => {
                        last_char = character;
                        skip = false;

                        term_print(
                            &mut stdout,
                            character.to_string().as_str(),
                            25,
                            1,
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
                            }
                        }
                        term_print(
                            &mut stdout,
                            character.to_string().as_str(),
                            5,
                            1,
                        );
                    },
                    _ => {},
                }
            };
        } else {
            // if you dont print anything here
            // the app will freeze
            println!("");
        }

        print_stats(
            &mut stdout,
            &text,
            index,
            wrong_index,
            display_index,
            skip,
            last_char,
        );
    }
    Ok(())
}
