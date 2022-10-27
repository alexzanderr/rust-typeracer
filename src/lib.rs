#![feature(error_generic_member_access)]
#![feature(type_alias_impl_trait)]
#![feature(provide_any)]
#![allow(
    unused,
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    semicolon_in_expressions_from_macros,
    redundant_semicolons
)]

mod terminal_screen;
pub use terminal_screen::{
    TermLines,
    TerminalScreen,
    TerminalScreenResult
};

mod typeracer;
pub use typeracer::{
    AppState,
    Stats,
    Typeracer,
    TyperacerResult,
    TyperacerUI
};
mod statics;

mod music_player;
pub use music_player::{
    MusicPlayer,
    MusicPlayerErrors,
    MusicPlayerResult
};
