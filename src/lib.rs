// documentation right now its the readme
#![doc = include_str ! ("../README.md")]
#![forbid(unsafe_code)]
// features
#![feature(error_generic_member_access)]
#![feature(type_alias_impl_trait)]
#![feature(provide_any)]
// allows
#![allow(
// for this kind
// help: use an automatic link instead: `<https://play.typeracer.com/>`
rustdoc::bare_urls,
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
// allows: clippy
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
pub use self::typeracer::{
    AppState,
    GameState,
    Stats,
    Typeracer,
    TyperacerErrors,
    TyperacerResult,
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

mod utils;
mod config;

pub use config::{
    ConfigErrors,
    ConfigResult,
    TyperacerConfig
};
