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
        Green,
    },
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
    )
    .unwrap();
    stdout.flush().unwrap();
}

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

use core_dev::terminal::screen::Screen;

fn main() {
    let mut screen = Screen::new();
    screen.init_screen();
    let (mut stdin, mut stdout) = init_term();
    // let mut stdin = termion::async_stdin();


    let mut index = 0;
    let mut wrong_index = 0;
    let mut skip = false;

    let text = String::from(
        "hello from ansi term, its andrew from another world.",
    );
    // let text = String::from("hello");
    screen.println(&text, 2, 1);
    screen.println("^", 3, 1);
    screen.println("│", 4, 1);
    screen.println(format!("Index: {index}").as_str(), 6, 1);
    screen.println(format!("Wrong: {wrong_index}").as_str(), 7, 1);
    screen.println(
        format!("Index + Wrong: {}", index + wrong_index).as_str(),
        8,
        1,
    );
    screen.println(&get_text_colored(&text, index, wrong_index), 2, 1);
    if !skip {
        screen.println("^", 3, index + wrong_index + 1);
        screen.println("│", 4, index + wrong_index + 1);
    }
    screen.println(format!("text len: {}", text.len()).as_str(), 9, 1);
    screen.println(
        format!("text len-1: {}", text.len() - 1).as_str(),
        10,
        1,
    );
    screen.refresh();

    for event in stdin.events() {
        let event = event.unwrap();
        match event {
            Event::Key(Key::Ctrl('c')) => break,
            Event::Key(Key::Backspace) => {
                skip = false;
                if wrong_index > 0 {
                    wrong_index -= 1
                } else {
                    if index > 0 {
                        index -= 1;
                    }
                }
            },
            Event::Key(Key::Char(character)) => {
                skip = false;
                screen.println(character.to_string().as_str(), 10, 1);

                if index == text.len() - 1 {
                    index += 1;
                    break;
                }
                if character == text.get_char(index).unwrap()
                    && wrong_index == 0
                {
                    index += 1;
                } else {
                    if index + wrong_index < text.len() {
                        wrong_index += 1;
                    } else {
                        let y = 4;
                        let x = index + wrong_index + 3;
                        screen
                            .println("you cant go further, sorry", y, x)
                            .refresh();
                        skip = true;
                    }
                }
                screen.println(character.to_string().as_str(), 5, 1);
            },
            _ => {},
        }
        screen.println(format!("Index: {index}").as_str(), 6, 1);
        screen.println(format!("Wrong: {wrong_index}").as_str(), 7, 1);
        screen.println(
            format!("Index + Wrong: {}", index + wrong_index).as_str(),
            8,
            1,
        );
        screen.println(&get_text_colored(&text, index, wrong_index), 2, 1);
        if !skip {
            screen.println("^", 3, index + wrong_index + 1);
            screen.println("│", 4, index + wrong_index + 1);
        }
        screen.println(format!("text len: {}", text.len()).as_str(), 9, 1);
        screen.println(
            format!("text len-1: {}", text.len() - 1).as_str(),
            10,
            1,
        );
        screen.refresh()
    }
    screen
        .println(&get_text_colored(&text, index, wrong_index), 2, 1)
        .refresh();


    screen.end_screen();
}
