#![allow(
    dead_code,
    unused_labels,
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

use std::time::{
    Duration,
    Instant
};
use std::thread::{
    sleep,
    spawn,
    Builder as ThreadBuilder
};
use std::sync::{
    Arc,
    Mutex
};
use std::io::{
    stdout,
    Stdout,
    Write
};

pub enum GameState {
    Pause,
    Continue
}

fn main() {
    let elapsed_arc = Arc::new(Mutex::new(0usize));
    let elapsed_arc_clone = elapsed_arc.clone();

    let stopwatch_thread = ThreadBuilder::new()
        .name("stopwatch-thread".to_string())
        .spawn(move || {
            let elapsed_arc = elapsed_arc_clone;

            loop {
                if let Ok(mut elapsed_mutex) = elapsed_arc.lock() {
                    *elapsed_mutex += 1;
                } else {
                    eprintln!("wrong")
                }

                sleep(Duration::from_millis(1000));
            }
        })
        .unwrap();

    let mut stdout = stdout();
    'mainloop: loop {
        {
            let elapsed_mutex = elapsed_arc.lock().unwrap();
            println!("elapsed from main: {}", *elapsed_mutex);
        }

        sleep(Duration::from_millis(500));
    }
}
