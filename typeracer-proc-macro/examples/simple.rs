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

use typeracer_proc_macro::stopwatch;

#[stopwatch]
fn its_time() -> i128 {
    let mut sum = 0i128;
    for index in 0..10000000 {
        sum += index;
    }
    sum
}

struct Timer;

impl Timer {
    #[stopwatch]
    fn from_struct() -> i128 {
        let mut sum = 0i128;
        for index in 0..10000000 {
            sum += index;
        }
        sum
    }
}

fn main() {
    its_time();
    Timer::from_struct();
}
