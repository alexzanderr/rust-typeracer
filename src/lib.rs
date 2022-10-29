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
// warning: allow(clippy::all) incompatible with previous forbid
// fix: use `deny` instead of `forbid`
#![deny(clippy::all)]
#![allow(
    clippy::module_inception,
    // because i like to see the `&'static str` there
    clippy::redundant_static_lifetimes,
    // this is stupid, when i have `&mut String` inside function
    // its tells me to change to `&mut str`, then how am i supposed to `String::push()`
    // on a `&mut str`, cuz no method named `str::push()` for type `str`
    clippy::ptr_arg,
    clippy::let_and_return
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
    TyperacerErrors,
    TyperacerUI
};
mod statics;

mod music_player;
pub use music_player::{
    MusicPlayer,
    MusicPlayerErrors,
    MusicPlayerResult,
    MusicState
};
