//! Demonstrates how to match on modifiers like: Control, alt, shift.
//!
//! cargo run --example event-poll-read

use rand::Rng;
use termion::color::Color;
use termion::event::Key;
use termion::input::TermRead;
use termion::input::MouseTerminal;

use termion::cursor::{self, DetectCursorPos};

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use std::io::Stdout;
use std::io::{stdin, stdout, Stdin, Write};
use std::time::Duration;

use crossterm::{
    cursor::position,
    event::{
        poll, read, DisableMouseCapture, EnableMouseCapture, Event,
        KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use rand::thread_rng;

fn print_events(stdout: &mut Stdout) {
    loop {
        let random_number = thread_rng().gen_range(10..10000);
        write!(
            stdout,
            "{}{}{}{}",
            // Clear the screen.
            termion::clear::CurrentLine,
            // Goto (1,1).
            termion::cursor::Goto(1, 5),
            // Hide the cursor.
            termion::cursor::Hide,
            format!("{}-----{}", random_number, random_number)
        )
        .unwrap();
        // Wait up to 1s for another event
        if poll(Duration::from_millis(100)).unwrap() {
            // It's guaranteed that read() wont block if `poll` returns `Ok(true)`
            let event = read().unwrap();

            // asta se ruleaza decat daca read primeste ceva ca input
            // std::thread::sleep(std::time::Duration::from_secs(1));
            // println!("Event::{:?}\r", event);
            // Event::Key(KeyEvent { code: Char('c'), modifiers: CONTROL })
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers,
                }) => {
                    println!("da");
                    break;
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Char(character),
                    modifiers,
                }) => {
                    write!(
                        stdout,
                        "{}{}",
                        // Clear the screen.
                        // termion::clear::CurrentLine,
                        // Goto (1,1).
                        termion::cursor::Goto(1, 10),
                        // Hide the cursor.
                        termion::cursor::Hide,
                        // character
                        // format!("char is: {}", character)
                    )
                    .unwrap();
                    // println!("{}", character);
                    // println!("{:?}", modifiers);
                    // if modifiers == KeyModifiers::CONTROL {
                    //     println!("da");
                    // }
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers,
                }) => break,
                _ => {},
            }

            // if event == Event::Key(KeyCode::Char('c').into()) {
            //     println!("Cursor position: {:?}\r", position());
            // }

            // if event == Event::Key(KeyCode::Esc.into()) {
            //     break;
            // }
        } else {
            // Timeout expired, no event for 1s
            println!(".\r");
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();

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

    execute!(stdout, EnableMouseCapture)?;

    print_events(&mut stdout);

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()
}
