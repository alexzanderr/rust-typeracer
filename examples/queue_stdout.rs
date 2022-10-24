#![allow(unused)]

use std::io::Write;
use std::io::{
    stdout,
    Stdout
};
use std::thread::sleep;
use std::time::Duration;

use crossterm::{
    execute,
    queue
};
use crossterm::style::Print;

fn main() -> std::result::Result<(), std::io::Error> {
    let mut stdout = stdout();

    let some_string = "why does it flush the stdout?????";
    queue!(stdout, Print(some_string));
    queue!(stdout, Print(some_string));
    queue!(stdout, Print(some_string));
    queue!(stdout, Print(some_string));
    queue!(stdout, Print(some_string));
    queue!(stdout, Print(some_string));

    // print!("\nsleeping 5 seconds ...\n");
    sleep(Duration::from_secs(5));

    stdout.flush()?;
    // stdout.write_fmt(format_args!("{}", "some text")).unwrap();
    // write!(&mut stdout, "{}", "some text").unwrap();
    Ok(())
}
