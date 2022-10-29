use std::thread::{
    self,
    sleep
};
use std::time::Duration;

#[derive(Debug)]
pub enum MusicState {
    Stopped,
    Paused,
    Playing
}

use crossbeam_channel::unbounded;

fn main() {
    // Create a channel of unbounded capacity.
    let (s, r) = unbounded();
    let handle = thread::spawn(move || {
        let x = 123;
        loop {
            let music_state = r.recv().unwrap();
            // match
            sleep(Duration::from_secs(1));
        }
    });

    loop {
        s.send(MusicState::Stopped).unwrap();

        sleep(Duration::from_secs(1));
    }

    // Send a message into the channel.

    // Receive the message from the channel.
    // assert_eq!(r.recv(), Ok("Hello, world!"));
}
