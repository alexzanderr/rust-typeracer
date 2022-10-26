pub use std::time::Duration;
pub use std::io::Stdout;
pub use std::io::{
    stdin,
    stdout,
    Stdin,
    Write
};

pub use crossterm::event::{
    Event::{
        self,
        *
    },
    KeyCode,
    KeyEvent
};
pub use crossterm::{
    cursor,
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        *
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
pub use colored::*;
pub use dotenv::dotenv;
pub use typeracer::*;
