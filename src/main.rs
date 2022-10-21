#![feature(type_alias_impl_trait)]
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
    Event,
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
use core_dev::traits::string_extended::StringExtended;
use rand::thread_rng;
use rand::Rng;

mod terminal_screen;
pub use terminal_screen::{
    TermLines,
    TerminalScreen,
    TerminalScreenResult
};

mod typeracer;
pub use typeracer::Typeracer;
mod errors;
pub use errors::TyperacerResult;

fn main() -> TyperacerResult<()> {
    let mut term = TerminalScreen::builder()
        .alternate(true)
        .capture_mouse(false)
        .build()?;

    term.enter_raw_terminal()?;

    // term.print("hello world", 0, 0)?.refresh()?;
    let multi_line_string = r#"

string = "andrew is here and he wants to tell you something"
--->--->--->x = 3↵
--->--->y = 2↵
--->print()↵
from core.json__ import *↵
--->print("hello world")↵
↵
↵
for i in range(100):↵
--->x = 3↵
--->y = 2↵
↵
print("something22222222")↵

"#;
    // term.draw_rectangle(&multi_line_string, 5, 0)?.refresh()?;

    // term.rectangle()
    //     .text(multi_line_string)
    //     .xy(5, 0)
    //     .screens_width(true)
    //     .align_center(false)
    //     .build()?
    //     .draw()?;

    // term.refresh()?;

    let player = Typeracer::from_term(&mut term);
    player.mainloop();

    term.leave_raw_terminal()?;

    Ok(())
}
