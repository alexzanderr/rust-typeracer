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
    Read,
};

use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{
    thread,
    time,
};

fn main() {
    let stdin_channel = spawn_stdin_channel();
    loop {
        match stdin_channel.try_recv() {
            Ok(key) => {
                println!("Received: {}", key)
            },
            Err(TryRecvError::Empty) => {
                println!("Channel empty")
            },
            Err(TryRecvError::Disconnected) => {
                panic!("Channel disconnected")
            },
        }
        sleep(1000);
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    let stdin = stdin();
    thread::spawn(move || {
        for event in stdin.events() {
            println!("im inside loop");
            let key = event.unwrap();
            match key {
                Event::Key(Key::Ctrl('q')) => break,
                Event::Key(Key::Char(character)) => {
                    tx.send(character.to_string()).unwrap()
                },
                _ => {}
            }
        }
        // let mut buffer = vec![];
        // io::stdin().read(&mut buffer).unwrap();
        // tx.send(String::from_utf8(buffer).unwrap()).unwrap();
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
