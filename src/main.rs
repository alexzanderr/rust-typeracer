#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use
)]
use termion::color::Color;
use termion::event::Key;
use termion::event::Event;
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

use std::io::Stdout;
use std::io::{
    stdin,
    stdout,
    Stdin,
    Write,
};
use std::process::Command;

fn init_term() -> (Stdin, MouseTerminal<RawTerminal<Stdout>>) {
    let mut stdin = stdin();
    let mut stdout =
        MouseTerminal::from(stdout().into_raw_mode().unwrap());
    write!(
        stdout,
        "{}{}{}",
        // Clear the screen.
        termion::clear::All,
        // Goto (1,1).
        termion::cursor::Goto(1, 1),
        // Hide the cursor.
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
    (stdin, stdout)
}

type TERM = MouseTerminal<RawTerminal<Stdout>>;

fn end_term(mut stdout: MouseTerminal<RawTerminal<Stdout>>) {
    write!(
        stdout,
        "{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All,
        termion::cursor::Show,
    )
    .unwrap();
    stdout.flush().unwrap();
}

use ansi_term::{
    Style,
    Colour,
    Color::{
        Red,
        Green
    }
};

use core_dev::traits::StringExtended;

fn term_print(stdout: &mut TERM, text: &str, x: usize, y: usize) {
    write!(
        stdout,
        "{}{}{}{}",
        // Clear the screen.
        termion::cursor::Goto(y as u16, x as u16),
        termion::clear::CurrentLine,
        text,
        // Hide the cursor.
        termion::cursor::Hide,
    ).unwrap();
    stdout.flush().unwrap();
}

fn get_text_colored(text: &str, index: usize, wrong_index: usize) -> String {
    let green = Green.paint(&text[..index]).to_string();
    let red =
        Red.paint(&text[index..index + wrong_index]).to_string();
    let rest = &text[index + wrong_index..];
    green + &red + rest
}

fn main() {
    let (mut stdin, mut stdout) = init_term();


    let text = String::from("hello from ansi term, its andrew from another world.");
    // let text = String::from("hello");
    term_print(&mut stdout, &text, 2, 1);
    term_print(&mut stdout, "^", 3, 1);
    term_print(&mut stdout, "│", 4, 1);

    let mut index = 0;
    let mut wrong_index = 0;

    let mut skip = false;
    for c in stdin.events() {
        let e = c.unwrap();
        match e {
            Event::Key(Key::Ctrl('c')) => {
                break
            }
            Event::Key(Key::Backspace) => {
                skip = false;
                if wrong_index > 0 {
                    wrong_index -= 1
                } else {
                    if index > 0 {
                        index -= 1;
                    }
                }
            }
            Event::Key(Key::Char(character)) => {
                skip = false;
                term_print(&mut stdout, character.to_string().as_str(), 10, 1);

                if index == text.len() - 1 {
                    index += 1;
                    break
                }
                if character == text.get_char(index).unwrap() && wrong_index == 0 {
                    index += 1;
                } else {
                    if index + wrong_index < text.len() - 10 {
                        wrong_index += 1;
                    } else {
                        write!(
                            stdout,
                            "{}{}{}{}",
                            // Clear the screen.
                            termion::cursor::Goto(index as u16 + wrong_index as u16 + 3, 4),
                            termion::clear::AfterCursor,
                            "you cant go further, sorry",
                            // Hide the cursor.
                            termion::cursor::Hide,
                        ).unwrap();
                        stdout.flush().unwrap();
                        // term_print(&mut stdout, , 4, index + wrong_index + 3);
                        skip = true;
                    }
                }
                term_print(&mut stdout, character.to_string().as_str(), 5, 1);

            }
            _ => {}
        }
        term_print(&mut stdout, format!("Index: {index}").as_str(), 6, 1);
        term_print(&mut stdout, format!("Wrong: {wrong_index}").as_str(), 7, 1);
        term_print(&mut stdout, format!("Index + Wrong: {}", index + wrong_index).as_str(), 8, 1);
        term_print(&mut stdout, &get_text_colored(&text, index, wrong_index), 2, 1);
        if !skip {
            term_print(&mut stdout, "^", 3, index + wrong_index + 1);
            term_print(&mut stdout, "│", 4, index + wrong_index + 1);
        }
        term_print(&mut stdout, format!("text len: {}", text.len()).as_str(), 9, 1);
        term_print(&mut stdout, format!("text len-1: {}", text.len() - 1).as_str(), 10, 1);
    }
    term_print(&mut stdout, &get_text_colored(&text, index, wrong_index), 2, 1);
    // for c in stdin.events() {
    //     let e = c.unwrap();
    //     match e {
    //         Event::Key(Key::Char('a')) => {
    //             write!(
    //                 stdout,
    //                 "{}{}{}",
    //                 // Clear the screen.
    //                 // termion::clear::All,
    //                 termion::cursor::Goto(1, 10),
    //                 "its me mario",
    //                 // Goto (1,1).
    //                 // Hide the cursor.
    //                 termion::cursor::Hide,
    //             )
    //             .unwrap();
    //             stdout.flush().unwrap();
    //         },
    //         Event::Key(Key::Char('b')) => {
    //             write!(
    //                 stdout,
    //                 "{}{}{}",
    //                 // Clear the screen.
    //                 // termion::clear::All,
    //                 termion::cursor::Goto(1, 20),
    //                 "its working",
    //                 // Goto (1,1).
    //                 // Hide the cursor.
    //                 termion::cursor::Hide,
    //             )
    //             .unwrap();
    //             stdout.flush().unwrap();
    //         },
    //         Event::Key(Key::Char('c')) => {
    //             write!(
    //                 stdout,
    //                 "{}{}{}",
    //                 // Clear the screen.
    //                 termion::clear::All,
    //                 termion::cursor::Goto(1, 1),
    //                 // Goto (1,1).
    //                 // Hide the cursor.
    //                 termion::cursor::Hide,
    //             )
    //             .unwrap();
    //             stdout.flush().unwrap();
    //         },
    //         Event::Key(Key::Char(character)) => {
    //             println!("{}", character);
    //             break
    //         },
    //         Event::Key(Key::Ctrl('c')) => break,

    //         Event::Mouse(m) => {
    //             // println!("asd");
    //             match m {

    //             MouseEvent::Press(_, a, b)
    //             | MouseEvent::Release(a, b)
    //             | MouseEvent::Hold(a, b) => {
    //                 write!(stdout, "{}", termion::cursor::Goto(a, b)).unwrap();
    //                 // let (x, y) = stdout.cursor_pos().unwrap();
    //                 write!(
    //                     stdout,
    //                     "{}",
    //                     "*")

    //                 // write!(
    //                 //     stdout,
    //                 //     "{}{}Cursor is at:
    //                 //         ({},{}){}",
    //                 //     termion::cursor::Goto(5, 5),
    //                 //     termion::clear::UntilNewline,
    //                 //     5,
    //                 //     5,
    //                 //     termion::cursor::Goto(a, b)
    //                 // )
    //                 .unwrap();
    //                 stdout.flush().unwrap();
    //             },
    //         }},
    //         _ => {},
    //     }
    // }
    write!(
        stdout,
        "{}{}\n",
        // termion::cursor::Goto(1, 1),
        termion::clear::AfterCursor,
        termion::cursor::Show,
    )
    .unwrap();
    stdout.flush().unwrap();
    // end_term(stdout);
}
