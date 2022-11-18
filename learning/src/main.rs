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

use std::thread::sleep;
use std::time::Duration;

use log::*;
use logging_timer::{
    stime,
    time,
    *
};

#[stime]
fn find_files() {
    sleep(Duration::from_secs(2));
    trace!("asd");
    let tmr1 = timer!(Level::Warn; "TIMER_AT_WARN");

    // expensive operation here
} // 'TimerFinished' message is logged here

fn main() {
    let tmr1 = timer!(Level::Warn; "TIMER_AT_WARN");
    find_files();
    trace!("asd");
    debug!("asdasd");
}
