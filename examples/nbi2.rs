extern crate epoll;
extern crate libc;
extern crate termion;

use std::io;
use std::io::{
    Read,
    Write,
};
use std::os::unix::io::AsRawFd;
use termion::raw::IntoRawMode;

/// Waits for a key press timeout milisseconds
/// 0 - no wait
/// <0 - wait until a key is pressed
/// >0 wait for an event or timeout millisecods for an event.
fn inkey(timeout: i32) -> bool {
    let input = io::stdin();
    let _output = io::stdout().into_raw_mode().unwrap();
    let fd = input.as_raw_fd();
    let pool = epoll::create(true).unwrap();
    let e = epoll::Event {
        events: epoll::Events::EPOLLIN.bits(),
        data:   0,
    };
    epoll::ctl(pool, epoll::ControlOptions::EPOLL_CTL_ADD, fd, e);
    let mut v = [e];
    let wresult = epoll::wait(pool, timeout, &mut v).unwrap();
    unsafe {
        libc::close(pool);
    }
    return wresult == 1;
}

fn main() {
    let mut buffer: [u8; 1] = [0; 1];
    let mut x = 0;
    loop {
        if inkey(10) {
            let result = io::stdin().read(&mut buffer);
            println!("{:?} {:?}", result, buffer);
            if buffer[0] == 3 {
                break;
            }
        }
        if x % 1000 == 0 {
            print!(".");
        }
        io::stdout().flush().unwrap();
        x += 1;
    }
}
