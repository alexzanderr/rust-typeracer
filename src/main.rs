#![allow(
    unused,
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    semicolon_in_expressions_from_macros,
    redundant_semicolons
)]

use colored::*;
use dotenv::dotenv;
use tty_racer as typeracer;
use typeracer::*;

fn main() -> TyperacerResult<()> {
    // this exists so i can override the DEBUG_MODE from the command line with
    // > DEBUG_MODE=off cargo run
    // always its better to override from the command line instead of overiding from file config
    // command line its closer to you than a config file
    let is_debug_mode_defined = std::env::var("DEBUG_MODE");
    match is_debug_mode_defined {
        Ok(debug_mode) => {},
        Err(err) => {
            // load dot env vars from .env file inside the project
            dotenv().ok();
        }
    }
    for (key, value) in std::env::vars() {
        if key == "DEBUG_MODE" {
            println!("{}: {}", key, value);
        }
    }

    // let TMUX_ALIASES_FILE = env!("DEBUG_MODE");
    // println!("{}", TMUX_ALIASES_FILE);
    // std::thread::sleep(::std::time::Duration::from_secs(10));

    let mut term = TerminalScreen::builder()
        .alternate(true)
        .capture_mouse(false)
        .build()?;

    term.enter_raw_terminal()?;
    term.set_panic_hook();

    let mut player = Typeracer::from_term(&mut term);

    let game_result = player.mainloop();

    term.leave_raw_terminal()?;

    match game_result {
        Ok(_) => {
            println!("game ended successfully, no errors.");
        },
        Err(game_error) => {
            eprintln!("{}", game_error);
        }
    }

    Ok(())
}
