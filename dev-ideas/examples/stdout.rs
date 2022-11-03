#![allow(unused)]

use std::io::Write;
use std::io::{
    stdout,
    Stdout
};
use std::thread::sleep;
use std::time::Duration;

fn main() -> std::result::Result<(), std::io::Error> {
    let mut stdout = stdout();

    let some_string = "does it auto-flush? not anymore";
    stdout.write_all(some_string.as_bytes());
    stdout.write(some_string.as_bytes());
    write!(stdout, "{}", some_string)?;

    // print!("sleeping 5 seconds ...");
    sleep(Duration::from_secs(5));
    // stdout.write_fmt(format_args!("{}", "some text")).unwrap();
    // write!(&mut stdout, "{}", "some text").unwrap();
    stdout.flush()?;
    Ok(())
}
