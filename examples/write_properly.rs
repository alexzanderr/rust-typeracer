#![allow(unused)]

use std::io::stdout;
use std::io::Error;

use ::std::io::Write;

fn main() -> std::result::Result<(), Error> {
    let mut stdout = stdout();
    let mut buffer = Vec::<u8>::new();
    let mut index = 0usize;
    loop {
        let some_string = format!("line {index}\n");

        write!(buffer, "{}", some_string)?;
        buffer.write(some_string.as_bytes())?;
        buffer.write_all(some_string.as_bytes())?;

        // error: buffer doesnt implement Display
        // write!(stdout, "{}", buffer)?;

        index += 1;

        stdout.write_all(buffer.as_slice())?;

        buffer.clear();

        std::thread::sleep(::std::time::Duration::from_secs(1));
    }
    Ok(())
}
