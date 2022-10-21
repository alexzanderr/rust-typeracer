use std::time::Duration;

use crossterm::{
    event::{
        read,
        poll,
    },
    Result,
};

fn print_events() -> Result<bool> {
    loop {
        if poll(Duration::from_millis(100))? {
            // It's guaranteed that `read` wont block, because `poll` returned
            // `Ok(true)`.
            println!("{:?}", read()?);
        } else {
            // Timeout expired, no `Event` is available
            println!(".")
        }
    }
}

fn main() {
    print_events();
}
