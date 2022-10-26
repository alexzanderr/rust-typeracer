#![allow(
    unused,
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    semicolon_in_expressions_from_macros,
    redundant_semicolons
)]

pub enum LoopActions {
    TimeToBreak,
    TimeToContinue,
    GameFinished,
    LoopDoesntQuit,
    PauseGame,
    ContinueGame,
    QuitGame
}

mod imports;
use imports::*;

mod handle_key;
pub use handle_key::*;
mod ui;
pub use ui::*;

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

    // let mut player = Typeracer::from_term(&mut term);

    // let game_result = player.mainloop();
    let app_state = AppState::init();
    loop {
        // render ui
        draw_ui(&mut term, &app_state);

        // handle keyboard input
        if event::poll(Duration::from_millis(100))? {
            let event = event::read()?;
            // handle key particurarly
            let loop_action =
                handle_event(&mut term, event, &app_state)?.1;
            match loop_action {
                LoopActions::TimeToBreak => break,
                LoopActions::TimeToContinue => continue,
                LoopActions::GameFinished => continue,
                LoopActions::LoopDoesntQuit => {
                    // do nothing, just continues (not continue from programming)
                },
                LoopActions::QuitGame => break,
                _ => {
                    // the rest are not implemented
                }
            }
        }
    }

    term.leave_raw_terminal()?;

    Ok(())
}
