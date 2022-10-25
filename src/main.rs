#![feature(type_alias_impl_trait)]
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

mod terminal_screen;
pub use terminal_screen::{
    TermLines,
    TerminalScreen,
    TerminalScreenResult
};

mod typeracer;
pub use typeracer::{
    Typeracer,
    TyperacerResult,
    TyperacerUI
};
mod statics;

fn main() -> TyperacerResult<()> {
    let mut term = TerminalScreen::builder()
        .alternate(true)
        .capture_mouse(false)
        .build()?;

    term.enter_raw_terminal()?;
    term.set_panic_hook();

    let player = Typeracer::from_term(&mut term);

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
